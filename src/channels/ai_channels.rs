use delegate::delegate;
use std::ffi::CString;

use ni_daqmx_sys::*;

use crate::daqmx_call;
use crate::error::{handle_error, DaqmxError, Result};
use crate::scales::PreScaledUnits;
use crate::tasks::{AnalogInput, Task};
//feels like a circular dependency. Don't love it.
use super::Channel;

macro_rules! delegate_ai_channel {
    () => {
        delegate! {
                to self.ai_channel {
                    pub fn ai_max(&self) -> Result<f64>;
                    pub fn ai_min(&self) -> Result<f64>;
                    pub fn physical_channel(&self) -> Result<String>;
                    pub fn ai_terminal_config(&self) -> Result<AnalogTerminalConfig>;
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
    pub fn ai_terminal_config(&self) -> Result<AnalogTerminalConfig> {
        self.read_channel_property(ni_daqmx_sys::DAQmxGetAITermCfg)?
            .try_into()
    }
    pub fn custom_scale_name(&self) -> Result<String> {
        self.read_channel_property_string(ni_daqmx_sys::DAQmxGetAICustomScaleName)
    }
}

pub struct VoltageInputChannel {
    ai_channel: AnalogInputChannelBase,
}

impl VoltageInputChannel {
    delegate_ai_channel!();
    pub fn scale(&self) -> Result<VoltageScale> {
        let scale: VoltageScale = self
            .ai_channel
            .read_channel_property(DAQmxGetAIVoltageUnits)?
            .try_into()?;

        if let VoltageScale::CustomScale(_) = scale {
            let name = self.ai_channel.custom_scale_name()?;
            Ok(VoltageScale::CustomScale(Some(CString::new(name)?)))
        } else {
            Ok(scale)
        }
    }
}

impl AnalogInputChannel for VoltageInputChannel {
    fn new(task: Task<AnalogInput>, name: &str) -> Result<Self> {
        let ai_channel = AnalogInputChannelBase::new(task, name)?;
        Ok(Self { ai_channel })
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Defines the input configuration for the analog input.
pub enum AnalogTerminalConfig {
    /// Uses the [default for the type/hardware combination](https://www.ni.com/docs/en-US/bundle/ni-daqmx-device-considerations/page/defaulttermconfig.html).
    Default = DAQmx_Val_Cfg_Default,
    /// Configures inputs for reference single ended (reference to AI GND)
    RSE = DAQmx_Val_RSE,
    /// Cofngures inputs for non-reference single ended (reference to AI SENSE)
    NRSE = DAQmx_Val_NRSE,
    /// Configures inputs for differential mode.
    Differential = DAQmx_Val_Diff,
    /// Configures inputs for pseudo-differential mode
    PseudoDifferential = DAQmx_Val_PseudoDiff,
}

impl Default for AnalogTerminalConfig {
    fn default() -> Self {
        AnalogTerminalConfig::Default
    }
}

impl TryFrom<i32> for AnalogTerminalConfig {
    type Error = DaqmxError;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        //The if statements look wierd but seemed like the best way for the type conversion to be combined.
        match value {
            DAQmx_Val_Cfg_Default => Ok(Self::Default),
            DAQmx_Val_RSE => Ok(Self::RSE),
            DAQmx_Val_NRSE => Ok(Self::NRSE),
            DAQmx_Val_Diff => Ok(Self::Differential),
            DAQmx_Val_PseudoDiff => Ok(Self::PseudoDifferential),
            _ => Err(DaqmxError::UnexpectedValue(
                "AnalogTerminalConfig".to_string(),
                value,
            )),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum VoltageScale {
    Volts,
    /// A custom scale is in use. If we have not determined the name yet then this contains `None`.
    /// If we have determined the name, it will be contained in the option.
    CustomScale(Option<CString>),
    /// Units are set from the TEDS configuration. This cas should be read only.
    FromTEDS,
}

impl From<VoltageScale> for i32 {
    fn from(scale: VoltageScale) -> Self {
        match scale {
            VoltageScale::Volts => PreScaledUnits::Volts as i32,
            VoltageScale::CustomScale(_) => DAQmx_Val_FromCustomScale,
            VoltageScale::FromTEDS => PreScaledUnits::FromTEDS as i32,
        }
    }
}

///For the scale name.
impl From<VoltageScale> for CString {
    fn from(scale: VoltageScale) -> Self {
        // review: should this actually error if not custom.
        match scale {
            VoltageScale::CustomScale(Some(name)) => name.clone(),
            _ => CString::default(),
        }
    }
}

impl TryFrom<i32> for VoltageScale {
    type Error = DaqmxError;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        //The if statements look wierd but seemed like the best way for the type conversion to be combined.
        match value {
            DAQmx_Val_Volts => Ok(Self::Volts),
            DAQmx_Val_FromCustomScale => Ok(Self::CustomScale(None)),
            DAQmx_Val_FromTEDS => Ok(Self::FromTEDS),
            _ => Err(DaqmxError::UnexpectedValue(
                "AnalogTerminalConfig".to_string(),
                value,
            )),
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
