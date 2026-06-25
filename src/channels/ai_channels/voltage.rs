use crate::channels::properties::{ChannelName, PropertyValue};
use crate::channels::{
    AnalogInputKind, ChannelBuilder, ChannelKind,
    TaskChannel,
};
use crate::daqmx_call;
use crate::error::DaqmxError;
use crate::scales::PreScaledUnits;
use ni_daqmx_sys::{
    DAQmx_Val_FromCustomScale, DAQmx_Val_FromTEDS, DAQmx_Val_Volts, DAQmxGetAIVoltageUnits,
    DAQmxSetAIVoltageUnits, TaskHandle,
};
use std::ffi::CString;
use std::sync::Arc;
use crate::channels::ai_channels::{AnalogChannelBuilder, AnalogTerminalConfig};

pub struct Voltage;
impl ChannelKind for Voltage {}

impl AnalogInputKind for Voltage {}

impl TaskChannel<Voltage> {
    pub fn scale(&self) -> Result<VoltageScale, DaqmxError> {
        let scale: VoltageScale = self.property_get(DAQmxGetAIVoltageUnits)?;

        if let VoltageScale::CustomScale(_) = scale {
            let name = self.custom_scale_name()?;
            Ok(VoltageScale::CustomScale(Some(Arc::new(CString::new(
                name,
            )?))))
        } else {
            Ok(scale)
        }
    }

    pub fn set_scale(&mut self, scale: VoltageScale) -> Result<(), DaqmxError> {
        if let VoltageScale::CustomScale(Some(name)) = &scale {
            self.set_custom_scale_name(name)?;
        }
        self.property_set(DAQmxSetAIVoltageUnits, scale)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum VoltageScale {
    Volts,
    /// A custom scale is in use. If we have not determined the name yet then this contains `None`.
    /// If we have determined the name, it will be contained in the option.
    CustomScale(Option<Arc<CString>>),
    /// Units are set from the TEDS configuration. This cas should be read only.
    FromTEDS,
}

impl VoltageScale {
    pub fn new_custom(name: &str) -> Result<Self, DaqmxError> {
        Ok(Self::CustomScale(Some(Arc::new(CString::new(name)?))))
    }

    pub fn new_custom_cstr(name: CString) -> Self {
        Self::CustomScale(Some(Arc::new(name)))
    }
}

impl PropertyValue for VoltageScale {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_Volts => Ok(Self::Volts),
            DAQmx_Val_FromCustomScale => Ok(Self::CustomScale(None)),
            DAQmx_Val_FromTEDS => Ok(Self::FromTEDS),
            _ => Err(DaqmxError::UnexpectedValue("Voltage Scale", raw)),
        }
    }
    fn into_raw(self) -> Self::Raw {
        match self {
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
            VoltageScale::CustomScale(Some(name)) => name.as_ref().clone(),
            _ => CString::default(),
        }
    }
}

pub struct VoltageChannelBuilder {
    physical_channel: CString,
    name: ChannelName,
    max: f64,
    min: f64,
    scale: VoltageScale,
    terminal_config: AnalogTerminalConfig,
}

impl ChannelBuilder for VoltageChannelBuilder {
    type Kind = Voltage;
    fn new<S: Into<Vec<u8>>>(physical_channel: S) -> crate::error::Result<Self> {
        Ok(Self {
            physical_channel: CString::new(physical_channel)?,
            name: ChannelName::default(),
            max: 5.0,
            min: -5.0,
            scale: VoltageScale::Volts,
            terminal_config: AnalogTerminalConfig::Default,
        })
    }

    fn name<S: Into<Vec<u8>>>(mut self, name: S) -> crate::error::Result<Self> {
        self.name.set(name)?;
        Ok(self)
    }

    fn add_to_task(self, task: TaskHandle) -> crate::error::Result<TaskChannel<Self::Kind>> {
        let expected_name = self.name.or(&self.physical_channel).to_owned();
        daqmx_call!(ni_daqmx_sys::DAQmxCreateAIVoltageChan(
            task,
            self.physical_channel.as_ptr(),
            self.name.as_ptr(),
            self.terminal_config as i32,
            self.min,
            self.max,
            self.scale.clone().into_raw(),
            CString::from(self.scale).as_ptr(),
        ))?;
        Ok(TaskChannel::new(task, expected_name))
    }
}

impl AnalogChannelBuilder for VoltageChannelBuilder {
    fn max(self, max: f64) -> Self {
        Self { max, ..self }
    }

    fn min(self, min: f64) -> Self {
        Self { min, ..self }
    }
}

impl VoltageChannelBuilder {
    pub fn scale(self, scale: VoltageScale) -> Self {
        Self { scale, ..self }
    }

    pub fn terminal_config(self, terminal_config: AnalogTerminalConfig) -> Self {
        Self {
            terminal_config,
            ..self
        }
    }
}
