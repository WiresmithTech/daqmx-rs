//! Implements custom scaling functions
//!

use ni_daqmx_sys::*;
use num::FromPrimitive;
use num_derive::FromPrimitive;

trait CustomScale {}

#[repr(i32)]
#[derive(FromPrimitive)]
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
    OunceInches = DAQmx_Val_InchOunces,
    PoundInches = DAQmx_Val_InchPounds,
    PoundFeet = DAQmx_Val_FootPounds,
    FromTEDS = DAQmx_Val_FromTEDS,
}
