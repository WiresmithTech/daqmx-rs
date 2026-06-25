use crate::channels::properties::PropertyValue;
use crate::error::DaqmxError;
use ni_daqmx_sys::*;

/// The excitation source for a resistance channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExcitationSource {
    Internal,
    External,
    None,
}

impl PropertyValue for ExcitationSource {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_Internal => Ok(ExcitationSource::Internal),
            DAQmx_Val_External => Ok(ExcitationSource::External),
            DAQmx_Val_None => Ok(ExcitationSource::None),
            _ => Err(DaqmxError::UnexpectedValue("ExcitationSource", raw)),
        }
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            ExcitationSource::Internal => DAQmx_Val_Internal,
            ExcitationSource::External => DAQmx_Val_External,
            ExcitationSource::None => DAQmx_Val_None,
        }
    }
}

/// The wire configuration for a resistance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireConfiguration {
    TwoWire,
    ThreeWire,
    FourWire,
}

impl PropertyValue for WireConfiguration {
    type Raw = i32;

    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError> {
        #[allow(non_upper_case_globals)]
        match raw {
            DAQmx_Val_2Wire => Ok(WireConfiguration::TwoWire),
            DAQmx_Val_3Wire => Ok(WireConfiguration::ThreeWire),
            DAQmx_Val_4Wire => Ok(WireConfiguration::FourWire),
            _ => Err(DaqmxError::UnexpectedValue("WireConfiguration", raw)),
        }
    }

    fn into_raw(self) -> Self::Raw {
        match self {
            WireConfiguration::TwoWire => DAQmx_Val_2Wire,
            WireConfiguration::ThreeWire => DAQmx_Val_3Wire,
            WireConfiguration::FourWire => DAQmx_Val_4Wire,
        }
    }
}
