use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

mod imp {

    use std::cell::RefCell;

    use super::*;
    use glib::subclass::InitializingObject;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/moe/tsukimi/check_row.ui")]
    pub struct CheckRow {
        #[template_child]
        pub check: TemplateChild<gtk::CheckButton>,
        pub track_id: RefCell<i64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CheckRow {
        const NAME: &'static str = "CheckRow";
        type Type = super::CheckRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CheckRow {}

    impl WidgetImpl for CheckRow {}
    impl ListBoxRowImpl for CheckRow {}
    impl PreferencesRowImpl for CheckRow {}
    impl ActionRowImpl for CheckRow {}
}

glib::wrapper! {
    pub struct CheckRow(ObjectSubclass<imp::CheckRow>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::ActionRow, adw::PreferencesRow, @implements gtk::Accessible;
}

impl CheckRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}