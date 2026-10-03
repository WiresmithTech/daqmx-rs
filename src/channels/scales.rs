use crate::channels::{ChannelKind, TaskChannel};
use crate::property;
use ni_daqmx_sys::{TaskHandle, int32, uInt32};
use std::ffi::{CStr, c_char};

pub trait CustomScaledChannel: ChannelKind {
    const GET_NAME: unsafe extern "C" fn(TaskHandle, *const c_char, *mut c_char, uInt32) -> int32;
    const SET_NAME: unsafe extern "C" fn(TaskHandle, *const c_char, *const c_char) -> i32;
    const RESET_NAME: unsafe extern "C" fn(TaskHandle, *const c_char) -> i32;
}

impl<K: CustomScaledChannel> TaskChannel<K> {
    property!(get_string custom_scale_name = K::GET_NAME);

    pub fn set_custom_scale_name(&self, name: &CStr) -> crate::error::Result<()> {
        self.property_set_raw(K::SET_NAME, name.as_ptr())
    }

    pub fn reset_custom_scale_name(&self) -> crate::error::Result<()> {
        self.property_reset(K::RESET_NAME)
    }
}
