// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT


glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceEvent(Boxed<ffi::AtspiDeviceEvent>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::atspi_device_event_get_type(), ptr as *mut _) as *mut ffi::AtspiDeviceEvent,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::atspi_device_event_get_type(), ptr as *mut _),
        type_ => || ffi::atspi_device_event_get_type(),
    }
}
