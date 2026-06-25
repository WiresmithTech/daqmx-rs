use std::ffi::CString;
use ni_daqmx_sys::{DAQmxGetAIVoltageUnits, DAQmx_Val_FromCustomScale, DAQmx_Val_FromTEDS, DAQmx_Val_Volts, TaskHandle};
use crate::channels::{AnalogInputChannelBuilder, AnalogInputKind, AnalogTerminalConfig, ChannelBuilder, TaskChannel};
use crate::channels::properties::PropertyValue;
use crate::daqmx_call;
use crate::error::DaqmxError;
use crate::scales::PreScaledUnits;

pub struct Voltage;
impl AnalogInputKind for Voltage {}

impl TaskChannel<Voltage> {
    pub fn scale(&self) -> crate::error::Result<VoltageScale> {
        let scale: VoltageScale = self
            .property_get(DAQmxGetAIVoltageUnits)?;

        if let VoltageScale::CustomScale(_) = scale {
            let name = self.custom_scale_name()?;
            Ok(VoltageScale::CustomScale(Some(CString::new(name)?)))
        } else {
            Ok(scale)
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

impl PropertyValue for VoltageScale {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        match raw {
            DAQmx_Val_Volts => Ok(Self::Volts),
            DAQmx_Val_FromCustomScale => Ok(Self::CustomScale(None)),
            DAQmx_Val_FromTEDS => Ok(Self::FromTEDS),
            _ => Err(DaqmxError::UnexpectedValue(
                "Voltage Scale".to_string(),
                raw,
            )),
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
            VoltageScale::CustomScale(Some(name)) => name.clone(),
            _ => CString::default(),
        }
    }
}

pub struct VoltageChannelBuilder {
    physical_channel: CString,
    name: Option<CString>,
    pub max: f64,
    pub min: f64,
    pub scale: VoltageScale,
    pub terminal_config: AnalogTerminalConfig,
}

impl VoltageChannelBuilder {
    pub fn new<S: Into<Vec<u8>>>(physical_channel: S) -> crate::error::Result<Self> {
        Ok(Self {
            physical_channel: CString::new(physical_channel)?,
            name: None,
            max: 5.0,
            min: -5.0,
            scale: VoltageScale::Volts,
            terminal_config: AnalogTerminalConfig::Default,
        })
    }

    pub fn name<S: Into<Vec<u8>>>(&mut self, name: S) -> crate::error::Result<&mut Self> {
        self.name = Some(CString::new(name)?);
        Ok(self)
    }
}

impl ChannelBuilder for VoltageChannelBuilder {
    fn add_to_task(self, task: TaskHandle) -> crate::error::Result<()> {
        let empty_string = CString::default();
        daqmx_call!(ni_daqmx_sys::DAQmxCreateAIVoltageChan(
            task,
            self.physical_channel.as_ptr(),
            self.name.as_ref().unwrap_or(&empty_string).as_ptr(),
            self.terminal_config as i32,
            self.min,
            self.max,
            self.scale.clone().into_raw(),
            CString::from(self.scale).as_ptr(),
        ))
    }
}

impl AnalogInputChannelBuilder for VoltageChannelBuilder {}
