use log::warn;
/// Error handling types and functions.
use thiserror::Error;

use crate::types::buffer_to_string;

#[derive(Error, Debug)]
pub enum DaqmxError {
    /// A DAQmx Generated Error. The i32 is the return code and the string is the extended description.
    #[error("DAQmx Generated Error: {1}")]
    DaqmxError(i32, String),
    #[error("String Value Not Valid for DAQmx API. Probably Contains Null")]
    CStringError(#[from] std::ffi::NulError),
}

pub type Result<T> = std::result::Result<T, DaqmxError>;

pub fn handle_error(return_code: i32) -> Result<()> {
    // This structure is based on how they handle this in Python.
    match return_code {
        0 => {
            return Result::Ok(());
        } //do nothing.
        i32::MIN..=-1 => {
            //use extended info for errors.
            unsafe {
                let mut buffer = vec![0i8; 2048];
                ni_daqmx_sys::DAQmxGetExtendedErrorInfo(buffer.as_mut_ptr(), 2048);
                let message = buffer_to_string(buffer);
                return Result::Err(DaqmxError::DaqmxError(return_code, message));
            }
        }
        1..=i32::MAX => {
            //use error string for warning. Just report to log.
            unsafe {
                let mut buffer = vec![0i8; 2048];
                ni_daqmx_sys::DAQmxGetErrorString(return_code, buffer.as_mut_ptr(), 2048);
                let message = buffer_to_string(buffer);
                warn!("DAQmx Warning: {:}", message);
            }
            return Result::Ok(());
        }
    }
}
