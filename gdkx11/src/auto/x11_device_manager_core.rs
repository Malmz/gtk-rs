// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct X11DeviceManagerCore(Object<ffi::GdkX11DeviceManagerCore, ffi::GdkX11DeviceManagerCoreClass>) @extends gdk::DeviceManager;

    match fn {
        get_type => || ffi::gdk_x11_device_manager_core_get_type(),
    }
}

impl X11DeviceManagerCore {}

pub const NONE_X11_DEVICE_MANAGER_CORE: Option<&X11DeviceManagerCore> = None;

impl fmt::Display for X11DeviceManagerCore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11DeviceManagerCore")
    }
}
