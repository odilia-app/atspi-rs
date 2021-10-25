use std::ptr;

use glib::{translate::*, IsA};

use crate::{
    events::DeviceListener, KeyDefinition, KeyEventMask, KeyListenerSyncType, KeyMaskType,
};

// ALL OF THIS IS A HACK FOR NOW! IT SHOULD WORK, BUT I WOULDN'T TRUST THE COMPILER NOT TO BREAK IT.

#[doc(alias = "atspi_deregister_keystroke_listener")]
pub fn register_keystroke_listener<P: IsA<DeviceListener>>(
    listener: &P,
    key_set: Option<Vec<KeyDefinition>>,
    modmask: KeyMaskType,
    event_types: KeyEventMask,
    sync_type: KeyListenerSyncType,
) -> Result<(), glib::Error> {
    let _stashes;
    let key_set = if let Some(ref key_set) = key_set {
        let (array_ptr, s) = garray_from_slice_none(key_set);
        _stashes = s;
        array_ptr
    } else {
        ptr::null_mut()
    };
    let mut err = ptr::null_mut();
    let _res = unsafe {
        ffi::atspi_register_keystroke_listener(
            listener.as_ref().to_glib_none().0,
            key_set,
            modmask,
            event_types,
            sync_type.bits(),
            &mut err,
        )
    };
    if err.is_null() {
        Ok(())
    } else {
        Err(unsafe { from_glib_full(err) })
    }
}

pub fn garray_from_slice_none<'a, T, U>(
    slice: &'a [U],
) -> (*mut glib::ffi::GArray, Vec<Stash<'a, T, U>>)
where
    T: Copy + Ptr,
    U: 'a + ToGlibPtr<'a, T>,
{
    use std::convert::TryInto;

    let len = slice
        .len()
        .try_into()
        .expect("Vec is too long to fit into a GArray");
    // Create a new GArray of the correct size
    let ptr = unsafe {
        glib::ffi::g_array_sized_new(
            false.into(),
            false.into(),
            std::mem::size_of::<T>()
                .try_into()
                .expect("Pointer is too large to fit into a GArray"),
            len,
        )
    };
    // Convert Rust types to glib types
    // This Vec is the backing storage for the glib types and the pointers to them
    let stashes: Vec<_> = slice.iter().map(|i| i.to_glib_none()).collect();
    // Add all the elements
    for elem in &stashes {
        unsafe {
            glib::ffi::g_array_append_vals(ptr, elem.0.to(), 1);
        }
    }
    assert!(!ptr.is_null(), "Out of memory");
    (ptr, stashes)
}
