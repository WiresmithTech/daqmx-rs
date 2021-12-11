/// A task for analog input channels.
/// 
use super::task::{Task, TaskHandle, InputTask, new_task};
use crate::error::Result;

pub struct AnalogInputTask {
    handle: TaskHandle
}

impl Task for AnalogInputTask {
    fn handle(&mut self) -> TaskHandle {
        self.handle.clone()
    }
}

impl AnalogInputTask {

    /// Creates a new task with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - A name for the task. If empty then DAQmx will assign one automatically.
    ///
    /// # Examples
    ///
    /// ```
    /// use daqmx::tasks::AnalogInputTask;
    /// 
    /// let task = AnalogInputTask::new("My Task");
    /// ```
    /// ```
    /// use daqmx::tasks::AnalogInputTask;
    /// 
    /// let task = AnalogInputTask::new("");
    /// ```
    pub fn new(name: &str) -> Result<Self> {

        let handle = new_task(name)?;
        Ok(Self {
            handle
        })

    }



    pub fn add_channel(&mut self, channel: Box<dyn AnalogInputTaskChannel>) -> Result<()> {
        channel.add_to_task(self.task_handle())
    }

    fn task_handle(&self) -> nidaqmx_sys::TaskHandle {
        self.handle
    }
}


impl Drop for AnalogInputTask {
    fn drop(&mut self) {
        self.clear();
    }
}

impl InputTask<f64> for AnalogInputTask {

    fn read(&mut self) -> f64 {
        let mut value: f64 = 0.0;
        let error_code = unsafe {
            nidaqmx_sys::DAQmxReadAnalogScalarF64(self.handle(), 1.0, &mut value, std::ptr::null_mut() )
        };
        return value;

    }

} 

pub trait AnalogInputTaskChannel {

    fn add_to_task(&self, handle: TaskHandle) -> Result<()>;
}