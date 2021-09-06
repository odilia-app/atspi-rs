// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::DeviceEvent;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtspiDeviceListener")]
    pub struct DeviceListener(Object<ffi::AtspiDeviceListener, ffi::AtspiDeviceListenerClass>);

    match fn {
        type_ => || ffi::atspi_device_listener_get_type(),
    }
}

impl DeviceListener {
    #[doc(alias = "atspi_device_listener_new")]
    pub fn new<P: Fn(&DeviceEvent) -> bool + 'static>(callback: P) -> DeviceListener {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&DeviceEvent) -> bool + 'static>(stroke: *const ffi::AtspiDeviceEvent, user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
            let stroke = from_glib_full(stroke);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&stroke);
            res.into_glib()
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn callback_destroyed_func<P: Fn(&DeviceEvent) -> bool + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call2 = Some(callback_destroyed_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib_full(ffi::atspi_device_listener_new(callback, Box_::into_raw(super_callback0) as *mut _, destroy_call2))
        }
    }

    //#[doc(alias = "atspi_device_listener_new_simple")]
    //pub fn new_simple<P: Fn(&DeviceEvent) -> bool + 'static>(callback: P) -> DeviceListener {
    //    unsafe { TODO: call ffi:atspi_device_listener_new_simple() }
    //}
}

pub const NONE_DEVICE_LISTENER: Option<&DeviceListener> = None;

pub trait DeviceListenerExt: 'static {
    #[doc(alias = "atspi_device_listener_add_callback")]
    fn add_callback<P: Fn(&DeviceEvent) -> bool + 'static>(&self, callback: P);

    //#[doc(alias = "atspi_device_listener_remove_callback")]
    //fn remove_callback<P: FnMut(&DeviceEvent) -> bool>(&self, callback: P);
}

impl<O: IsA<DeviceListener>> DeviceListenerExt for O {
    fn add_callback<P: Fn(&DeviceEvent) -> bool + 'static>(&self, callback: P) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&DeviceEvent) -> bool + 'static>(stroke: *const ffi::AtspiDeviceEvent, user_data: glib::ffi::gpointer) -> glib::ffi::gboolean {
            let stroke = from_glib_full(stroke);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&stroke);
            res.into_glib()
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn callback_destroyed_func<P: Fn(&DeviceEvent) -> bool + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call2 = Some(callback_destroyed_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::atspi_device_listener_add_callback(self.as_ref().to_glib_none().0, callback, destroy_call2, Box_::into_raw(super_callback0) as *mut _);
        }
    }

    //fn remove_callback<P: FnMut(&DeviceEvent) -> bool>(&self, callback: P) {
    //    unsafe { TODO: call ffi:atspi_device_listener_remove_callback() }
    //}
}

impl fmt::Display for DeviceListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceListener")
    }
}