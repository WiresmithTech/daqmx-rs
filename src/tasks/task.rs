use crate::daqmx_call;
use crate::error::{handle_error, Result};
use crate::types::*;
use ni_daqmx_sys::DAQmxGetTaskName;
/// Provides a wrapper and functions for the DAQmx Task
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;
use std::sync::Arc;

/// New type for the raw task handle from the C FFI
struct TaskHandle(ni_daqmx_sys::TaskHandle);

unsafe impl Send for TaskHandle {}
unsafe impl Sync for TaskHandle {}

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
    /// use daqmx::tasks::{Task, AnalogInput};
    ///
    /// let mut task = Task::<AnalogInput>::new("").unwrap();
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

    /// Configure a hardware timed task with the provided parameters.
    ///
    /// # Argument Notes
    ///
    /// * If [`None`] is provided to source then the onboard clock is used.
    /// * For external sources, set `rate` to the maximum expected frequency.
    /// * [`ClockEdge`] provides a default if you aren't concerned with the actual edge. This is the rising edge.
    /// * For a continuous acquisition, `samples per channel` is used to configure the internal buffer size.
    pub fn configure_sample_clock_timing(
        &mut self,
        source: Option<&str>,
        rate: f64,
        edge: ClockEdge,
        mode: SampleMode,
        samples_per_channel: u64,
    ) -> Result<()> {
        let source_c = match source {
            Some(name) => CString::new(name)?,
            None => CString::new("OnboardClock")?,
        };

        daqmx_call!(ni_daqmx_sys::DAQmxCfgSampClkTiming(
            self.raw_handle(),
            source_c.as_ptr(),
            rate,
            edge.into(),
            mode.into(),
            samples_per_channel
        ))
    }

    /// Transitions the task from the committed state to the running state, which begins measurement or generation.
    /// Using this function is required for some applications and optional for others.
    ///
    /// If you do not use this function, a measurement task starts automatically when a read operation begins.
    /// The autoStart parameter of the NI-DAQmx Write functions determines if a generation task starts automatically when you use an NI-DAQmx Write function.
    ///
    ///If you do not call [`Task::start`] and [`Task::stop`] when you call NI-DAQmx Read functions or NI-DAQmx Write functions multiple times, such as in a loop, the task starts and stops repeatedly.
    /// Starting and stopping a task repeatedly reduces the performance of the application.
    pub fn start(&mut self) -> Result<()> {
        daqmx_call!(ni_daqmx_sys::DAQmxStartTask(self.raw_handle()))
    }

    /// Stops the task and returns it to the state it was in before it was started.
    ///
    ///If you do not call [`Task::start`] and [`Task::stop`] when you call NI-DAQmx Read functions or NI-DAQmx Write functions multiple times, such as in a loop, the task starts and stops repeatedly.
    /// Starting and stopping a task repeatedly reduces the performance of the application.
    pub fn stop(&mut self) -> Result<()> {
        daqmx_call!(ni_daqmx_sys::DAQmxStopTask(self.raw_handle()))
    }

    /// Waits for the measurement or generation to complete. Use this function to ensure that the specified operation is complete before you stop the task.
    pub fn wait_until_done(&mut self, timeout: Timeout) -> Result<()> {
        daqmx_call!(ni_daqmx_sys::DAQmxWaitUntilTaskDone(
            self.raw_handle(),
            timeout.into()
        ))
    }

    ///Gets whether DAQmx read automatically starts the task.
    pub fn read_auto_start(&mut self) -> Result<bool> {
        let mut value: bool32 = 0;
        daqmx_call!(ni_daqmx_sys::DAQmxGetReadAutoStart(
            self.raw_handle(),
            &mut value
        ))?;
        Ok(value != 0)
    }

    ///Sets whether DAQmx read automatically starts the task.
    pub fn set_read_auto_start(&mut self, value: bool) -> Result<()> {
        let value: bool32 = match value {
            true => 1,
            false => 0,
        };
        daqmx_call!(ni_daqmx_sys::DAQmxSetReadAutoStart(
            self.raw_handle(),
            value
        ))?;
        Ok(())
    }
}
