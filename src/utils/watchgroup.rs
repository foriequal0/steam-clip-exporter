use eyre::Result;
use gtk::gio::{File, FileMonitor, FileMonitorFlags};
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone)]
pub struct WatchGroup {
    inner: Rc<WatchGroupImpl>,
}

impl WatchGroup {
    pub fn new(callback: impl Fn() + 'static) -> Self {
        Self {
            inner: Rc::new(WatchGroupImpl::new(callback)),
        }
    }

    pub fn add(&self, path: &Path) {
        _ = self.inner.add(path);
    }

    pub fn extend<'a>(&'a self, paths: impl IntoIterator<Item = impl AsRef<Path> + 'a>) {
        for path in paths {
            _ = self.inner.add(path.as_ref());
        }
    }
}

struct WatchGroupImpl {
    callback: Rc<Box<dyn Fn()>>,
    cancellable: gtk::gio::Cancellable,
    monitors: RefCell<Vec<FileMonitor>>,
}

impl WatchGroupImpl {
    fn new(callback: impl Fn() + 'static) -> Self {
        Self {
            callback: Rc::new(Box::new(callback)),
            cancellable: gtk::gio::Cancellable::new(),
            monitors: RefCell::new(Vec::new()),
        }
    }

    fn add(&self, path: &Path) -> Result<(), glib::Error> {
        let file = File::for_path(path);
        let monitor = file.monitor(FileMonitorFlags::NONE, Some(&self.cancellable))?;
        monitor.set_rate_limit(1000);
        monitor.connect_changed(clone!(
            #[strong(rename_to=callback)]
            self.callback,
            move |_monitor, _oldfile, _newfile, _event| {
                callback();
            }
        ));
        self.monitors.borrow_mut().push(monitor);
        Ok(())
    }
}

impl Drop for WatchGroupImpl {
    fn drop(&mut self) {
        self.cancellable.cancel();
    }
}
