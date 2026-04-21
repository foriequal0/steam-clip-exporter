use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct FitToWidthWidget(ObjectSubclass<imp::FitToWidthWidget>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl FitToWidthWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use adw::subclass::prelude::*;
    use gtk::prelude::*;
    use gtk::{SizeRequestMode, glib};

    #[derive(Default)]
    pub struct FitToWidthWidget;

    #[glib::object_subclass]
    impl ObjectSubclass for FitToWidthWidget {
        const NAME: &'static str = "SceFitToWidthWidget";
        type Type = super::FitToWidthWidget;
        type ParentType = gtk::Widget;
    }

    impl WidgetImpl for FitToWidthWidget {
        fn request_mode(&self) -> SizeRequestMode {
            SizeRequestMode::HeightForWidth
        }

        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let child = self.obj().first_child().unwrap();
            let (min, nat, min_baseline, nat_baseline) = child.measure(orientation, for_size);
            if orientation == gtk::Orientation::Vertical {
                return (nat, nat, min_baseline, nat_baseline);
            }

            (min, nat, min_baseline, nat_baseline)
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let child = self.obj().first_child().unwrap();
            child.allocate(width, height, baseline, None);
        }
    }

    impl ObjectImpl for FitToWidthWidget {
        fn dispose(&self) {
            let child = self.obj().first_child().unwrap();
            child.unparent();
        }
    }
}
