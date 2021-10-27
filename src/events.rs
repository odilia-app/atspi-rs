use std::os::raw::c_int;

use glib::translate::*;

use crate::accessible::Accessible;

pub use crate::auto::{traits::EventListenerExt, DeviceListener, Event, EventListener, EventType};

pub trait EventExt {
    fn detail1(&self) -> c_int;
    fn detail2(&self) -> c_int;
    fn any_data(&self) -> glib::Value;
    fn source(&self) -> Option<Accessible>;
    fn sender(&self) -> Option<Accessible>;
    fn kind(&self) -> glib::GString;
}

impl EventExt for Event {
    fn detail1(&self) -> c_int {
        let stash = self.to_glib_none();
        unsafe { (&*stash.0).detail1 }
    }

    fn detail2(&self) -> c_int {
        let stash = self.to_glib_none();
        unsafe { (&*stash.0).detail2 }
    }

    fn any_data(&self) -> glib::Value {
        let stash = self.to_glib_none();
        unsafe {
            let any_data = &(*stash.0).any_data as *const _;
            from_glib_none(any_data)
        }
    }

    fn source(&self) -> Option<Accessible> {
        let stash = self.to_glib_none();
        unsafe {
            let source = (*stash.0).source;
            from_glib_none(source)
        }
    }

    fn sender(&self) -> Option<Accessible> {
        let stash = self.to_glib_none();
        unsafe {
            let sender = (*stash.0).sender;
            from_glib_none(sender)
        }
    }

    fn kind(&self) -> glib::GString {
        let stash = self.to_glib_none();
        unsafe {
            let kind = (*stash.0).type_;
            from_glib_none(kind)
        }
    }
}
