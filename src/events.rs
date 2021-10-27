use std::{
    os::raw::{c_int, c_uint, c_ushort},
    time::SystemTime,
};

use glib::translate::*;

use crate::accessible::Accessible;

pub use crate::auto::{
    traits::EventListenerExt, DeviceEvent, DeviceListener, Event, EventListener, EventType,
};

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

pub trait DeviceEventExt {
    fn kind(&self) -> EventType;
    fn id(&self) -> c_uint;
    fn hw_code(&self) -> c_ushort;
    fn modifiers(&self) -> c_ushort;
    fn timestamp(&self) -> SystemTime;
    fn event_string(&self) -> glib::GString;
    fn is_text(&self) -> bool;
}

impl DeviceEventExt for DeviceEvent {
    fn kind(&self) -> EventType {
        let stash = self.to_glib_none();
        unsafe {
            let kind = (*stash.0).type_;
            EventType::from_glib(kind)
        }
    }

    fn id(&self) -> c_uint {
        let stash = self.to_glib_none();
        unsafe { (&*stash.0).id }
    }

    fn hw_code(&self) -> c_ushort {
        let stash = self.to_glib_none();
        unsafe { (&*stash.0).hw_code }
    }

    fn modifiers(&self) -> c_ushort {
        let stash = self.to_glib_none();
        unsafe { (&*stash.0).modifiers }
    }

    fn timestamp(&self) -> SystemTime {
        // Get the raw guint value
        let stash = self.to_glib_none();
        let ts = unsafe { (&*stash.0).timestamp };
        // Convert it into a Duration
        let dur = std::time::Duration::from_secs(ts as _);
        // Convert this to a SystemTime using the UNIX_EPOCH
        std::time::UNIX_EPOCH + dur
    }

    fn event_string(&self) -> glib::GString {
        let stash = self.to_glib_none();
        unsafe {
            let event_string = (*stash.0).event_string;
            from_glib_none(event_string)
        }
    }

    fn is_text(&self) -> bool {
        let stash = self.to_glib_none();
        unsafe {
            let is_text = (*stash.0).is_text;
            bool::from_glib(is_text)
        }
    }
}
