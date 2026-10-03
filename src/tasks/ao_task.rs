use std::ffi::CString;
use ni_daqmx_sys::bool32;
use crate::channels::{AnalogOutputKind, ChannelBuilder, TaskChannel};
use crate::daqmx_call;
use crate::tasks::{AnalogInput, AnalogOutput, Task};
use crate::tasks::output::{DAQmxOutput, OutputTask};
use crate::types::Timeout;

impl Task<AnalogOutput> {

    pub fn create_channel<K: AnalogOutputKind, B: ChannelBuilder<Kind = K>>(
        &mut self,
        builder: B,
    ) -> crate::error::Result<TaskChannel<K>> {
        builder.add_to_task(self.raw_handle())
    }

    pub fn get_channel<K: AnalogOutputKind>(&self, name: &str) -> crate::error::Result<TaskChannel<K>> {
        //todo: Check the channel exists and it is the correct type.
        let name = CString::new(name)?;
        Ok(TaskChannel::new(self.raw_handle(), name))
    }
}


impl OutputTask<f64> for Task<AnalogOutput> {
    fn write_scalar(&mut self, timeout: Timeout, auto_start: bool, value: f64) -> crate::error::Result<()> {
        daqmx_call!(ni_daqmx_sys::DAQmxWriteAnalogScalarF64(
            self.raw_handle(),
            auto_start.into(),
            timeout.into(),
            value,
            std::ptr::null_mut(),
        )) 
    }
}


impl DAQmxOutput<f64> for Task<AnalogOutput> {
    unsafe fn daqmx_write(&mut self, samples_per_channel: i32, auto_start: bool32, timeout: f64, data_layout: bool32, buffer: *const f64, actual_samples_per_channel: *mut i32) -> i32 {
        unsafe {
            ni_daqmx_sys::DAQmxWriteAnalogF64(
                self.raw_handle(),
                samples_per_channel,
                auto_start.into(),
                timeout.into(),
                data_layout,
                buffer,
                actual_samples_per_channel,
                std::ptr::null_mut(),
            )
        }
    }
}