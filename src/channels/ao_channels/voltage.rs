use std::ffi::CString;
use std::sync::Arc;
use ni_daqmx_sys::{DAQmxGetAOVoltageUnits, DAQmxSetAOVoltageUnits, DAQmx_Val_FromCustomScale, DAQmx_Val_Volts};
use crate::channels::{AnalogOutputKind, ChannelBuilder, ChannelKind, TaskChannel};
use crate::channels::ai_channels::AnalogChannelBuilder;
use crate::channels::properties::{ChannelName, PropertyValue};
use crate::daqmx_call;
use crate::error::DaqmxError;
use crate::scales::PreScaledUnits;
use crate::ao_custom_scale;

pub struct Voltage;

impl ChannelKind for Voltage {}
impl AnalogOutputKind for Voltage {}
pub struct VoltageOutputChannelBuilder {
    physical_channel: CString,
    name: ChannelName,
    max: f64,
    min: f64,
    scale: VoltageOutputScale,
}

impl ChannelBuilder for VoltageOutputChannelBuilder {
    type Kind = Voltage;
    fn new<S: Into<Vec<u8>>>(physical_channel: S) -> crate::error::Result<Self> {
        Ok(Self {
            physical_channel: CString::new(physical_channel)?,
            name: ChannelName::default(),
            max: 5.0,
            min: -5.0,
            scale: VoltageOutputScale::Volts,
        })
    }

    fn name<S: Into<Vec<u8>>>(mut self, name: S) -> crate::error::Result<Self> {
        self.name.set(name)?;
        Ok(self)
    }

    fn add_to_task(self, task: TaskHandle) -> crate::error::Result<TaskChannel<Self::Kind>> {
        let expected_name = self.name.or(&self.physical_channel).to_owned();
        daqmx_call!(ni_daqmx_sys::DAQmxCreateAOVoltageChan(
            task,
            self.physical_channel.as_ptr(),
            self.name.as_ptr(),
            self.min,
            self.max,
            self.scale.clone().into_raw(),
            CString::from(self.scale).as_ptr(),
        ))?;
        Ok(TaskChannel::new(task, expected_name))
    }
}

/// The scale settings available for an input voltage.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum VoltageOutputScale {
    Volts,
    /// A custom scale is in use. If we have not determined the name yet then this contains `None`.
    /// If we have determined the name, it will be contained in the option.
    CustomScale(Option<Arc<CString>>),
}

impl VoltageOutputScale {
    pub fn new_custom(name: &str) -> Result<Self, DaqmxError> {
        Ok(Self::CustomScale(Some(Arc::new(CString::new(name)?))))
    }

    pub fn new_custom_cstr(name: CString) -> Self {
        Self::CustomScale(Some(Arc::new(name)))
    }
}

impl PropertyValue for VoltageOutputScale {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_Volts => Ok(Self::Volts),
            DAQmx_Val_FromCustomScale => Ok(Self::CustomScale(None)),
            _ => Err(DaqmxError::UnexpectedValue("Voltage Output Scale", raw)),
        }
    }
    fn into_raw(self) -> Self::Raw {
        match self {
            VoltageOutputScale::Volts => PreScaledUnits::Volts as i32,
            VoltageOutputScale::CustomScale(_) => DAQmx_Val_FromCustomScale,
        }
    }
}

///For the scale name.
impl From<VoltageOutputScale> for CString {
    fn from(scale: VoltageOutputScale) -> Self {
        // review: should this actually error if not custom.
        match scale {
            VoltageOutputScale::CustomScale(Some(name)) => name.as_ref().clone(),
            _ => CString::default(),
        }
    }
}

impl AnalogChannelBuilder for VoltageOutputChannelBuilder {
    fn max(self, max: f64) -> Self {
        Self { max, ..self }
    }

    fn min(self, min: f64) -> Self {
        Self { min, ..self }
    }
}

impl VoltageOutputChannelBuilder {
    pub fn scale(self, scale: VoltageOutputScale) -> Self {
        Self { scale, ..self }
    }

}
impl TaskChannel<Voltage> {
    pub fn scale(&self) -> Result<VoltageOutputScale, DaqmxError> {
        let scale: VoltageOutputScale = self.property_get(DAQmxGetAOVoltageUnits)?;

        if let VoltageOutputScale::CustomScale(_) = scale {
            let name = self.custom_scale_name()?;
            Ok(VoltageOutputScale::CustomScale(Some(Arc::new(CString::new(
                name,
            )?))))
        } else {
            Ok(scale)
        }
    }

    pub fn set_scale(&mut self, scale: VoltageOutputScale) -> Result<(), DaqmxError> {
        if let VoltageOutputScale::CustomScale(Some(name)) = &scale {
            self.set_custom_scale_name(name)?;
        }
        self.property_set(DAQmxSetAOVoltageUnits, scale)
    }
}

ao_custom_scale!(Voltage);
