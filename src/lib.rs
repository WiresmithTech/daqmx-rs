mod channels;
pub mod error;
mod tasks;
mod types;

pub use tasks::Task;

use crate::channels::AnalogInputChannelBase;

pub fn get_value() -> f64 {
    let mut task = Task::new("test task").unwrap();
    println!("Task Name: {}", task.name().unwrap());
    task.create_voltage_channel("PXI1Slot2/ai0").unwrap();
    task.configure_channel("PXI1Slot2/ai0", |channel: AnalogInputChannelBase| {
        println!("AI Max: {}", channel.ai_max().unwrap());
        Ok(())
    })
    .unwrap();
    //return a value

    return task.read_scalar(std::time::Duration::from_secs(1)).unwrap();
}

#[macro_export]
macro_rules! daqmx_call {
    ($l:expr) => {
        handle_error(unsafe { $l })
    };
}
