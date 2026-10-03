use crate::daqmx_call;
use crate::error::{DaqmxError, handle_error, string_property_size_error};
use crate::properties::PropertyValue;
use crate::tasks::Task;
use ni_daqmx_sys::TaskHandle;

// int32 DAQmxGetXXX(TaskHandle, const char* chan, T* value)
pub type ScalarGetter<T> = unsafe extern "C" fn(TaskHandle, *mut T) -> i32;
// int32 DAQmxSetXXX(TaskHandle, const char* chan, T value)
pub type ScalarSetter<T> = unsafe extern "C" fn(TaskHandle, T) -> i32;

pub type Resetter = unsafe extern "C" fn(TaskHandle) -> i32;

impl<TYPE> Task<TYPE> {
    ///Read a channel property as a string, given a raw DAQmx Function.
    pub fn property_get_string(
        &self,
        daqmx_fn: unsafe extern "C" fn(
            ni_daqmx_sys::TaskHandle,
            *mut std::os::raw::c_char,
            u32,
        ) -> i32,
    ) -> crate::error::Result<String> {
        let return_value = unsafe { daqmx_fn(self.raw_handle(), std::ptr::null_mut(), 0) };

        if return_value < 0 {
            handle_error(return_value)?;
        }

        let buffer_size = return_value as u32;

        let mut buffer = vec![0u8; return_value as usize];

        let return_value = unsafe {
            daqmx_fn(
                self.raw_handle(),
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

    fn property_get_raw<T: Default>(&self, daqmx_fn: ScalarGetter<T>) -> crate::error::Result<T> {
        let mut value: T = T::default();

        daqmx_call!(daqmx_fn(self.raw_handle(), &mut value))?;

        Ok(value)
    }

    pub(crate) fn property_set_raw<T: Default>(
        &self,
        daqmx_fn: ScalarSetter<T>,
        value: T,
    ) -> crate::error::Result<()> {
        daqmx_call!(daqmx_fn(self.raw_handle(), value))?;

        Ok(())
    }

    pub fn property_reset(&self, daqmx_fn: Resetter) -> crate::error::Result<()> {
        daqmx_call!(daqmx_fn(self.raw_handle(),))?;

        Ok(())
    }

    pub fn property_get<T: PropertyValue>(
        &self,
        get_fn: ScalarGetter<T::Raw>,
    ) -> crate::error::Result<T> {
        T::from_raw(self.property_get_raw(get_fn)?)
    }

    pub fn property_set<T: PropertyValue>(
        &self,
        set_fn: ScalarSetter<T::Raw>,
        value: T,
    ) -> crate::error::Result<()> {
        self.property_set_raw(set_fn, value.into_raw())
    }
}
