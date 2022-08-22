use crate::channels::{AnalogInputChannel, AnalogInputChannelBuilder, ChannelBuilder};
use crate::daqmx_call;
use crate::error::{handle_error, Result};
use crate::types::buffer_to_string;
use ni_daqmx_sys::DAQmxGetTaskName;
/// Provides a wrapper and functions for the DAQmx Task
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;
use std::sync::Arc;

/// New type for the raw task handle from the C FFI
struct TaskHandle(ni_daqmx_sys::TaskHandle);

impl Drop for TaskHandle {
    fn drop(&mut self) {
        unsafe { ni_daqmx_sys::DAQmxClearTask(self.0) };
    }
}

#[derive(Clone)]
///Marker type for an analog input task.
pub struct AnalogInput;

#[derive(Clone)]
pub struct Task<TYPE> {
    handle: Arc<TaskHandle>,
    channel_type: PhantomData<TYPE>,
}

impl<TYPE> Task<TYPE> {
    ///Get the sys crate handle for the task.
    ///
    /// This is designed for immediate use in the FFI.
    /// You should not hold this raw handle yourself as you
    /// lose the memory safety given by the wrapped task.
    pub(crate) fn raw_handle(&self) -> ni_daqmx_sys::TaskHandle {
        self.handle.0
    }

    /// Create a new task handle from a name. For use in specific task types.
    pub fn new(name: &str) -> Result<Self> {
        let c_name = CString::new(name)?;

        let handle = {
            let mut tmp_handle: ni_daqmx_sys::TaskHandle = ptr::null_mut();
            daqmx_call!(ni_daqmx_sys::DAQmxCreateTask(
                c_name.as_ptr(),
                &mut tmp_handle
            ))?;
            tmp_handle
        };

        Ok(Self {
            handle: Arc::new(TaskHandle(handle)),
            channel_type: PhantomData,
        })
    }

    /// Gets the name assigned to the task in DAQmx.
    ///
    /// Useful if no name is specified.
    ///
    /// # Example
    /// ```
    /// use daqmx::Task;
    ///
    /// let mut task = Task::new("").unwrap();
    /// let name = task.name().unwrap();
    ///
    /// // Returns Non-Empty Name
    /// assert_ne!(&name, "");
    /// ```
    pub fn name(&mut self) -> Result<String> {
        //first call to get size.
        let return_code = unsafe { DAQmxGetTaskName(self.raw_handle(), std::ptr::null_mut(), 0) };
        if return_code < 0 {
            handle_error(return_code)?;
        }

        let buffer_size = return_code as u32;
        let mut buffer: Vec<i8> = vec![0i8; buffer_size as usize];
        daqmx_call!(DAQmxGetTaskName(
            self.raw_handle(),
            buffer.as_mut_ptr(),
            buffer_size
        ))?;

        Ok(buffer_to_string(buffer))
    }
}
