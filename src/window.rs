use std::io::Read;

use gtk::gio::ListStore;
use gtk::glib;
use gtk::gio;
use gtk::glib::{ GString};
use gtk::glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassExt;
use libadwaita as adw;

use crate::DesktopFileModel;
use crate::dialogs;
use crate::dialogs::Reader;

mod imp {
    use gtk::glib::once_cell::sync::Lazy;
    use gtk::glib::subclass::{InitializingObject, Signal};
    use gtk::{glib, CompositeTemplate};
    use gtk::subclass::prelude::*;
    use libadwaita::subclass::prelude::*;
    use libadwaita as adw;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/dev/diegovsky/DesktopFileCreator/window.ui")]
    pub struct Window {
        #[template_child]
        pub name: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub exec: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub categories: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub icon: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub open_icon_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub save_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub open_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    }

    impl Window {
        pub fn send_notification(&self, text: &str) {
            self.toast_overlay.add_toast(adw::Toast::new(text))
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "DFCWindow";
        type Type = super::Window;
        type ParentType = libadwaita::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }

    }

    impl ObjectImpl for Window {}
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[derive(Debug)]
pub enum SaveMethod {
    SaveAs,
    Save,
}


impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder()
            .property("application", app)
            .build()
    }
    fn imp(&self) -> &imp::Window {
        imp::Window::from_obj(self)
    }
    pub fn connect_file_save<F: Fn(&Self, SaveMethod) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        let this = self.clone();
        self.imp().save_button.connect_clicked(move |_| f(&this, SaveMethod::Save))
    }
    pub fn connect_file_open<F: Fn(&Self, Reader) + 'static + Copy>(&self, f: F) -> glib::SignalHandlerId {
        self.imp().open_button.connect_clicked({
            let this = self.clone();
            move |_| {
                let f2 = {
                    let this = this.clone();
                    move |file| f(&this, file)
                };
                dialogs::open_file(&this.clone(), dialogs::filters(["filter_desktop", "filter_any"]), f2)
            }
        })
    }
    pub fn get_model(&self) -> DesktopFileModel {
        let obj = self.imp();
        let categories = obj.categories.text().into();//.split(",").map(|s| s.into()).collect();
        DesktopFileModel { name: obj.name.text().into(), exec: obj.exec.text().into(), categories, icon: obj.icon.text().into() }

    }
    pub fn set_model(&self, model: DesktopFileModel) {
        let obj = self.imp();
        let DesktopFileModel { name, exec, categories, icon } = model;
        obj.name.set_text(&name);
        obj.exec.set_text(&exec);
        obj.categories.set_text(&categories);
        obj.icon.set_text(&icon);
    }
    pub fn send_notification(&self, text: &str) {
        self.imp().send_notification(text);
    }
    pub fn handle_error<F, E: std::fmt::Display, T>(&self, f: F) -> Option<T> where F: FnOnce() -> Result<T, E> {
        let res = f();
        match res {
            Ok(t) => Some(t),
            Err(e) => {
                self.send_notification(&format!("Error: {}", e));
                None
            }
        }
    }
}
