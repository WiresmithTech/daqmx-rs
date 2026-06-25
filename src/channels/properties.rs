//! The properties module handles properties of channels in a flexible
//! way based on the standardized format of the DAQmx API.

use std::ffi::c_char;
use ni_daqmx_sys::TaskHandle;
use crate::channels::TaskChannel;
use crate::daqmx_call;
use crate::error::{handle_error, string_property_size_error, DaqmxError};

/// Handles conversion between the DAQmx API and the Rust type.
pub trait PropertyValue: Sized {
    /// The value used at the DAQmx API.
    type Raw: Default;
    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError>;
    fn into_raw(self) -> Self::Raw;
}

macro_rules! identity_property {
    ($($t:ty),*) => {$(
        impl PropertyValue for $t {
            type Raw = $t;
            fn from_raw(raw: $t) -> Result<Self, DaqmxError> { Ok(raw) }
            fn into_raw(self) -> $t { self }
        }
    )*};
}

identity_property!(f64, i32, u32, u64);



// int32 DAQmxGetXXX(TaskHandle, const char* chan, T* value)
pub type ScalarGetter<T> = unsafe extern "C" fn(TaskHandle, *const c_char, *mut T) -> i32;
// int32 DAQmxSetXXX(TaskHandle, const char* chan, T value)
pub type ScalarSetter<T> = unsafe extern "C" fn(TaskHandle, *const c_char, T) -> i32;

impl <K> TaskChannel<K> {

    ///Read a channel property as a string, given a raw DAQmx Function.
    pub fn property_get_string(
        &self,
        daqmx_fn: unsafe extern "C" fn(
            ni_daqmx_sys::TaskHandle,
            *const std::os::raw::c_char,
            *mut std::os::raw::c_char,
            u32,
        ) -> i32,
    ) -> crate::error::Result<String> {
        let return_value = unsafe {
            daqmx_fn(
                self.task(),
                self.name().as_ptr(),
                std::ptr::null_mut(),
                0,
            )
        };

        if return_value < 0 {
            handle_error(return_value)?;
        }

        let buffer_size = return_value as u32;

        let mut buffer = vec![0u8; return_value as usize];

        let return_value = unsafe {
            daqmx_fn(
                self.task(),
                self.name().as_ptr(),
                buffer.as_mut_ptr() as *mut std::os::raw::c_char,
                buffer_size,
            )
        };

        let should_retry = string_property_size_error(return_value)?;

        if should_retry {
            // Just error for now - will review retries in the future.
            return Err(DaqmxError::StringPropertyLengthChanged);
        }

        //pop the null off.
        buffer.pop();
        return Ok(String::from_utf8(buffer)?);
    }

    fn property_get_raw<T: Default>(
        &self,
        daqmx_fn: ScalarGetter<T>,
    ) -> crate::error::Result<T> {
        let mut value: T = T::default();

        daqmx_call!(daqmx_fn(
            self.task(),
            self.name().as_ptr(),
            &mut value
        ))?;

        Ok(value)
    }

    fn property_set_raw<T: Default>(&self, daqmx_fn: ScalarSetter<T>, value: T) -> crate::error::Result<()> {
        daqmx_call!(daqmx_fn(
            self.task(),
            self.name().as_ptr(),
            value
        ))?;

        Ok(())
    }

    pub fn property_get<T: PropertyValue>(&self, get_fn: ScalarGetter<T::Raw>) -> crate::error::Result<T> {
        T::from_raw(self.property_get_raw(get_fn)?)
    }

    pub fn property_set<T: PropertyValue>(&self, set_fn: ScalarSetter<T::Raw>, value: T) -> crate::error::Result<()> {
        self.property_set_raw(set_fn, value.into_raw())
    }
}


