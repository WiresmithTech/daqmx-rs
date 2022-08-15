use crate::channels::{AnalogInputChannel, ChannelBuilder};
use crate::daqmx_call;
use crate::error::{handle_error, Result};
use crate::types::buffer_to_string;
use ni_daqmx_sys::DAQmxGetTaskName;
/// Provides a wrapper and functions for the DAQmx Task
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;

pub type TaskHandle = ni_daqmx_sys::TaskHandle;

/// The task state represents the DAQmx Task State Machine.
///
/// Certain actions on a task will move it through the states
pub enum ChannelType {
    AnalogInput,
    AnalogOutput,
    DigitalInput,
    DigitalOutput,
    CounterInput,
    CounterOutput,
}

pub struct AnalogInput;

pub struct Task<TYPE> {
    handle: TaskHandle,
    channel_type: PhantomData<TYPE>,
}

impl<TYPE> Task<TYPE> {
    fn handle(&mut self) -> TaskHandle {
        self.handle
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
            handle,
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
        let return_code = unsafe { DAQmxGetTaskName(self.handle(), std::ptr::null_mut(), 0) };
        if return_code < 0 {
            handle_error(return_code)?;
        }

        let buffer_size = return_code as u32;
        let mut buffer: Vec<i8> = vec![0i8; buffer_size as usize];
        daqmx_call!(DAQmxGetTaskName(
            self.handle(),
            buffer.as_mut_ptr(),
            buffer_size
        ))?;

        Ok(buffer_to_string(buffer))
    }
}

impl<TYPE> Drop for Task<TYPE> {
    fn drop(&mut self) {
        unsafe { ni_daqmx_sys::DAQmxClearTask(self.handle()) };
    }
}

impl Task<AnalogInput> {
    pub fn create_voltage_channel(
        &mut self,
        physical_channel: &str,
        name: Option<&str>,
    ) -> Result<()> {
        let c_channel = CString::new(physical_channel)?;
        let c_name = CString::new("").unwrap();
        let c_scale = CString::new("").unwrap();

        daqmx_call!(ni_daqmx_sys::DAQmxCreateAIVoltageChan(
            self.handle,
            c_channel.as_ptr(),
            c_name.as_ptr(),
            ni_daqmx_sys::DAQmx_Val_Cfg_Default,
            -10.0,
            10.0,
            ni_daqmx_sys::DAQmx_Val_Volts as i32,
            c_scale.as_ptr(),
        ))
    }

    pub fn create_channel<B: ChannelBuilder>(&mut self, builder: B) -> Result<()> {
        builder.add_to_task(self.handle)
    }

    pub fn get_channel<'a, C: AnalogInputChannel<'a>>(&'a mut self, name: &'a str) -> Result<C> {
        C::new(&mut self.handle, name)
    }

    pub fn configure_channel<'a, C: AnalogInputChannel<'a>>(
        &'a mut self,
        name: &'a str,
        configuration_function: fn(C) -> Result<()>,
    ) -> Result<()> {
        let channel = self.get_channel(name)?;
        configuration_function(channel)
    }

    pub fn read_scalar(&mut self, timeout: std::time::Duration) -> Result<f64> {
        let mut value = 0.0;
        let error_code = unsafe {
            ni_daqmx_sys::DAQmxReadAnalogScalarF64(
                self.handle,
                timeout.as_secs_f64(),
                &mut value,
                ptr::null_mut(),
            )
        };
        handle_error(error_code)?;
        Ok(value)
    }
}
