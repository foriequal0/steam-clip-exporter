use glib::Object;
use gtk::Application;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct ApplicationWindow(ObjectSubclass<imp::ApplicationWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ApplicationWindow {
    pub fn new(app: impl AsRef<Application>) -> Self {
        let obj: ApplicationWindow = Object::builder()
            .property("application", app.as_ref())
            .build();
        obj
    }
}

mod imp {
    use adw::ActionRow;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use exporter_core::{ClipInfo, SteamRoot};
    use gio::ListStore;
    use glib::subclass::InitializingObject;
    use glib::{DateTime, GString, Object, clone};
    use gtk::gio::ActionEntry;
    use gtk::{CompositeTemplate, Image};
    use gtk::{gio, glib};

    use crate::clip_info_boxed::ClipInfoObject;
    use crate::widgets::clip_detail::ClipDetail;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/io/github/foriequal0/steam-clip-exporter/window.ui")]
    pub struct ApplicationWindow {
        #[template_child]
        split_view: TemplateChild<adw::NavigationSplitView>,
        #[template_child]
        clip_list_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        content_view: TemplateChild<adw::Bin>,
    }

    impl ApplicationWindow {
        pub fn register_window_actions(&self) {
            let action = ActionEntry::builder("lock-ui")
                .parameter_type(Some(&bool::static_variant_type()))
                .activate(move |window: &super::ApplicationWindow, _, parameter| {
                    let parameter = parameter
                        .and_then(|x| x.get::<bool>())
                        .expect("bool type parameter should be given");
                    window.set_focusable(parameter);
                })
                .build();
            self.obj().add_action_entries([action]);
        }

        pub fn reload(&self) {
            let steam_root = SteamRoot::new();

            let mut vec = Vec::new();
            for path in steam_root.clip_paths().expect("Failed to get clip paths") {
                let clip_info =
                    ClipInfo::load(&steam_root, &path).expect("Failed to load clip info");
                let clip_info_boxed = ClipInfoObject::new(clip_info);
                vec.push(clip_info_boxed);
            }

            // Sort timestamp decreasing order
            vec.sort_by(|x, y| x.value().timestamp.cmp(&y.value().timestamp).reverse());

            let list_store = {
                let list_store = ListStore::builder()
                    .item_type(ClipInfoObject::static_type())
                    .build();
                list_store.extend_from_slice(&vec);
                list_store
            };

            self.clip_list_group.bind_model(
                Some(&list_store),
                clone!(
                    #[strong(rename_to=obj)]
                    self.obj(),
                    move |list_item: &Object| {
                        let clip_info: &ClipInfoObject =
                            list_item.downcast_ref().expect("ClipInfoObject");
                        clip_info_to_action_row(&obj, clip_info).upcast()
                    },
                ),
            );

            if let Some(first) = vec.first() {
                self.set_clip_info(first);
            }
        }

        fn set_clip_info(&self, clip_info: &ClipInfoObject) {
            self.content_view
                .set_child(Some(&ClipDetail::new(clip_info)));
        }
    }

    fn clip_info_to_action_row(
        app: &super::ApplicationWindow,
        clip_info: &ClipInfoObject,
    ) -> ActionRow {
        let value = clip_info.value();
        let row = ActionRow::new();

        row.set_title(&value.title);
        row.set_subtitle(&format_timestamp(value.timestamp));
        row.add_prefix(&{
            let image = Image::from_file(value.clip_path.thumbnail_jpg().as_path());
            image.set_pixel_size(64);
            image
        });
        row.set_activatable(true);
        row.connect_activated(clone!(
            #[strong]
            app,
            #[strong]
            clip_info,
            move |_| {
                let imp = app.imp();
                imp.set_clip_info(&clip_info);
                imp.split_view.set_show_content(true);
            }
        ));

        row
    }

    fn format_timestamp(timestamp: i32) -> GString {
        DateTime::from_unix_utc(timestamp as i64)
            .expect("Failed to convert timestamp to DateTime")
            .to_local()
            .expect("Failed to convert timestamp to local time")
            .format("%c")
            .expect("Failed to format timestamp")
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationWindow {
        const NAME: &'static str = "SceApplicationWindow";
        type Type = super::ApplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ApplicationWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.register_window_actions();
            self.reload();
        }
    }

    impl WidgetImpl for ApplicationWindow {}

    impl WindowImpl for ApplicationWindow {}

    impl ApplicationWindowImpl for ApplicationWindow {}

    impl AdwApplicationWindowImpl for ApplicationWindow {}
}
