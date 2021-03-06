// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Accessible;
use crate::Object;
use crate::Range;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "AtspiHyperlink")]
    pub struct Hyperlink(Object<ffi::AtspiHyperlink, ffi::AtspiHyperlinkClass>) @extends Object;

    match fn {
        type_ => || ffi::atspi_hyperlink_get_type(),
    }
}

pub const NONE_HYPERLINK: Option<&Hyperlink> = None;

pub trait HyperlinkExt: 'static {
    #[doc(alias = "atspi_hyperlink_get_end_index")]
    #[doc(alias = "get_end_index")]
    fn end_index(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "atspi_hyperlink_get_index_range")]
    #[doc(alias = "get_index_range")]
    fn index_range(&self) -> Result<Range, glib::Error>;

    #[doc(alias = "atspi_hyperlink_get_n_anchors")]
    #[doc(alias = "get_n_anchors")]
    fn n_anchors(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "atspi_hyperlink_get_object")]
    #[doc(alias = "get_object")]
    fn object(&self, i: i32) -> Result<Accessible, glib::Error>;

    #[doc(alias = "atspi_hyperlink_get_start_index")]
    #[doc(alias = "get_start_index")]
    fn start_index(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "atspi_hyperlink_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self, i: i32) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_hyperlink_is_valid")]
    fn is_valid(&self) -> Result<bool, glib::Error>;
}

impl<O: IsA<Hyperlink>> HyperlinkExt for O {
    fn end_index(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_hyperlink_get_end_index(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn index_range(&self) -> Result<Range, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_hyperlink_get_index_range(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn n_anchors(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_hyperlink_get_n_anchors(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn object(&self, i: i32) -> Result<Accessible, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_hyperlink_get_object(self.as_ref().to_glib_none().0, i, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn start_index(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_hyperlink_get_start_index(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn uri(&self, i: i32) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_hyperlink_get_uri(self.as_ref().to_glib_none().0, i, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_valid(&self) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_hyperlink_is_valid(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for Hyperlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Hyperlink")
    }
}
