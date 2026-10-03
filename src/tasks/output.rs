use crate::channels::AnalogOutputKind;
use crate::daqmx_call;
use crate::error::{DaqmxError, Result};
use crate::properties::PropertyValue;
use crate::property;
use crate::tasks::{AnalogOutput, Task};
use crate::types::{DataFillMode, Timeout};
use ni_daqmx_sys::{DAQmx_Val_AllowRegen, DAQmx_Val_DoNotAllowRegen};
/// Provides traits around input task behaviours - notably reading.
///
/// In future it may expose a reader struct for managing the buffers and providing
/// the different data representations for us.
use ni_daqmx_sys::{DAQmx_Val_CurrWritePos, DAQmx_Val_FirstSample, bool32};

/// Whether the output task can reuse the same samples again, allowing for a looping effect
/// over the buffer.
///
/// See <https://www.ni.com/en/support/documentation/supplemental/06/analog-output-regeneration-in-ni-daqmx.html>
/// for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegenerationMode {
    /// Allow regeneration which means the system can loop over the same buffer multiple times.
    Allowed,
    /// Do not allow regeneration of samples. Once the buffer is complete the output will stop.
    NotAllowed,
}

impl PropertyValue for RegenerationMode {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> std::result::Result<Self, DaqmxError> {
        match raw {
            DAQmx_Val_AllowRegen => Ok(RegenerationMode::Allowed),
            DAQmx_Val_DoNotAllowRegen => Ok(RegenerationMode::NotAllowed),
            _ => Err(DaqmxError::UnexpectedValue("Regeneration Mode", raw)),
        }
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            RegenerationMode::Allowed => DAQmx_Val_AllowRegen,
            RegenerationMode::NotAllowed => DAQmx_Val_DoNotAllowRegen,
        }
    }
}

/// Specifies the point in the buffer at which to write data. If you also specify an offset with Offset,
/// the write operation begins at that offset relative to this point you select with this property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteRelativeTo {
    /// Write samples relative to the first sample.
    FirstSample,
    /// Write samples relative to the current write position.
    CurrentWritePosition,
}

impl PropertyValue for WriteRelativeTo {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> std::result::Result<Self, DaqmxError> {
        match raw {
            DAQmx_Val_FirstSample => Ok(WriteRelativeTo::FirstSample),
            DAQmx_Val_CurrWritePos => Ok(WriteRelativeTo::CurrentWritePosition),
            _ => Err(DaqmxError::UnexpectedValue("WriteRelativeTo", raw)),
        }
    }
    fn into_raw(self) -> Self::Raw {
        match self {
            WriteRelativeTo::FirstSample => DAQmx_Val_FirstSample,
            WriteRelativeTo::CurrentWritePosition => DAQmx_Val_CurrWritePos,
        }
    }
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

impl Task<AnalogOutput> {
    property! {
    /// Specifies whether to allow NI-DAQmx to generate the same data multiple times.
    get_set_reset regeneration_mode / set_regeneration_mode / reset_regeneration_mode: RegenerationMode = ni_daqmx_sys::DAQmxGetWriteRegenMode, ni_daqmx_sys::DAQmxSetWriteRegenMode, ni_daqmx_sys::DAQmxResetWriteRegenMode }
    property! {
    /// Specifies the point in the buffer at which to write data. If you also specify an offset with Offset, the write operation begins at that offset relative to this point you select with this property.
    get_set_reset write_relative_to / set_write_relative_to / reset_write_relative_to: WriteRelativeTo = ni_daqmx_sys::DAQmxGetWriteRelativeTo, ni_daqmx_sys::DAQmxSetWriteRelativeTo, ni_daqmx_sys::DAQmxResetWriteRelativeTo }
    property! {
    /// Specifies in samples per channel an offset at which a write operation begins. This offset is relative to the location you specify with `Self::set_write_relative_to`.
    get_set_reset offset / set_offset / reset_offset: i32 = ni_daqmx_sys::DAQmxGetWriteOffset, ni_daqmx_sys::DAQmxSetWriteOffset, ni_daqmx_sys::DAQmxResetWriteOffset }
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
