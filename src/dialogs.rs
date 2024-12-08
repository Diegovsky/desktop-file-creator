use std::io::Read;

use gtk::prelude::*;

use crate::window::Window;

pub fn get_filter_by_id(id: impl gtk::glib::IntoGStr) -> gtk::FileFilter {
    let builder = gtk::Builder::from_resource("/dev/diegovsky/DesktopFileCreator/filters.ui");
    builder.object::<gtk::FileFilter>(id).unwrap()
}

pub fn filters(
    ids: impl IntoIterator<Item = impl gtk::glib::IntoGStr>,
) -> impl Iterator<Item = gtk::FileFilter> {
    ids.into_iter().map(get_filter_by_id)
}

pub type Reader = Box<dyn Read>;

pub fn open_file(
    window: &Window,
    filters: impl IntoIterator<Item = gtk::FileFilter>,
    f: impl FnOnce(Reader) + 'static,
) {
    let filters = gtk::gio::ListStore::from_iter(filters);
    let dialog = gtk::FileDialog::builder().filters(&filters).build();
    let this = window.clone();
    dialog.open(Some(window), gtk::gio::Cancellable::NONE, move |res| {
        if let Some(file) = this.handle_error(|| res?.open_readwrite(gtk::gio::Cancellable::NONE))
        {
            f(Box::new(file.input_stream().into_read()))
        }
    })
}
