/// Crate for handling FFI interface types and general DAQmx Types.


/// The FFI exposes the char* interface as i8 and requires preallocation in a way
/// that CString doesn't like for string outputs.
///
/// This function will strip the end null out of the buffer and format to a string.
pub fn buffer_to_string(buffer: Vec<i8>) -> String {

    // First get just valid chars as u8
    let buffer_u8 = buffer.into_iter().take_while(|&e| e != 0 ).map(|e| e as u8).collect();

    // Build from utf8 - I think it may be ascii but should still be compliant as utf8.
    // In the Python API this is treated as UTF8 as well.
    String::from_utf8(buffer_u8).expect("Invalid Characters in Error Buffer")

}

#[cfg(test)]
mod tests {
    use crate::types::buffer_to_string;

    #[test]
    fn test_error_buffer_to_string_good() {
        let buffer: Vec<i8> =  vec![68, 101, 118, 105, 99, 101, 32, 105, 100, 101, 110, 0, 0, 0];
        let string = buffer_to_string(buffer);
        assert_eq!(&string, "Device iden");
    }

    #[test]
    fn test_error_buffer_to_string_no_null() {
        let buffer: Vec<i8> =  vec![68, 101, 118, 105, 99, 101, 32, 105, 100, 101, 110];
        let string = buffer_to_string(buffer);
        assert_eq!(&string, "Device iden");
    }

}