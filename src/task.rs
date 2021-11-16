/// Provides a wrapper and functions for the DAQmx Task
use std::{alloc::handle_alloc_error, ffi::CString};
use std::ptr;
use nidaqmx_sys::{DAQmxGetTaskName, TaskHandle};
use crate::error::{Result, handle_error};
use crate::types::{buffer_to_string};


pub struct Task {
    handle: nidaqmx_sys::TaskHandle
}

impl Task {

    /// Creates a new task with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - A name for the task. If empty then DAQmx will assign one automatically.
    ///
    /// # Examples
    ///
    /// ```
    /// use daqmx::Task;
    /// 
    /// let task = Task::new("My Task");
    /// ```
    /// ```
    /// use daqmx::Task;
    /// 
    /// let task = Task::new("");
    /// ```
    pub fn new(name: &str) -> Result<Self> {


        let c_name = CString::new(name)?;

        let handle = unsafe {
            let mut tmp_handle: nidaqmx_sys::TaskHandle = ptr::null_mut();
            let error_code = nidaqmx_sys::DAQmxCreateTask(c_name.as_ptr(), &mut tmp_handle);
            handle_error(error_code)?;
            tmp_handle
        };

        Ok(Self {
            handle
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
        let return_code = unsafe { DAQmxGetTaskName(self.task_handle(), std::ptr::null_mut(), 0)};
        if return_code < 0 {
            handle_error(return_code)?;
        }
        
        let buffer_size = return_code as u32;
        let mut buffer: Vec<i8> = vec![0i8; buffer_size as usize];
        let return_code = unsafe { DAQmxGetTaskName(self.task_handle(), buffer.as_mut_ptr(), buffer_size)};
        handle_error(return_code)?;


        Ok(buffer_to_string(buffer))
    }

    pub fn add_channel(&mut self, channel: Box<dyn TaskChannel>) -> Result<()> {
        channel.add_to_task(self.task_handle())
    }

    pub fn read(&mut self) -> f64 {
        let mut value: f64 = 0.0;
        let error_code = unsafe {
            nidaqmx_sys::DAQmxReadAnalogScalarF64(self.task_handle(), 1.0, &mut value, std::ptr::null_mut() )
        };
        return value;

    }

    fn task_handle(&self) -> nidaqmx_sys::TaskHandle {
        self.handle
    }
}


impl Drop for Task {
    fn drop(&mut self) {
        unsafe { nidaqmx_sys::DAQmxClearTask(self.task_handle()) };
    }
}

pub trait TaskChannel {

    fn add_to_task(&self, handle: TaskHandle) -> Result<()>;
}



