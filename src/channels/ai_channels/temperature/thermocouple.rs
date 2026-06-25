use crate::channels::ai_channels::temperature::{TemperatureInputKind, TemperatureUnits};
use crate::channels::properties::{ChannelName, PropertyValue};
use crate::channels::{AnalogInputKind, ChannelBuilder, ChannelKind, TaskChannel, property};
use crate::daqmx_call;
use crate::error::DaqmxError;
use ni_daqmx_sys::*;
use std::ffi::CString;

pub struct Thermocouple;

impl ChannelKind for Thermocouple {}
impl AnalogInputKind for Thermocouple {}
impl TemperatureInputKind for Thermocouple {}

impl TaskChannel<Thermocouple> {
    property!(get_string cjc_channel = DAQmxGetAIThrmcplCJCChan);
    property!(get cjc_source: DaqmxCjcSource = DAQmxGetAIThrmcplCJCSrc);
    property!(get_set_reset  cjc_value / set_cjc_value / reset_cjc_value: f64 = DAQmxGetAIThrmcplCJCVal, DAQmxSetAIThrmcplCJCVal, DAQmxResetAIThrmcplCJCVal);
    property!(get_set_reset thermocouple_scale_type / set_thermocouple_scale_type / reset_thermocouple_scale_type: ThermocoupleScaleType = DAQmxGetAIThrmcplScaleType, DAQmxSetAIThrmcplScaleType, DAQmxResetAIThrmcplScaleType);
    property!(get_set_reset thermocouple_type / set_thermocouple_type / reset_thermocouple_type: ThermocoupleType = DAQmxGetAIThrmcplType, DAQmxSetAIThrmcplType, DAQmxResetAIThrmcplType);
}

pub enum CjcSource {
    BuiltIn,
    ConstValue(f64),
    Channel(CString),
}

impl From<&CjcSource> for DaqmxCjcSource {
    fn from(source: &CjcSource) -> Self {
        match source {
            CjcSource::BuiltIn => DaqmxCjcSource::BuiltIn,
            CjcSource::ConstValue(_) => DaqmxCjcSource::ConstValue,
            CjcSource::Channel(_) => DaqmxCjcSource::Channel,
        }
    }
}

pub enum DaqmxCjcSource {
    BuiltIn,
    ConstValue,
    Channel,
}

impl PropertyValue for DaqmxCjcSource {
    type Raw = i32;
    fn from_raw(raw: i32) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_BuiltIn => Ok(DaqmxCjcSource::BuiltIn),
            DAQmx_Val_ConstVal => Ok(DaqmxCjcSource::ConstValue),
            DAQmx_Val_Chan => Ok(DaqmxCjcSource::Channel),
            _ => Err(DaqmxError::UnexpectedValue("CJC source", raw)),
        }
    }

    fn into_raw(self) -> i32 {
        match self {
            DaqmxCjcSource::BuiltIn => DAQmx_Val_BuiltIn,
            DaqmxCjcSource::ConstValue => DAQmx_Val_ConstVal,
            DaqmxCjcSource::Channel => DAQmx_Val_Chan,
        }
    }
}

pub enum ThermocoupleScaleType {
    Polynominal,
    Table,
}

impl PropertyValue for ThermocoupleScaleType {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_Polynomial => Ok(ThermocoupleScaleType::Polynominal),
            DAQmx_Val_Table => Ok(ThermocoupleScaleType::Table),
            _ => Err(DaqmxError::UnexpectedValue("Thermocouple scale type", raw)),
        }
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            ThermocoupleScaleType::Polynominal => DAQmx_Val_Polynomial,
            ThermocoupleScaleType::Table => DAQmx_Val_Table,
        }
    }
}

pub enum ThermocoupleType {
    J,
    K,
    N,
    R,
    S,
    T,
    B,
    E,
    A,
    C,
}

impl PropertyValue for ThermocoupleType {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_J_Type_TC => Ok(ThermocoupleType::J),
            DAQmx_Val_K_Type_TC => Ok(ThermocoupleType::K),
            DAQmx_Val_N_Type_TC => Ok(ThermocoupleType::N),
            DAQmx_Val_R_Type_TC => Ok(ThermocoupleType::R),
            DAQmx_Val_S_Type_TC => Ok(ThermocoupleType::S),
            DAQmx_Val_T_Type_TC => Ok(ThermocoupleType::T),
            DAQmx_Val_B_Type_TC => Ok(ThermocoupleType::B),
            DAQmx_Val_E_Type_TC => Ok(ThermocoupleType::E),
            DAQmx_Val_A_Type_TC => Ok(ThermocoupleType::A),
            DAQmx_Val_C_Type_TC => Ok(ThermocoupleType::C),
            _ => Err(DaqmxError::UnexpectedValue("Thermocouple type", raw)),
        }
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            ThermocoupleType::J => DAQmx_Val_J_Type_TC,
            ThermocoupleType::K => DAQmx_Val_K_Type_TC,
            ThermocoupleType::N => DAQmx_Val_N_Type_TC,
            ThermocoupleType::R => DAQmx_Val_R_Type_TC,
            ThermocoupleType::S => DAQmx_Val_S_Type_TC,
            ThermocoupleType::T => DAQmx_Val_T_Type_TC,
            ThermocoupleType::B => DAQmx_Val_B_Type_TC,
            ThermocoupleType::E => DAQmx_Val_E_Type_TC,
            ThermocoupleType::A => DAQmx_Val_A_Type_TC,
            ThermocoupleType::C => DAQmx_Val_C_Type_TC,
        }
    }
}

pub struct ThermocoupleBuilder {
    physical_channel: CString,
    channel_name: ChannelName,
    min: f64,
    max: f64,
    thermocouple_type: ThermocoupleType,
    units: TemperatureUnits,
    cjc_source: CjcSource,
}

impl ChannelBuilder for ThermocoupleBuilder {
    type Kind = Thermocouple;

    fn new<S: Into<Vec<u8>>>(physical_channel: S) -> crate::error::Result<Self> {
        Ok(Self {
            physical_channel: CString::new(physical_channel)?,
            channel_name: ChannelName::default(),
            min: 0.0,
            max: 100.0,
            thermocouple_type: ThermocoupleType::J,
            units: TemperatureUnits::Celsius,
            cjc_source: CjcSource::ConstValue(25.0),
        })
    }

    fn name<S: Into<Vec<u8>>>(mut self, name: S) -> crate::error::Result<Self> {
        self.channel_name.set(name)?;
        Ok(self)
    }

    fn add_to_task(self, task: TaskHandle) -> crate::error::Result<TaskChannel<Self::Kind>> {
        let expected_name = self.channel_name.or(&self.physical_channel).to_owned();
        let source: DaqmxCjcSource = (&self.cjc_source).into();
        let (value, channel) = match &self.cjc_source {
            CjcSource::BuiltIn => (0.0, std::ptr::null()),
            CjcSource::ConstValue(value) => (*value, std::ptr::null()),
            CjcSource::Channel(channel) => (0.0, channel.as_ptr()),
        };
        daqmx_call!(DAQmxCreateAIThrmcplChan(
            task,
            self.physical_channel.as_ptr(),
            self.channel_name.as_ptr(),
            self.min,
            self.max,
            self.units.into_raw(),
            self.thermocouple_type.into_raw(),
            source.into_raw(),
            value,
            channel
        ))?;
        Ok(TaskChannel::new(task, expected_name))
    }
}
