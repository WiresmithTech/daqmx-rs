pub mod channels;
pub mod error;
pub mod scales;
pub mod tasks;
pub mod types;

#[macro_export]
macro_rules! daqmx_call {
    ($l:expr) => {
        handle_error(unsafe { $l })
    };
}
