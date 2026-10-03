pub mod current;
pub mod resistance;
pub mod temperature;
pub mod voltage;

use super::{ChannelBuilder, ChannelKind, TaskChannel, property};
use crate::error::{DaqmxError, Result};
use crate::properties::PropertyValue;
use ni_daqmx_sys::*;

pub trait AnalogInputKind: ChannelKind {}

impl<K: AnalogInputKind> TaskChannel<K> {
    property!(get_set ai_max / set_ai_max: f64 = DAQmxGetAIMax, DAQmxSetAIMax);
    property!(get_set ai_min / set_ai_min: f64 = DAQmxGetAIMin, DAQmxSetAIMin);
    property!(get_set terminal_config / set_terminal_config:
              AnalogTerminalConfig = DAQmxGetAITermCfg, DAQmxSetAITermCfg);
    property!(get_set_reset coupling / set_coupling / reset_coupling : AnalogCoupling = DAQmxGetAICoupling, DAQmxSetAICoupling, DAQmxResetAICoupling);

    // Advanced ADC Section
    property!(get_set_reset adc_timing_mode / set_adc_timing_mode / reset_adc_timing_mode : AdcTimingMode = DAQmxGetAIADCTimingMode, DAQmxSetAIADCTimingMode, DAQmxResetAIADCTimingMode);
    property!(get_set_reset adc_custom_timing_mode / set_adc_custom_timing_mode / reset_adc_custom_timing_mode : u32 = DAQmxGetAIADCCustomTimingMode, DAQmxSetAIADCCustomTimingMode, DAQmxResetAIADCCustomTimingMode);
    property!(get resolution: f64 = DAQmxGetAIResolution);
    property!(get resolution_units: AIResolutionUnits = DAQmxGetAIResolutionUnits);
    property!(get raw_sample_size: u32 = DAQmxGetAIRawSampSize);
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

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Specifies the coupling for the channel.
pub enum AnalogCoupling {
    /// Remove the DC offset from the signal.
    AC = DAQmx_Val_AC,
    /// Allow DAQmx to read all the signal.
    DC = DAQmx_Val_DC,
    /// Remove the signal from the measurement and only measure ground.
    GND = DAQmx_Val_GND,
}

impl PropertyValue for AnalogCoupling {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> std::result::Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_AC => Ok(Self::AC),
            DAQmx_Val_DC => Ok(Self::DC),
            DAQmx_Val_GND => Ok(Self::GND),
            _ => Err(DaqmxError::UnexpectedValue("AnalogCoupling", raw)),
        }
    }

    fn into_raw(self) -> i32 {
        match self {
            AnalogCoupling::AC => DAQmx_Val_AC,
            AnalogCoupling::DC => DAQmx_Val_DC,
            AnalogCoupling::GND => DAQmx_Val_GND,
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Specifies the ADC timing mode, controlling the tradeoff between speed and effective resolution.
/// Some ADC timing modes provide increased powerline noise rejection.
/// On devices that have an AI Convert clock, this setting affects both the maximum and default values for Rate.
/// You must use the same ADC timing mode for all channels on a device,
/// but you can use different ADC timing modes for different devices in the same task.
pub enum AdcTimingMode {
    /// Uses the most appropriate supported timing mode based on the Sample Clock Rate.
    Automatic = DAQmx_Val_Automatic,
    /// Increases resolution and noise rejection while decreasing conversion rate.
    HighResolution = DAQmx_Val_HighResolution,
    /// Increases conversion rate while decreasing resolution.
    HighSpeed = DAQmx_Val_HighSpeed,
    /// Improves 50 Hz noise rejection while decreasing noise rejection at other frequencies.
    Best50HzRejection = DAQmx_Val_Best50HzRejection,
    /// Improves 60 Hz noise rejection while decreasing noise rejection at other frequencies.
    Best60HzRejection = DAQmx_Val_Best60HzRejection,
    /// Use Custom Timing Mode to specify a custom value controlling the tradeoff between speed and resolution.
    Custom = DAQmx_Val_Custom,
}

impl PropertyValue for AdcTimingMode {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> std::result::Result<Self, DaqmxError> {
        Ok(match raw {
            DAQmx_Val_Automatic => Self::Automatic,
            DAQmx_Val_HighResolution => Self::HighResolution,
            DAQmx_Val_HighSpeed => Self::HighSpeed,
            DAQmx_Val_Best50HzRejection => Self::Best50HzRejection,
            DAQmx_Val_Best60HzRejection => Self::Best60HzRejection,
            DAQmx_Val_Custom => Self::Custom,
            _ => return Err(DaqmxError::UnexpectedValue("ADC Timing Mode", raw)),
        })
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            Self::Automatic => DAQmx_Val_Automatic,
            Self::HighResolution => DAQmx_Val_HighResolution,
            Self::HighSpeed => DAQmx_Val_HighSpeed,
            Self::Best50HzRejection => DAQmx_Val_Best50HzRejection,
            Self::Best60HzRejection => DAQmx_Val_Best60HzRejection,
            Self::Custom => DAQmx_Val_Custom,
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// The units used when reading the AI resolution.
pub enum AIResolutionUnits {
    Bits = DAQmx_Val_Bits,
}

impl PropertyValue for AIResolutionUnits {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> std::result::Result<Self, DaqmxError> {
        match raw {
            DAQmx_Val_Bits => Ok(Self::Bits),
            _ => Err(DaqmxError::UnexpectedValue("AI Resolution Units", raw)),
        }
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            Self::Bits => DAQmx_Val_Bits,
        }
    }
}

pub trait AnalogChannelBuilder: ChannelBuilder {
    fn max(self, max: f64) -> Self;
    fn min(self, min: f64) -> Self;
}

/// A macro to add the custom scale options to an ao channel.
///
/// This is needed to avoid "diamond" dependencies in the type system.
#[macro_export]
macro_rules! ai_custom_scale {
    ($channel_kind:ty) => {
        use crate::channels::scales::CustomScaledChannel;
        use ni_daqmx_sys::{TaskHandle, int32, uInt32};
        use std::ffi::c_char;
        impl CustomScaledChannel for $channel_kind {
            const GET_NAME: unsafe extern "C" fn(
                TaskHandle,
                *const c_char,
                *mut c_char,
                uInt32,
            ) -> int32 = ni_daqmx_sys::DAQmxGetAICustomScaleName;
            const SET_NAME: unsafe extern "C" fn(
                TaskHandle,
                *const c_char,
                *const c_char,
            ) -> int32 = ni_daqmx_sys::DAQmxSetAICustomScaleName;
            const RESET_NAME: unsafe extern "C" fn(TaskHandle, *const c_char) -> int32 =
                ni_daqmx_sys::DAQmxResetAICustomScaleName;
        }
    };
}
