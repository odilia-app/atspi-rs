use std::ptr;

use glib::{object::IsA, translate::*};

pub use crate::auto::{
    traits::TextExt, Text, TextBoundaryType, TextClipType, TextGranularity, TextRange,
};

pub trait TextExtManual {
    fn character_at_offset(&self, offset: i32) -> Result<Option<char>, glib::Error>;
}

impl<O: IsA<Text>> TextExtManual for O {
    fn character_at_offset(&self, offset: i32) -> Result<Option<char>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_text_get_character_at_offset(
                self.as_ref().to_glib_none().0,
                offset,
                &mut error,
            );
            if error.is_null() {
                Ok(char::from_u32(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
