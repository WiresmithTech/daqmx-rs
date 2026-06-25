use crate::channels::properties::{ChannelName, PropertyValue};
use crate::channels::{
    AnalogInputKind, ChannelBuilder, ChannelKind,
    TaskChannel,
};
use crate::daqmx_call;
use crate::error::DaqmxError;
use ni_daqmx_sys::*;
use std::ffi::CString;
use std::str::FromStr;
use std::sync::Arc;
use crate::channels::ai_channels::{AnalogChannelBuilder, AnalogTerminalConfig};

pub struct CurrentChannel;

impl ChannelKind for CurrentChannel {}
impl AnalogInputKind for CurrentChannel {}

impl TaskChannel<CurrentChannel> {
    pub fn scale(&self) -> Result<CurrentScale, DaqmxError> {
        let mut initial: CurrentScale = self.property_get(DAQmxGetAICurrentUnits)?;

        if let CurrentScale::CustomScale(None) = initial {
            let name = self.custom_scale_name()?;
            let name_cstr = CString::from_str(&name)?;
            initial = CurrentScale::CustomScale(Some(Arc::new(name_cstr)));
        }
        Ok(initial)
    }

    pub fn set_scale(&self, scale: CurrentScale) -> Result<(), DaqmxError> {
        if let CurrentScale::CustomScale(Some(name)) = &scale {
            self.set_custom_scale_name(name)?;
        }
        self.property_set(DAQmxSetAICurrentUnits, scale.clone().into_raw())?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CurrentScale {
    Amps,
    /// A custom scale is in use. If we have not determined the name yet then this contains `None`.
    /// If we have determined the name, it will be contained in the option.
    // Arc for cheap clone so this can act like other parameters.
    CustomScale(Option<Arc<CString>>),
    /// Units are set from the TEDS configuration. This cas should be read only.
    FromTEDS,
}

impl CurrentScale {
    pub fn new_custom(name: &str) -> Result<Self, DaqmxError> {
        Ok(Self::CustomScale(Some(Arc::new(CString::new(name)?))))
    }

    pub fn new_custom_cstr(name: CString) -> Self {
        Self::CustomScale(Some(Arc::new(name)))
    }
}

impl PropertyValue for CurrentScale {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_Amps => Ok(CurrentScale::Amps),
            DAQmx_Val_FromCustomScale => Ok(CurrentScale::CustomScale(None)),
            DAQmx_Val_FromTEDS => Ok(CurrentScale::FromTEDS),
            _ => Err(DaqmxError::UnexpectedValue("Current Scale", raw)),
        }
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            CurrentScale::Amps => DAQmx_Val_Amps,
            CurrentScale::CustomScale(_) => DAQmx_Val_FromCustomScale,
            CurrentScale::FromTEDS => DAQmx_Val_FromTEDS,
        }
    }
}

pub enum ShuntResistorLocation {
    Default,
    Internal,
    /// External with a value in ohms.
    External(f64),
}

impl PropertyValue for ShuntResistorLocation {
    type Raw = i32;

    fn into_raw(self) -> Self::Raw {
        match self {
            ShuntResistorLocation::Default => DAQmx_Val_Default,
            ShuntResistorLocation::Internal => DAQmx_Val_Internal,
            ShuntResistorLocation::External(_) => DAQmx_Val_External,
        }
    }

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_Default => Ok(ShuntResistorLocation::Default),
            DAQmx_Val_Internal => Ok(ShuntResistorLocation::Internal),
            DAQmx_Val_External => Ok(ShuntResistorLocation::External(0.0)),
            _ => Err(DaqmxError::UnexpectedValue("Shunt Resistor Location", raw)),
        }
    }
}

pub struct CurrentChannelBuilder {
    physical_channel: CString,
    name: ChannelName,
    min: f64,
    max: f64,
    scale: CurrentScale,
    shunt_resistor_location: ShuntResistorLocation,
    terminal_config: AnalogTerminalConfig,
}

impl ChannelBuilder for CurrentChannelBuilder {
    type Kind = CurrentChannel;

    fn new<S: Into<Vec<u8>>>(physical_channel: S) -> crate::error::Result<Self> {
        Ok(Self {
            physical_channel: CString::new(physical_channel.into())?,
            name: ChannelName::default(),
            min: -0.01,
            max: 0.01,
            scale: CurrentScale::Amps,
            shunt_resistor_location: ShuntResistorLocation::Default,
            terminal_config: AnalogTerminalConfig::Default,
        })
    }

    fn name<S: Into<Vec<u8>>>(mut self, name: S) -> crate::error::Result<Self> {
        self.name.set(name)?;
        Ok(self)
    }

    fn add_to_task(self, task: TaskHandle) -> crate::error::Result<TaskChannel<Self::Kind>> {
        let shunt_value = match self.shunt_resistor_location {
            ShuntResistorLocation::External(value) => value,
            _ => 249.0,
        };
        let custom_scale_name = match &self.scale {
            CurrentScale::CustomScale(Some(name)) => name.as_ptr(),
            _ => std::ptr::null(),
        };
        let expected_name = self.name.or(&self.physical_channel).to_owned();
        daqmx_call!(DAQmxCreateAICurrentChan(
            task,
            self.physical_channel.as_ptr(),
            self.name.as_ptr(),
            self.terminal_config.into_raw(),
            self.min,
            self.max,
            self.scale.into_raw(),
            self.shunt_resistor_location.into_raw(),
            shunt_value,
            custom_scale_name
        ))?;

        Ok(TaskChannel::new(task, expected_name))
    }
}

impl AnalogChannelBuilder for CurrentChannelBuilder {
    fn max(self, max: f64) -> Self {
        Self { max, ..self }
    }

    fn min(self, min: f64) -> Self {
        Self { min, ..self }
    }
}

impl CurrentChannelBuilder {
    pub fn terminal_config(self, terminal_config: AnalogTerminalConfig) -> Self {
        Self {
            terminal_config,
            ..self
        }
    }

    pub fn scale(self, scale: CurrentScale) -> Self {
        Self { scale, ..self }
    }

    pub fn shunt_resistor_location(self, shunt_resistor_location: ShuntResistorLocation) -> Self {
        Self {
            shunt_resistor_location,
            ..self
        }
    }
}
