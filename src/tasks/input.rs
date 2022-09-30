/// Provides traits around input task behaviours - notably reading.
///
/// In future it may expose a reader struct for managing the buffers and providing
/// the different data representations for us.
use ni_daqmx_sys::bool32;

use crate::daqmx_call;
use crate::error::{handle_error, Result};
use crate::types::{DataFillMode, Timeout};

pub trait InputTask<T>: DAQmxInput<T> {
    /// Read a single value from the task with the given timeout.
    fn read_scalar(&mut self, timeout: Timeout) -> Result<T>;

    /// Reads an array of samples from the task where the array can hold multiple channels and/or multiple samples.
    ///
    /// # Samples Per Channel Behaviour
    ///
    /// * Writing [`None`] on a finite task will wait until the full acquisition is ready to read.
    /// * Writing [`None`] on a continuous task will read all of the samples available in the buffer.
    /// * If you attempt to read more samples than can fit into the buffer, then only the samples that fit in the buffer will be read.
    ///
    /// # Buffer
    ///
    /// The buffer should be large enough to contain the number of samples * the number of channels that you want to read.
    ///
    /// todo: If we timeout we may still read samples - that needs to be expressed with a more complicated return type.
    fn read(
        &mut self,
        timeout: Timeout,
        fill_mode: DataFillMode,
        samples_per_channel: Option<u32>,
        buffer: &mut [T],
    ) -> Result<i32> {
        let mut actual_samples_per_channel = 0;
        let requested_samples_per_channel = match samples_per_channel {
            Some(val) => val as i32,
            None => -1,
        };
        // Just saturate the buffer size at u32 boundary.
        // If it is larger, this will still be memory safe.
        let buffer_length = buffer.len().try_into().unwrap_or(u32::MAX);

        daqmx_call!(self.daqmx_read(
            requested_samples_per_channel,
            timeout.into(),
            fill_mode.into(),
            buffer.as_mut_ptr(),
            buffer_length,
            &mut actual_samples_per_channel as *mut i32
        ))?;

        Ok(actual_samples_per_channel)
    }
}

pub trait DAQmxInput<T> {
    /// A basic wrapper for the daqmx read function so that implementers don't have to repeat common setup for input task.
    unsafe fn daqmx_read(
        &mut self,
        samples_per_channel: i32,
        timeout: f64,
        fill_mode: bool32,
        buffer: *mut T,
        buffer_size: u32,
        actual_samples_per_channel: *mut i32,
    ) -> i32;
}
