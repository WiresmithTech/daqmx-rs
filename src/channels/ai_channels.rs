use delegate::delegate;
use std::ffi::CString;

use ni_daqmx_sys::*;

use crate::daqmx_call;
use crate::error::{handle_error, Result};
use crate::tasks::{AnalogInput, Task};
//feels like a circular dependency. Don't love it.
use super::Channel;

macro_rules! delegate_ai_channel {
    () => {
        delegate! {
                to self.ai_channel {
                    pub fn ai_max(&self) -> Result<f64>;
                    pub fn physical_channel(&self) -> Result<String>;
                }
        }
    };
}

pub trait AnalogInputChannel: Sized {
    fn new(task: Task<AnalogInput>, name: &str) -> Result<Self>;
}

pub struct AnalogInputChannelBase {
    task: Task<AnalogInput>,
    name: CString,
}

impl AnalogInputChannel for AnalogInputChannelBase {
    fn new(task: Task<AnalogInput>, name: &str) -> Result<Self> {
        let name = CString::new(name)?;
        Ok(Self { task, name })
    }
}

impl Channel for AnalogInputChannelBase {
    fn raw_handle(&self) -> *mut std::os::raw::c_void {
        self.task.raw_handle()
    }

    fn name(&self) -> &std::ffi::CStr {
        &self.name
    }
}

impl AnalogInputChannelBase {
    pub fn physical_channel(&self) -> Result<String> {
        self.read_channel_property_string(ni_daqmx_sys::DAQmxGetPhysicalChanName)
    }
    pub fn ai_max(&self) -> Result<f64> {
        self.read_channel_property(ni_daqmx_sys::DAQmxGetAIMax)
    }
    pub fn ai_min(&self) -> Result<f64> {
        self.read_channel_property(ni_daqmx_sys::DAQmxGetAIMin)
    }
}

pub struct VoltageInputChannel {
    ai_channel: AnalogInputChannelBase,
}

impl VoltageInputChannel {
    delegate_ai_channel!();
}

impl AnalogInputChannel for VoltageInputChannel {
    fn new(task: Task<AnalogInput>, name: &str) -> Result<Self> {
        let ai_channel = AnalogInputChannelBase::new(task, name)?;
        Ok(Self { ai_channel })
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// Defines the input configuration for the analog input.
pub enum AnalogTerminalConfig {
    /// Uses the [default for the type/hardware combination](https://www.ni.com/docs/en-US/bundle/ni-daqmx-device-considerations/page/defaulttermconfig.html).
    Default = DAQmx_Val_Cfg_Default,
    /// Configures inputs for reference single ended (reference to AI GND)
    RSE = DAQmx_Val_RSE as i32,
    /// Cofngures inputs for non-reference single ended (reference to AI SENSE)
    NRSE = DAQmx_Val_NRSE as i32,
    /// Configures inputs for differential mode.
    Differential = DAQmx_Val_Diff as i32,
    /// Configures inputs for pseudo-differential mode
    PseudoDifferential = DAQmx_Val_PseudoDiff as i32,
}

impl Default for AnalogTerminalConfig {
    fn default() -> Self {
        AnalogTerminalConfig::Default
    }
}

#[derive(Clone)]
pub enum VoltageScale {
    Volts,
    CustomScale(CString),
}

impl From<VoltageScale> for i32 {
    fn from(scale: VoltageScale) -> Self {
        match scale {
            VoltageScale::Volts => DAQmx_Val_Volts as i32,
            VoltageScale::CustomScale(_) => DAQmx_Val_FromCustomScale as i32,
        }
    }
}

///For the scale name.
impl From<VoltageScale> for CString {
    fn from(scale: VoltageScale) -> Self {
        match scale {
            VoltageScale::Volts => CString::default(),
            VoltageScale::CustomScale(name) => name.clone(),
        }
    }
}

pub trait ChannelBuilder {
    fn add_to_task(self, task: TaskHandle) -> Result<()>;
}

/// Marker trait for Analog Input channel builders so the task can adapt to the type.
pub trait AnalogInputChannelBuilder: ChannelBuilder {}

pub struct VoltageChannelBuilder {
    physical_channel: CString,
    name: Option<CString>,
    pub max: f64,
    pub min: f64,
    pub scale: VoltageScale,
    pub terminal_config: AnalogTerminalConfig,
}

impl VoltageChannelBuilder {
    pub fn new<S: Into<Vec<u8>>>(physical_channel: S) -> Result<Self> {
        Ok(Self {
            physical_channel: CString::new(physical_channel)?,
            name: None,
            max: 5.0,
            min: -5.0,
            scale: VoltageScale::Volts,
            terminal_config: AnalogTerminalConfig::Default,
        })
    }

    pub fn name<S: Into<Vec<u8>>>(&mut self, name: S) -> Result<&mut Self> {
        self.name = Some(CString::new(name)?);
        Ok(self)
    }
}

impl ChannelBuilder for VoltageChannelBuilder {
    fn add_to_task(self, task: TaskHandle) -> Result<()> {
        let empty_string = CString::default();
        daqmx_call!(ni_daqmx_sys::DAQmxCreateAIVoltageChan(
            task,
            self.physical_channel.as_ptr(),
            self.name.as_ref().unwrap_or(&empty_string).as_ptr(),
            self.terminal_config as i32,
            self.min,
            self.max,
            self.scale.clone().into(),
            CString::from(self.scale).as_ptr(),
        ))
    }
}

impl AnalogInputChannelBuilder for VoltageChannelBuilder {}
