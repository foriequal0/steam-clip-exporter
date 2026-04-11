use std::ops::Deref;
use std::rc::Rc;

use exporter_core::clip_info::ClipInfo;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct ClipInfoObject(ObjectSubclass<imp::ClipInfoObject>);
}

impl ClipInfoObject {
    pub fn new(clip_info: ClipInfo) -> Self {
        let clip_info = ClipInfoValue(Rc::new(clip_info));
        Object::builder().property("value", clip_info).build()
    }
}

#[derive(Clone, Debug, glib::Boxed)]
#[boxed_type(name = "ClipInfoBoxed")]
pub struct ClipInfoValue(Rc<ClipInfo>);

impl Deref for ClipInfoValue {
    type Target = ClipInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

mod imp {
    use super::*;

    use glib::Properties;
    use glib::prelude::*;
    use gtk::subclass::prelude::*;
    use std::cell::OnceCell;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ClipInfoObject)]
    pub struct ClipInfoObject {
        #[property(get, construct_only)]
        pub value: OnceCell<ClipInfoValue>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ClipInfoObject {
        const NAME: &'static str = "ClipInfoObject";
        type Type = super::ClipInfoObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ClipInfoObject {}
}
