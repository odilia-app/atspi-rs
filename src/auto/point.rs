// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Point(Boxed<ffi::AtspiPoint>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::atspi_point_get_type(), ptr as *mut _) as *mut ffi::AtspiPoint,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::atspi_point_get_type(), ptr as *mut _),
        type_ => || ffi::atspi_point_get_type(),
    }
}
