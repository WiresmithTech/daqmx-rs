use crate::channels::{ChannelBuilder, TaskChannel};
use crate::daqmx_call;
use crate::error::Result;
use crate::types::Timeout;
use std::ffi::CString;
use std::ptr;
use crate::channels::ai_channels::AnalogInputKind;
use super::input::{DAQmxInput, InputTask};
use super::{Task, task::AnalogInput};

impl Task<AnalogInput> {
    pub fn create_channel<K: AnalogInputKind, B: ChannelBuilder<Kind = K>>(
        &mut self,
        builder: B,
    ) -> Result<TaskChannel<K>> {
        builder.add_to_task(self.raw_handle())
    }

    pub fn get_channel<K: AnalogInputKind>(&self, name: &str) -> Result<TaskChannel<K>> {
        //todo: Check the channel exists and it is the correct type.
        let name = CString::new(name)?;
        Ok(TaskChannel::new(self.raw_handle(), name))
    }
}

impl InputTask<f64> for Task<AnalogInput> {
    fn read_scalar(&mut self, timeout: Timeout) -> Result<f64> {
        let mut value = 0.0;
        daqmx_call!(ni_daqmx_sys::DAQmxReadAnalogScalarF64(
            self.raw_handle(),
            timeout.into(),
            &mut value,
            ptr::null_mut(),
        ))?;
        Ok(value)
    }
}

impl DAQmxInput<f64> for Task<AnalogInput> {
    unsafe fn daqmx_read(
        &mut self,
        samples_per_channel: i32,
        timeout: f64,
        fill_mode: ni_daqmx_sys::bool32,
        buffer: *mut f64,
        buffer_size: u32,
        actual_samples_per_channel: *mut i32,
    ) -> i32 {
        unsafe {
            ni_daqmx_sys::DAQmxReadAnalogF64(
                self.raw_handle(),
                samples_per_channel,
                timeout,
                fill_mode,
                buffer,
                buffer_size,
                actual_samples_per_channel,
                ptr::null_mut(),
            )
        }
    }
}
