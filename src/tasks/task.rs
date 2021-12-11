/// Provides a wrapper and functions for the DAQmx Task
use std::ffi::CString;
use std::ptr;
use nidaqmx_sys::{DAQmxGetTaskName};
use crate::error::{Result, handle_error};
use crate::types::{buffer_to_string};

pub type TaskHandle = nidaqmx_sys::TaskHandle;

pub trait Task {
    fn handle(&mut self) -> TaskHandle;

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
    fn name(&mut self) -> Result<String> {

        //first call to get size.
        let return_code = unsafe { DAQmxGetTaskName(self.handle(), std::ptr::null_mut(), 0)};
        if return_code < 0 {
            handle_error(return_code)?;
        }
        
        let buffer_size = return_code as u32;
        let mut buffer: Vec<i8> = vec![0i8; buffer_size as usize];
        let return_code = unsafe { DAQmxGetTaskName(self.handle(), buffer.as_mut_ptr(), buffer_size)};
        handle_error(return_code)?;


        Ok(buffer_to_string(buffer))
    }
    
    fn clear(&mut self) {
        unsafe { nidaqmx_sys::DAQmxClearTask(self.handle()) };
    }
}

/// Create a new task handle from a name. For use in specific task types.
pub fn new_task(name: &str) -> Result<TaskHandle> {

    let c_name = CString::new(name)?;

    let handle = unsafe {
        let mut tmp_handle: nidaqmx_sys::TaskHandle = ptr::null_mut();
        let error_code = nidaqmx_sys::DAQmxCreateTask(c_name.as_ptr(), &mut tmp_handle);
        handle_error(error_code)?;
        tmp_handle
    };

    Ok(handle)
}




pub trait InputTask<T> {
    fn read(&mut self) -> T;
}

/* impl InputTask<f64> for Task {

    fn read(&mut self) -> f64 {
        let mut value: f64 = 0.0;
        let error_code = unsafe {
            nidaqmx_sys::DAQmxReadAnalogScalarF64(self.task_handle(), 1.0, &mut value, std::ptr::null_mut() )
        };
        return value;

    }

} */



