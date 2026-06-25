use crate::channels::ai_channels::temperature::{TemperatureInputKind, TemperatureUnits};
use crate::channels::properties::{ChannelName, PropertyValue};
use crate::channels::{
    AnalogInputKind, ChannelBuilder, ChannelKind, TaskChannel, property,
};
use crate::daqmx_call;
use crate::error::DaqmxError;
use ni_daqmx_sys::*;
use std::ffi::CString;
use crate::channels::ai_channels::AnalogChannelBuilder;
use crate::channels::ai_channels::resistance::{ExcitationSource, WireConfiguration};

pub struct Rtd {}

impl ChannelKind for Rtd {}
impl AnalogInputKind for Rtd {}
impl TemperatureInputKind for Rtd {}

impl TaskChannel<Rtd> {
    property!(get_set_reset r0 / set_r0 / reset_r0: f64 = DAQmxGetAIRTDR0, DAQmxSetAIRTDR0, DAQmxResetAIRTDR0);

    pub fn get_type(&self) -> Result<RTDType, DaqmxError> {
        let daqmx_type: DaqmxRTDType = self.property_get(DAQmxGetAIRTDType)?;
        match daqmx_type {
            DaqmxRTDType::Pt3750 => Ok(RTDType::Pt3750),
            DaqmxRTDType::Pt3851 => Ok(RTDType::Pt3851),
            DaqmxRTDType::Pt3911 => Ok(RTDType::Pt3911),
            DaqmxRTDType::Pt3916 => Ok(RTDType::Pt3916),
            DaqmxRTDType::Pt3920 => Ok(RTDType::Pt3920),
            DaqmxRTDType::PT3928 => Ok(RTDType::PT3928),
            DaqmxRTDType::Custom => {
                let a = self.property_get(DAQmxGetAIRTDA)?;
                let b = self.property_get(DAQmxGetAIRTDB)?;
                let c = self.property_get(DAQmxGetAIRTDC)?;
                Ok(RTDType::Custom { a, b, c })
            }
        }
    }

    pub fn set_type(&mut self, rtd_type: RTDType) -> Result<(), DaqmxError> {
        let daqmx_type: DaqmxRTDType = (&rtd_type).into();
        self.property_set(DAQmxSetAIRTDType, daqmx_type)?;
        if let RTDType::Custom { a, b, c } = rtd_type {
                self.property_set(DAQmxSetAIRTDA, a)?;
                self.property_set(DAQmxSetAIRTDB, b)?;
                self.property_set(DAQmxSetAIRTDC, c)?;
        }
        Ok(())
    }

    pub fn reset_type(&mut self) -> Result<(), DaqmxError> {
        self.property_reset(DAQmxResetAIRTDType)?;
        self.property_reset(DAQmxResetAIRTDA)?;
        self.property_reset(DAQmxResetAIRTDB)?;
        self.property_reset(DAQmxResetAIRTDC)?;
        Ok(())
    }
}

/// The possible values of the DAQmx enum for the RTD type.
enum DaqmxRTDType {
    Pt3750,
    Pt3851,
    Pt3911,
    Pt3916,
    Pt3920,
    PT3928,
    Custom,
}

impl PropertyValue for DaqmxRTDType {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_Pt3750 => Ok(DaqmxRTDType::Pt3750),
            DAQmx_Val_Pt3851 => Ok(DaqmxRTDType::Pt3851),
            DAQmx_Val_Pt3911 => Ok(DaqmxRTDType::Pt3911),
            DAQmx_Val_Pt3916 => Ok(DaqmxRTDType::Pt3916),
            DAQmx_Val_Pt3920 => Ok(DaqmxRTDType::Pt3920),
            DAQmx_Val_Pt3928 => Ok(DaqmxRTDType::PT3928),
            DAQmx_Val_Custom => Ok(DaqmxRTDType::Custom),
            _ => Err(DaqmxError::UnexpectedValue("RTD type", raw)),
        }
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            DaqmxRTDType::Pt3750 => DAQmx_Val_Pt3750,
            DaqmxRTDType::Pt3851 => DAQmx_Val_Pt3851,
            DaqmxRTDType::Pt3911 => DAQmx_Val_Pt3911,
            DaqmxRTDType::Pt3916 => DAQmx_Val_Pt3916,
            DaqmxRTDType::Pt3920 => DAQmx_Val_Pt3920,
            DaqmxRTDType::PT3928 => DAQmx_Val_Pt3928,
            DaqmxRTDType::Custom => DAQmx_Val_Custom,
        }
    }
}

