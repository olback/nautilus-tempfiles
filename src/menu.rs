use nautilus_extension::{
    nautilus_menu_item_activate_cb,
    FileInfo,
    MenuItem,
    MenuProvider
};
use gobject_sys::GObject;
use glib_sys::gpointer;
use gtk_sys::GtkWidget;
use std::path::PathBuf;
use crate::{
    tokio_runtime_builder::TokioRuntimeBuilder,
    upload::upload,
    utils
};
use lazy_static::lazy_static;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

lazy_static! {
    static ref RT: tokio::runtime::Runtime = TokioRuntimeBuilder::new().build().unwrap();
}

pub struct TempfilesMenuProvider { }

impl TempfilesMenuProvider {

    pub fn new() -> Self {

        Self { }

    }

}

impl MenuProvider for TempfilesMenuProvider {

    fn get_file_items(&self, _window: *mut GtkWidget, files: &Vec<FileInfo>) -> Vec<MenuItem> {

        let mut menu_items = Vec::<MenuItem>::new();

        if files.len() == 1 && PathBuf::from(utils::fix_filename(files[0].get_uri())).is_file() {

            let mut menu_item = MenuItem::new(
                "TempfilesUpload::Upload",
                "Upload to TempFiles",
                "Upload this file to TempFiles for 24 hours",
                None
            );
            menu_item.set_activate_cb(upload_clicked_cb);

            menu_items.push(menu_item);

        }

        menu_items

    }

}

nautilus_menu_item_activate_cb!(upload_clicked_cb, upload_clicked);

fn upload_clicked(files: Vec<FileInfo>) {

    if files.len() != 1 {
        return
    }

    let path = PathBuf::from(utils::fix_filename(files[0].get_uri()));

    RT.spawn(async move {

        match upload(path).await {

            Ok(url) => {

                utils::show_notif("Success! File uploaded.");

                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                drop(ctx.set_contents(url));

            },

            Err(err) => utils::show_notif(format!("{}", err))

        }

    });

}
