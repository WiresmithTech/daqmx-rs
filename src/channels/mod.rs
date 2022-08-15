use std::ffi::CString;

use crate::daqmx_call;
use crate::error::{handle_error, Result};
use crate::tasks::TaskHandle;

pub trait AnalogInputChannel<'a>: Sized {
    fn new(task: &'a mut TaskHandle, name: &'a str) -> Result<Self>;
}

pub struct AnalogInputChannelBase<'a> {
    task: &'a mut TaskHandle,
    name: CString,
}

impl<'a> AnalogInputChannel<'a> for AnalogInputChannelBase<'a> {
    fn new(task: &'a mut TaskHandle, name: &'a str) -> Result<Self> {
        let name = CString::new(name)?;
        Ok(Self { task, name })
    }
}

impl<'a> AnalogInputChannelBase<'a> {
    pub fn ai_max(&self) -> Result<f64> {
        let mut value: f64 = 0.0;
        daqmx_call!(ni_daqmx_sys::DAQmxGetAIMax(
            *self.task,
            self.name.as_ptr(),
            &mut value
        ))?;
        Ok(value)
    }
}

pub struct VoltageInputChannel<'a> {
    ai_channel: AnalogInputChannelBase<'a>,
}
