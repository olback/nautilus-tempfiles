use nautilus_extension::{nautilus_module, NautilusModule};
use gobject_sys::GTypeModule;
use glib_sys::GType;
// use glib;
use libc::c_int;
// use clipboard::ClipboardProvider;
// use clipboard::ClipboardContext;

mod config;
mod upload;
mod menu;
mod utils;
mod error;
mod tokio_runtime_builder;
use menu::TempfilesMenuProvider;

fn init(module: *mut GTypeModule) -> GType {

    println!("Initializing Tempfiles Nautilus {}", env!("CARGO_PKG_VERSION"));

    NautilusModule::new(module, "Tempfiles")
        .add_menu_provider(TempfilesMenuProvider::new())
        .register()

}

nautilus_module!(init);
