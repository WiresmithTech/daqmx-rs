pub mod current;
pub mod resistance;
pub mod temperature;
pub mod voltage;

use super::properties::PropertyValue;
use super::{ChannelBuilder, ChannelKind, TaskChannel, property};
use crate::error::{DaqmxError, Result};
use ni_daqmx_sys::*;
use std::ffi::CStr;

pub trait AnalogInputKind: ChannelKind {}

impl<K: AnalogInputKind> TaskChannel<K> {
    property!(get_string physical_channel = ni_daqmx_sys::DAQmxGetPhysicalChanName);
    property!(get_set ai_max / set_ai_max: f64 = DAQmxGetAIMax, DAQmxSetAIMax);
    property!(get_set ai_min / set_ai_min: f64 = DAQmxGetAIMin, DAQmxSetAIMin);
    property!(get_set terminal_config / set_terminal_config:
              AnalogTerminalConfig = DAQmxGetAITermCfg, DAQmxSetAITermCfg);
    property!(get_string custom_scale_name = ni_daqmx_sys::DAQmxGetAICustomScaleName);

    pub fn set_custom_scale_name(&self, name: &CStr) -> Result<()> {
        self.property_set_raw(DAQmxSetAICustomScaleName, name.as_ptr())
    }

    pub fn reset_custom_scale_name(&self) -> Result<()> {
        self.property_reset(DAQmxResetAICustomScaleName)
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

impl PropertyValue for AnalogTerminalConfig {
    type Raw = i32;

    fn from_raw(value: i32) -> Result<Self> {
        #[allow(non_upper_case_globals)]
        match value {
            DAQmx_Val_Cfg_Default => Ok(Self::Default),
            DAQmx_Val_RSE => Ok(Self::RSE),
            DAQmx_Val_NRSE => Ok(Self::NRSE),
            DAQmx_Val_Diff => Ok(Self::Differential),
            DAQmx_Val_PseudoDiff => Ok(Self::PseudoDifferential),
            _ => Err(DaqmxError::UnexpectedValue("AnalogTerminalConfig", value)),
        }
    }

    fn into_raw(self) -> i32 {
        match self {
            AnalogTerminalConfig::Default => DAQmx_Val_Cfg_Default,
            AnalogTerminalConfig::RSE => DAQmx_Val_RSE,
            AnalogTerminalConfig::NRSE => DAQmx_Val_NRSE,
            AnalogTerminalConfig::Differential => DAQmx_Val_Diff,
            AnalogTerminalConfig::PseudoDifferential => DAQmx_Val_PseudoDiff,
        }
    }
}

pub trait AnalogChannelBuilder: ChannelBuilder {
    fn max(self, max: f64) -> Self;
    fn min(self, min: f64) -> Self;
}
