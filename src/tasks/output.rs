/// Provides traits around input task behaviours - notably reading.
///
/// In future it may expose a reader struct for managing the buffers and providing
/// the different data representations for us.
use ni_daqmx_sys::bool32;

use crate::daqmx_call;
use crate::error::Result;
use crate::types::{DataFillMode, Timeout};

pub enum RegenerationMode {
    /// Allow regeneration which means the system can loop over the same buffer multiple times.
    Allowed,
    /// Do not allow regeneration of samples. Once the buffer is complete the output will stop.
    NotAllowed,
}

pub trait OutputTask<T>: DAQmxOutput<T> {
    /// Read a single value from the task with the given timeout.
    fn write_scalar(&mut self, timeout: Timeout, auto_start: bool, value: T) -> Result<()>;

    /// Writes an array of samples from the task where the array can hold multiple channels and/or multiple samples.
    ///
    /// # Buffer
    ///
    /// The buffer should be large enough to contain the number of samples * the number of channels that you want to read.
    fn write(
        &mut self,
        timeout: Timeout,
        auto_start: bool,
        fill_mode: DataFillMode,
        samples_per_channel: usize,
        values: &mut [T],
    ) -> Result<i32> {

        let mut actual_samples_per_channel = 0;

        daqmx_call!(self.daqmx_write(
            samples_per_channel as i32,
            auto_start.into(),
            timeout.into(),
            fill_mode.into(),
            values.as_ptr(),
            &mut actual_samples_per_channel as *mut i32
        ))?;

        Ok(actual_samples_per_channel)
    }

}

pub trait DAQmxOutput<T> {
    /// A basic wrapper for the daqmx read function so that implementers don't have to repeat common setup for input task.
    unsafe fn daqmx_write(
        &mut self,
        samples_per_channel: i32,
        auto_start: bool32,
        timeout: f64,
        data_layout: bool32,
        buffer: *const T,
        actual_samples_per_channel: *mut i32,
    ) -> i32;
}
