use adw::glib::spawn_future_local;
use debounced::debounced;
use futures::channel::mpsc::UnboundedSender;
use futures_util::{SinkExt, StreamExt};
use gtk::gio::{File, FileMonitor, FileMonitorEvent, FileMonitorFlags};
use gtk::glib::clone;
use gtk::prelude::*;
use std::cell::RefCell;
use std::path::Path;
use std::time::Duration;

pub struct WatchGroup {
    tx: UnboundedSender<()>,
    cancellable: gtk::gio::Cancellable,
    monitors: RefCell<Vec<FileMonitor>>,
}

impl WatchGroup {
    pub fn new(callback: impl Fn() + 'static) -> Self {
        let (tx, rx) = futures::channel::mpsc::unbounded();
        spawn_future_local(async move {
            let mut debounced = debounced(rx, Duration::from_secs(1));
            while debounced.next().await.is_some() {
                callback();
            }
        });

        Self {
            tx,
            cancellable: gtk::gio::Cancellable::new(),
            monitors: RefCell::new(Vec::new()),
        }
    }

    pub fn extend<'a>(&'a self, paths: impl IntoIterator<Item = impl AsRef<Path> + 'a>) {
        for path in paths {
            self.add(path.as_ref());
        }
    }

    pub fn add(&self, path: &Path) {
        let file = File::for_path(path);
        let Ok(monitor) = file.monitor(FileMonitorFlags::NONE, Some(&self.cancellable)) else {
            return;
        };

        monitor.connect_changed(clone!(
            #[strong(rename_to=tx)]
            self.tx,
            move |_monitor, _old_file, _new_file, event| {
                // it's rather spurious event for us
                if let FileMonitorEvent::ChangesDoneHint = event {
                    return;
                };

                spawn_future_local({
                    let mut tx = tx.clone();
                    async move {
                        _ = tx.send(()).await;
                    }
                });
            }
        ));

        self.monitors.borrow_mut().push(monitor);
    }
}

impl Drop for WatchGroup {
    fn drop(&mut self) {
        self.cancellable.cancel();
        self.monitors.borrow_mut().clear();
    }
}
