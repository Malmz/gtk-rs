// This file was generated by gir (81f9b8c) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ListBoxRow(Object<ffi::GtkListBoxRow>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_list_box_row_get_type(),
    }
}

impl ListBoxRow {
    #[cfg(feature = "v3_10")]
    pub fn new() -> ListBoxRow {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_box_row_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn changed(&self) {
        unsafe {
            ffi::gtk_list_box_row_changed(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_activatable(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_header(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_row_get_header(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_list_box_row_get_index(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_selectable(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_is_selected(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_activatable(&self, activatable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_activatable(self.to_glib_none().0, activatable.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_header<T: IsA<Widget>>(&self, header: Option<&T>) {
        unsafe {
            ffi::gtk_list_box_row_set_header(self.to_glib_none().0, header.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_selectable(self.to_glib_none().0, selectable.to_glib());
        }
    }

    pub fn connect_activate<F: Fn(&ListBoxRow) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ListBoxRow) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline(this: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ListBoxRow) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
