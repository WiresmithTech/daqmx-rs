use log::warn;
/// Error handling types and functions.
use thiserror::Error;

use crate::types::buffer_to_string;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DaqmxError {
    /// A DAQmx Generated Error. The i32 is the return code and the string is the extended description.
    #[error("DAQmx Generated Error: {1}")]
    DaqmxError(i32, String),
    #[error("String Value Not Valid for DAQmx API. Probably Contains Null")]
    CStringError(#[from] std::ffi::NulError),
    #[error("String value from DAQmx API does not contain valid Unicode (UTF8). This should not be possible and probably indicates corruption")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("String property length changed between reading the required length and reading the value. This is likely a race condition with another piece of code and a retry will probably correct this.")]
    StringPropertyLengthChanged,
    #[error("Value for given type ({0}) isn't a value that is expected: {1}")]
    UnexpectedValue(String, i32),
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

/// Checks the return code and either:
///
/// * Errors if it is an unexpected error.
/// * Returns `true` if there is a size error.
/// * Returns `false` if there is no error.
pub fn string_property_size_error(return_code: i32) -> Result<bool> {
    const TRUNCATED_WARNING: i32 = ni_daqmx_sys::DAQmxWarningCAPIStringTruncatedToFitBuffer as i32;
    match return_code {
        ni_daqmx_sys::DAQmxErrorBufferTooSmallForString | TRUNCATED_WARNING => {
            // Wrong size, go again.
            Ok(true)
        }
        //Given we know this rante of codes provides an error, the map should never be called.
        //Just used to satisfy the type system.
        i32::MIN..=-1 => handle_error(return_code).map(|()| false),
        _ => Ok(false),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_property_size_error() {
        assert_eq!(string_property_size_error(0), Ok(false));
        assert_eq!(
            string_property_size_error(ni_daqmx_sys::DAQmxErrorBufferTooSmallForString),
            Ok(true)
        );
        assert_eq!(
            string_property_size_error(
                ni_daqmx_sys::DAQmxWarningCAPIStringTruncatedToFitBuffer as i32
            ),
            Ok(true)
        );
        assert!(matches!(
            string_property_size_error(-1000),
            Err(DaqmxError::DaqmxError(-1000, _))
        ));
    }
}
