pub mod rtd;
mod thermocouple;

use crate::channels::properties::PropertyValue;
use crate::channels::{AnalogInputKind, TaskChannel, property};
use crate::error::DaqmxError;
use ni_daqmx_sys::*;

pub trait TemperatureInputKind: AnalogInputKind {}

impl<K: TemperatureInputKind> TaskChannel<K> {
    property!(get_set_reset temperature_units / set_temperature_units / reset_temperature_units: TemperatureUnits = DAQmxGetAITempUnits, DAQmxSetAITempUnits, DAQmxResetAITempUnits);
}

pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    CustomScale,
}

impl PropertyValue for TemperatureUnits {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_DegC => Ok(TemperatureUnits::Celsius),
            DAQmx_Val_DegF => Ok(TemperatureUnits::Fahrenheit),
            DAQmx_Val_Kelvins => Ok(TemperatureUnits::Kelvin),
            DAQmx_Val_DegR => Ok(TemperatureUnits::Rankine),
            DAQmx_Val_FromCustomScale => Ok(TemperatureUnits::CustomScale),
            _ => Err(DaqmxError::UnexpectedValue("Invalid temperature unit", raw)),
        }
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            TemperatureUnits::Celsius => DAQmx_Val_DegC,
            TemperatureUnits::Fahrenheit => DAQmx_Val_DegF,
            TemperatureUnits::Kelvin => DAQmx_Val_Kelvins,
            TemperatureUnits::Rankine => DAQmx_Val_DegR,
            TemperatureUnits::CustomScale => DAQmx_Val_FromCustomScale,
        }
    }
}