/// The types of RTD we can set.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RTDType {
    Pt3750,
    Pt3851,
    Pt3911,
    Pt3916,
    Pt3920,
    PT3928,
    /// A custom RTD type where you specify the coefficients for the Callendar-Van Dusen equation.
    Custom {
        a: f64,
        b: f64,
        c: f64,
    },
}

impl From<&RTDType> for DaqmxRTDType {
    fn from(rtd_type: &RTDType) -> Self {
        match rtd_type {
            RTDType::Pt3750 => DaqmxRTDType::Pt3750,
            RTDType::Pt3851 => DaqmxRTDType::Pt3851,
            RTDType::Pt3911 => DaqmxRTDType::Pt3911,
            RTDType::Pt3916 => DaqmxRTDType::Pt3916,
            RTDType::Pt3920 => DaqmxRTDType::Pt3920,
            RTDType::PT3928 => DaqmxRTDType::PT3928,
            RTDType::Custom { .. } => DaqmxRTDType::Custom,
        }
    }
}

pub struct RTDBuilder {
    physical_channel: CString,
    channel_name: ChannelName,
    max: f64,
    min: f64,
    units: TemperatureUnits,
    rtd_type: RTDType,
    current_source: ExcitationSource,
    current_value: f64,
    r0: f64,
    wire_configuration: WireConfiguration,
}

impl ChannelBuilder for RTDBuilder {
    type Kind = Rtd;

    fn new<S: Into<Vec<u8>>>(physical_channel: S) -> crate::error::Result<Self> {
        Ok(Self {
            physical_channel: CString::new(physical_channel.into())?,
            channel_name: ChannelName::default(),
            max: 100.0,
            min: 0.0,
            units: TemperatureUnits::Celsius,
            rtd_type: RTDType::Pt3750,
            current_source: ExcitationSource::External,
            current_value: 0.0025,
            r0: 100.0,
            wire_configuration: WireConfiguration::TwoWire,
        })
    }

    fn name<S: Into<Vec<u8>>>(mut self, name: S) -> crate::error::Result<Self> {
        self.channel_name.set(name)?;
        Ok(self)
    }

    fn add_to_task(self, task: TaskHandle) -> crate::error::Result<TaskChannel<Rtd>> {
        let expected_name = self.channel_name.or(&self.physical_channel).to_owned();
        daqmx_call!(DAQmxCreateAIRTDChan(
            task,
            self.physical_channel.as_ptr(),
            self.channel_name.as_ptr(),
            self.min,
            self.max,
            self.units.into_raw(),
            DaqmxRTDType::from(&self.rtd_type).into_raw(),
            self.wire_configuration.into_raw(),
            self.current_source.into_raw(),
            self.current_value,
            self.r0
        ))?;
        let mut channel = TaskChannel::new(task, expected_name);
        // It isn't clear that the create channel can handle the custom type so apply again here.
        channel.set_type(self.rtd_type)?;
        Ok(channel)
    }
}

impl AnalogChannelBuilder for RTDBuilder {
    fn max(self, max: f64) -> Self {
        Self { max, ..self }
    }

    fn min(self, min: f64) -> Self {
        Self { min, ..self }
    }
}

impl RTDBuilder {
    pub fn units(self, units: TemperatureUnits) -> Self {
        Self { units, ..self }
    }

    pub fn rtd_type(self, rtd_type: RTDType) -> Self {
        Self { rtd_type, ..self }
    }

    pub fn wire_configuration(self, wire_configuration: WireConfiguration) -> Self {
        Self {
            wire_configuration,
            ..self
        }
    }

    pub fn excitation_source(self, excitation_source: ExcitationSource) -> Self {
        Self {
            current_source: excitation_source,
            ..self
        }
    }

    pub fn excitation_current(self, excitation_current: f64) -> Self {
        Self {
            current_value: excitation_current,
            ..self
        }
    }

    pub fn r0(self, r0: f64) -> Self {
        Self { r0, ..self }
    }
}
