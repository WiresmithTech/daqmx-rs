use std::ffi::CString;

use crate::task::{TaskChannel};
use crate::error::{ Result, handle_error };

use nidaqmx_sys;
use nidaqmx_sys::{TaskHandle};


pub struct VoltageInputChannel {}

impl TaskChannel for VoltageInputChannel {

    fn add_to_task(&self, handle: TaskHandle) -> Result<()> {
        let c_channel = CString::new("PXI1Slot2/ai0").expect("Cant Create C String");

        let error_code = unsafe {
            nidaqmx_sys::DAQmxCreateAIVoltageChan(handle, c_channel.as_ptr(), empty_c_string().as_ptr(), nidaqmx_sys::DAQmx_Val_Cfg_Default, -10.0, 10.0, nidaqmx_sys::DAQmx_Val_Volts as i32, empty_c_string().as_ptr())
        };
        handle_error(error_code)
    }

}

fn empty_c_string() -> CString {
    CString::new("").expect("Cant Create C String")
}