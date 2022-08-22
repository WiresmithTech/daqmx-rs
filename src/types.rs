/// Module for handling FFI interface types and general DAQmx Types.

/// The FFI exposes the char* interface as i8 and requires preallocation in a way
/// that CString doesn't like for string outputs.
///
/// This function will strip the end null out of the buffer and format to a string.
pub fn buffer_to_string(buffer: Vec<i8>) -> String {
    // First get just valid chars as u8
    let buffer_u8 = buffer
        .into_iter()
        .take_while(|&e| e != 0)
        .map(|e| e as u8)
        .collect();

    // Build from utf8 - I think it may be ascii but should still be compliant as utf8.
    // In the Python API this is treated as UTF8 as well.
    String::from_utf8(buffer_u8).expect("Invalid Characters in Error Buffer")
}

/// Describes the memory layout of a 1D buffer that represents 2D data.
///
/// This will impact the access patterns when you read the data which can impact performance.
pub enum DataFillMode {
    /// The layout groups data by channel. i.e. [Channel 0 Sample 0-2, Channel 1 Sample 0-2]
    /// Also known as noninterleaved.
    GroupByChannel,
    /// The layout groups data by sample.  i.e. [Sample 0 Channel 0-1, Sample 1 Channel 0-1, Sample 2 Channel 0-1]
    /// Also known as interleaved.
    GroupByScanNumber,
}

impl From<DataFillMode> for ni_daqmx_sys::bool32 {
    fn from(fill_mode: DataFillMode) -> Self {
        match fill_mode {
            DataFillMode::GroupByChannel => ni_daqmx_sys::DAQmx_Val_GroupByChannel,
            DataFillMode::GroupByScanNumber => ni_daqmx_sys::DAQmx_Val_GroupByScanNumber,
        }
    }
}

/// Enum representing the timeout options in the read and write APIs.
pub enum Timeout {
    /// Wait forever for the samples to become available.
    WaitForever,
    /// Immediately read the samples.
    NoWait,
    /// Time in seconds to wait for the requested samples to become available.
    /// If not all samples are available then whatever samples are available are read and an error is returned
    Seconds(f64),
}

impl From<Timeout> for f64 {
    fn from(timeout: Timeout) -> Self {
        match timeout {
            Timeout::WaitForever => ni_daqmx_sys::DAQmx_Val_WaitInfinitely,
            Timeout::NoWait => 0.0,
            Timeout::Seconds(seconds) => seconds,
        }
    }
}

/// Represents the behaviour for special sample counts

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_buffer_to_string_good() {
        let buffer: Vec<i8> = vec![68, 101, 118, 105, 99, 101, 32, 105, 100, 101, 110, 0, 0, 0];
        let string = buffer_to_string(buffer);
        assert_eq!(&string, "Device iden");
    }

    #[test]
    fn test_error_buffer_to_string_no_null() {
        let buffer: Vec<i8> = vec![68, 101, 118, 105, 99, 101, 32, 105, 100, 101, 110];
        let string = buffer_to_string(buffer);
        assert_eq!(&string, "Device iden");
    }

    #[test]
    fn timeout_conversion_tests() {
        assert_eq!(
            f64::from(Timeout::WaitForever),
            ni_daqmx_sys::DAQmx_Val_WaitInfinitely
        );

        assert_eq!(f64::from(Timeout::NoWait), 0.0);

        assert_eq!(f64::from(Timeout::Seconds(2.1)), 2.1);
    }
}
