//! Implements custom scaling functions
//!

use std::ffi::CString;

use ni_daqmx_sys::*;

use crate::{
    daqmx_call,
    error::{DaqmxError, Result},
};

/// The custom scale type encapsulates common custom scale functions used by all scale types.
///
/// It is intended to be used as an inner type for more specific types.
pub struct CustomScale {
    name: CString,
}

impl CustomScale {
    fn new(name: CString) -> Self {
        Self { name }
    }
}

/// A linear custom scale.
///
/// This scales the inputs according to a y = mx + c linear equation.
pub struct LinearScale {
    inner: CustomScale,
}

impl LinearScale {
    pub fn new(
        name: &str,
        slope: f64,
        y_intercept: f64,
        pre_scaled_units: PreScaledUnits,
        scaled_units: &str,
    ) -> Result<Self> {
        let name = CString::new(name)?;
        daqmx_call!(DAQmxCreateLinScale(
            name.as_ptr(),
            slope,
            y_intercept,
            pre_scaled_units as i32,
            CString::new(scaled_units)?.as_ptr()
        ))?;
        Ok(Self {
            inner: CustomScale::new(name),
        })
    }
}

/// Represents the different scaled units provided by DAQmx Channel types.
#[repr(i32)]
#[derive(PartialEq, Debug, Clone, Eq)]
pub enum PreScaledUnits {
    Volts = DAQmx_Val_Volts,
    Amps = DAQmx_Val_Amps,
    DegreesFarenheit = DAQmx_Val_DegF,
    DegreesCelcius = DAQmx_Val_DegC,
    DegreesRankine = DAQmx_Val_DegR,
    Kelvin = DAQmx_Val_Kelvins,
    Strain = DAQmx_Val_Strain,
    Ohms = DAQmx_Val_Ohms,
    Hertz = DAQmx_Val_Hz,
    Seconds = DAQmx_Val_Seconds,
    Meters = DAQmx_Val_Meters,
    Inches = DAQmx_Val_Inches,
    Degrees = DAQmx_Val_Degrees,
    Radians = DAQmx_Val_Radians,
    G = DAQmx_Val_g,
    MetersPerSecondSquared = DAQmx_Val_MetersPerSecondSquared,
    Newtons = DAQmx_Val_Newtons,
    Pounds = DAQmx_Val_Pounds,
    PSI = DAQmx_Val_PoundsPerSquareInch,
    Bar = DAQmx_Val_Bar,
    Pascals = DAQmx_Val_Pascals,
    VoltsPerVolt = DAQmx_Val_VoltsPerVolt,
    MilliVoltsPerVolt = DAQmx_Val_mVoltsPerVolt,
    NewtonMeters = DAQmx_Val_NewtonMeters,
    InchOunces = DAQmx_Val_InchOunces,
    PoundInches = DAQmx_Val_InchPounds,
    PoundFeet = DAQmx_Val_FootPounds,
    FromTEDS = DAQmx_Val_FromTEDS,
}

impl TryFrom<i32> for PreScaledUnits {
    type Error = DaqmxError;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match  value {
            DAQmx_Val_Volts => Ok(PreScaledUnits::Volts),
            DAQmx_Val_Amps => Ok(PreScaledUnits::Amps),
            DAQmx_Val_DegF => Ok(PreScaledUnits::DegreesFarenheit),
            DAQmx_Val_DegC => Ok(PreScaledUnits::DegreesCelcius),
            DAQmx_Val_DegR => Ok(PreScaledUnits::DegreesRankine),
            DAQmx_Val_Kelvins => Ok(PreScaledUnits::Kelvin),
            DAQmx_Val_Strain => Ok(PreScaledUnits::Strain),
            DAQmx_Val_Ohms => Ok(PreScaledUnits::Ohms),
            DAQmx_Val_Hz => Ok(PreScaledUnits::Hertz),
            DAQmx_Val_Seconds => Ok(PreScaledUnits::Seconds),
            DAQmx_Val_Meters => Ok(PreScaledUnits::Meters),
            DAQmx_Val_Inches => Ok(PreScaledUnits::Inches),
            DAQmx_Val_Degrees => Ok(PreScaledUnits::Degrees),
            DAQmx_Val_Radians => Ok(PreScaledUnits::Radians),
            DAQmx_Val_g => Ok(PreScaledUnits::G),
            DAQmx_Val_MetersPerSecondSquared => Ok(PreScaledUnits::MetersPerSecondSquared),
            DAQmx_Val_Newtons => Ok(PreScaledUnits::Newtons),
            DAQmx_Val_Pounds => Ok(PreScaledUnits::Pounds),
            DAQmx_Val_PoundsPerSquareInch => Ok(PreScaledUnits::PSI),
            DAQmx_Val_Bar => Ok(PreScaledUnits::Bar),
            DAQmx_Val_Pascals => Ok(PreScaledUnits::Pascals),
            DAQmx_Val_VoltsPerVolt => Ok(PreScaledUnits::VoltsPerVolt),
            DAQmx_Val_mVoltsPerVolt => Ok(PreScaledUnits::MilliVoltsPerVolt),
            DAQmx_Val_NewtonMeters => Ok(PreScaledUnits::NewtonMeters),
            DAQmx_Val_InchOunces => Ok(PreScaledUnits::InchOunces),
            DAQmx_Val_InchPounds => Ok(PreScaledUnits::PoundInches),
            DAQmx_Val_FootPounds => Ok(PreScaledUnits::PoundFeet),
            DAQmx_Val_FromTEDS => Ok(PreScaledUnits::FromTEDS),
            _ => Err(DaqmxError::UnexpectedValue(
                "DAQmx Units".to_string(),
                value,
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unit_from_i32() {
        assert_eq!(
            PreScaledUnits::Volts,
            PreScaledUnits::try_from(DAQmx_Val_Volts).unwrap()
        );
    }

    #[test]
    fn test_unit_to_i32() {
        assert_eq!(PreScaledUnits::Volts as i32, DAQmx_Val_Volts);
    }

    #[test]
    fn test_error_invalid_unit() {
        assert!(PreScaledUnits::try_from(0).is_err())
    }
}
