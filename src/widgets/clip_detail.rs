use glib::Object;
use gtk::glib;

use crate::clip_info_boxed::ClipInfoObject;

glib::wrapper! {
    pub struct ClipDetail(ObjectSubclass<imp::ClipDetail>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ClipDetail {
    pub fn new(clip_info: &ClipInfoObject) -> Self {
        Object::builder().property("clip_info", clip_info).build()
    }
}

mod imp {
    use crate::clip_info_boxed::ClipInfoObject;
    use crate::widgets::fit_to_width_widget::FitToWidthWidget;
    use adw::prelude::*;
    use exporter_core::clip_info::ClipResolution;
    use gio::{Cancellable, File};
    use glib::subclass::InitializingObject;
    use glib::{DateTime, GString, Properties, gformat};
    use gtk::gio::ListStore;
    use gtk::glib::clone;
    use gtk::subclass::prelude::*;
    use gtk::{
        CompositeTemplate, FileDialog, FileFilter, FileLauncher, Picture, TemplateChild, Widget,
        Window,
    };
    use gtk::{gio, glib};
    use std::cell::{OnceCell, RefCell};
    use std::path::PathBuf;

    #[derive(CompositeTemplate, Properties, Default)]
    #[properties(wrapper_type = super::ClipDetail)]
    #[template(resource = "/io/github/foriequal0/steam-clip-exporter/clip_detail.ui")]
    pub struct ClipDetail {
        #[template_child]
        group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        thumbnail_bin: TemplateChild<adw::Bin>,
        #[template_child]
        appid_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        length_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        resolution_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        path_row: TemplateChild<adw::ActionRow>,

        #[template_child]
        toast_overlay: TemplateChild<adw::ToastOverlay>,

        #[property(set = ClipDetail::set_clip_info, get, construct_only)]
        clip_info: OnceCell<ClipInfoObject>,

        path: RefCell<PathBuf>,
    }

    #[gtk::template_callbacks]
    impl ClipDetail {
        #[template_callback]
        fn handle_export(&self) {
            glib::spawn_future_local(clone!(
                #[strong(rename_to = obj)]
                self.obj(),
                async move { obj.imp().handle_export_async().await }
            ));
        }

        #[template_callback]
        fn handle_open_path(&self) {
            let file = File::for_path(self.path.borrow().as_path());
            FileLauncher::new(Some(&file)).launch(Window::NONE, Cancellable::NONE, |_| {});
        }
    }

    impl ClipDetail {
        fn set_clip_info(&self, clip_info: &ClipInfoObject) {
            _ = self.clip_info.set(clip_info.clone());

            let clip_info = clip_info.value();

            let bin = FitToWidthWidget::new();
            Picture::for_filename(clip_info.clip_path.thumbnail_jpg().as_path()).set_parent(&bin);

            self.thumbnail_bin.set_child(Some(&bin));
            self.group.set_title(&clip_info.title);
            self.group
                .set_description(Some(&format_timestamp(clip_info.timestamp)));
            self.appid_row.set_subtitle(&clip_info.appid);
            self.length_row
                .set_subtitle(&format_length_ms(clip_info.length_ms));
            self.resolution_row
                .set_subtitle(&format_resolution(&clip_info.resolution));
            let path = clip_info.clip_path.root();
            self.path_row.set_subtitle(&path.to_string_lossy());
            self.path.replace(path.to_owned());
        }

        async fn handle_export_async(&self) {
            let nearest_window = nearest_window_to(self.obj().upcast_ref());
            let clip_info = self.obj().clip_info().value();
            let title = clip_info.title.clone();
            let mpd = clip_info.session_mpd.clone();

            enum ExportError {
                Cancelled,
                Error(String),
            }

            let filters = {
                let filter = FileFilter::new();
                filter.add_mime_type("video/mp4");
                let filters = ListStore::new::<FileFilter>();
                filters.append(&filter);
                filters
            };

            let result = self
                .with_lock_ui(clone!(
                    #[strong]
                    nearest_window,
                    async move || {
                        let dialog = {
                            let mut builder = FileDialog::builder()
                                .title("Export as")
                                .initial_name(format!("{title}.mp4"))
                                .filters(&filters)
                                .modal(true);
                            if let Some(video) = get_video_dir() {
                                builder = builder.initial_folder(&video);
                            }
                            builder.build()
                        };

                        let Ok(file) = dialog.save_future(nearest_window.as_ref()).await else {
                            return Err(ExportError::Cancelled);
                        };

                        let Some(path) = file.path() else {
                            return Err(ExportError::Error("Invalid path".to_string()));
                        };

                        let result = gio::spawn_blocking(move || {
                            exporter_core::exporter::ffmpeg(&mpd, &path)
                        })
                        .await;

                        match result {
                            Ok(Ok(_)) => Ok(file),
                            Ok(Err(err)) => Err(ExportError::Error(err.to_string())),
                            Err(_) => Err(ExportError::Error("Unknown error: ".to_string())),
                        }
                    }
                ))
                .await;
            match result {
                Ok(exported) => {
                    let toast = adw::Toast::builder()
                        .title("Clip exported")
                        .button_label("Open directory")
                        .build();
                    toast.connect_button_clicked({
                        move |_| {
                            FileLauncher::new(Some(&exported.parent().unwrap())).launch(
                                Window::NONE,
                                Cancellable::NONE,
                                |_| {},
                            );
                        }
                    });
                    self.toast_overlay.add_toast(toast);
                }
                Err(ExportError::Cancelled) => {}
                Err(ExportError::Error(err)) => {
                    self.show_error_toast(&format!("Failed to export clip: {}", err.to_string()))
                }
            }
        }

        fn show_error_toast(&self, message: &str) {
            let toast = adw::Toast::builder()
                .title(format!("Error: {}", message))
                .build();
            self.toast_overlay.add_toast(toast);
        }

        async fn with_lock_ui<F, Fut>(&self, f: F) -> Fut::Output
        where
            F: FnOnce() -> Fut,
            Fut: Future,
        {
            let obj = self.obj();
            _ = obj.activate_action("win.lock-ui", Some(&true.to_variant()));

            let result = f().await;

            _ = obj.activate_action("win.lock-ui", Some(&true.to_variant()));
            result
        }
    }

    fn format_timestamp(timestamp: i32) -> GString {
        let now = DateTime::now_local().expect("Failed to get current local time");

        let datetime = DateTime::from_unix_utc(timestamp as i64)
            .expect("Failed to convert timestamp to DateTime");

        let culture = datetime
            .to_local()
            .expect("Failed to convert timestamp to local time")
            .format("%c")
            .expect("Failed to format timestamp");

        let relative = {
            let now = now.difference(&datetime);
            if now.as_hours() < 1 {
                format!("{} minutes ago", now.as_minutes())
            } else if now.as_days() < 1 {
                format!("{} hours ago", now.as_hours())
            } else if now.as_days() < 7 {
                format!("{} days ago", now.as_days())
            } else {
                format!("{} weeks ago", now.as_days() / 7)
            }
        };

        gformat!("{} ({})", culture, relative)
    }

    fn format_length_ms(length_ms: Option<i32>) -> String {
        let Some(length_ms) = length_ms else {
            return String::from("Unknown");
        };

        let hours = length_ms / (60 * 60 * 1000);
        let minutes = (length_ms % (60 * 60 * 1000)) / 60000;
        let seconds = (length_ms % (60 * 1000)) / 1000;

        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }

    fn format_resolution(p0: &ClipResolution) -> String {
        format!("{}px × {}px", p0.width, p0.height)
    }

    fn nearest_window_to(widget: &Widget) -> Option<Window> {
        let mut current: Widget = widget.clone();
        while let Some(parent) = current.parent() {
            if let Some(window) = parent.downcast_ref::<Window>() {
                return Some(window.clone());
            }

            current = parent;
        }

        None
    }

    fn get_video_dir() -> Option<File> {
        let home = std::env::var("HOME").ok()?;

        let mut path_buf = PathBuf::from(home);
        path_buf.push("Videos");

        Some(File::for_path(&path_buf))
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ClipDetail {
        const NAME: &'static str = "SceClipDetail";
        type Type = super::ClipDetail;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for ClipDetail {}

    impl WidgetImpl for ClipDetail {}

    impl BoxImpl for ClipDetail {}
}
