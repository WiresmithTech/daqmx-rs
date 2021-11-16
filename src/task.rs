/// Provides a wrapper and functions for the DAQmx Task
use std::ffi::CString;
use std::ptr;
use nidaqmx_sys::{TaskHandle};


pub struct Task {
    handle: nidaqmx_sys::TaskHandle
}

impl Task {
    pub fn new(name: &str) -> Self {

        

        let c_name = CString::new(name).expect("Can't Make C String Name");
        let handle = unsafe {
            let mut tmp_handle: nidaqmx_sys::TaskHandle = ptr::null_mut();
            let error_code = nidaqmx_sys::DAQmxCreateTask(c_name.as_ptr(), &mut tmp_handle);
            tmp_handle

        };


        Self {
            handle
        }

    }

    pub fn add_channel(&mut self, channel: Box<dyn TaskChannel>) {
        channel.add_to_task(self.task_handle());
    }

    pub fn read(&mut self) -> f64 {
        let mut value: f64 = 0.0;
        let error_code = unsafe {
            nidaqmx_sys::DAQmxReadAnalogScalarF64(self.task_handle(), 1.0, &mut value, std::ptr::null_mut() )
        };
        return value;

    }

    fn task_handle(&self) -> nidaqmx_sys::TaskHandle {
        self.handle
    }
}

pub fn handle_error(return_code: i32) {

    // This structure is based on how they handle this in Python.
    match return_code {
        0 => {}//do nothing.
        i32::MIN..=-1 => {
            //use extended info for errors.
            unsafe {
                let mut buffer = vec![0i8; 2048];
                nidaqmx_sys::DAQmxGetExtendedErrorInfo(buffer.as_mut_ptr(), 2048);
                let message = error_buffer_to_string(buffer);
                println!("DAQmx Error: {:}", message);
            }
        }
        1..=i32::MAX => {
            //use error string for warning.
            unsafe {
                let mut buffer = vec![0i8; 2048];
                nidaqmx_sys::DAQmxGetErrorString(return_code, buffer.as_mut_ptr(), 2048);
                let message = error_buffer_to_string(buffer);
                println!("DAQmx Warning: {:}", message);
            }
        }
    }
}

fn error_buffer_to_string(buffer: Vec<i8>) -> String {

    // First get just valid chars as u8
    let buffer_u8 = buffer.into_iter().take_while(|&e| e != 0 ).map(|e| e as u8).collect();

    // Build from utf8 - I think it may be ascii but should still be compliant as utf8.
    // In the Python API this is treated as UTF8 as well.
    String::from_utf8(buffer_u8).expect("Invalid Characters in Error Buffer")

}

impl Drop for Task {
    fn drop(&mut self) {
        unsafe { nidaqmx_sys::DAQmxClearTask(self.task_handle()) };
    }
}

pub trait TaskChannel {

    fn add_to_task(&self, handle: TaskHandle);
}

#[cfg(test)]
mod tests {
    use crate::task::error_buffer_to_string;


    #[test]
    fn test_error_buffer_to_string_good() {
        let buffer: Vec<i8> =  vec![68, 101, 118, 105, 99, 101, 32, 105, 100, 101, 110, 0, 0, 0];
        let string = error_buffer_to_string(buffer);
        assert_eq!(&string, "Device iden");
    }

    #[test]
    fn test_error_buffer_to_string_no_null() {
        let buffer: Vec<i8> =  vec![68, 101, 118, 105, 99, 101, 32, 105, 100, 101, 110];
        let string = error_buffer_to_string(buffer);
        assert_eq!(&string, "Device iden");
    }

}


