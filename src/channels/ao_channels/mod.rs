pub mod voltage;
pub mod current;

use ni_daqmx_sys::{DAQmxGetAOMax, DAQmxGetAOMin, DAQmxSetAOMax, DAQmxSetAOMin};
use crate::channels::{property, ChannelKind, TaskChannel};

pub trait AnalogOutputKind: ChannelKind {}

impl<K: AnalogOutputKind> TaskChannel<K> {
    property!(get_set ao_max / set_ao_max: f64 = DAQmxGetAOMax, DAQmxSetAOMax);
    property!(get_set ao_min / set_ao_min: f64 = DAQmxGetAOMin, DAQmxSetAOMin);
}

/// A macro to add the custom scale options to an ao channel.
/// 
/// This is needed to avoid "diamond" dependencies in the type system.
#[macro_export]
macro_rules! ao_custom_scale {
    ($channel_kind:ty) => {
        use crate::channels::scales::CustomScaledChannel;
        use ni_daqmx_sys::{uInt32, TaskHandle, int32};
        use std::ffi::c_char;
        impl CustomScaledChannel for $channel_kind {
            const GET_NAME: unsafe extern "C" fn(TaskHandle, *const c_char, *mut c_char, uInt32) -> int32 = ni_daqmx_sys::DAQmxGetAOCustomScaleName;
            const SET_NAME: unsafe extern "C" fn(TaskHandle, *const c_char, *const c_char) -> int32 = ni_daqmx_sys::DAQmxSetAOCustomScaleName;
            const RESET_NAME: unsafe extern "C" fn(TaskHandle, *const c_char) -> int32 = ni_daqmx_sys::DAQmxResetAOCustomScaleName;
        }
    };
}