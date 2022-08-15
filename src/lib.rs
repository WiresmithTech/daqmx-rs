mod channels;
pub mod error;
mod tasks;
mod types;

pub use tasks::Task;

use crate::channels::AnalogInputChannelBase;
use crate::channels::VoltageChannelBuilder;

pub fn get_value() -> f64 {
    let mut task = Task::new("test task").unwrap();
    println!("Task Name: {}", task.name().unwrap());

    let ch1 = VoltageChannelBuilder::new("PXI1Slot2/ai0", -10.0, 10.0).unwrap();
    let ch2 = VoltageChannelBuilder::new("PXI1SLot2/ai1", -10.0, 10.0).unwrap();
    task.create_channel(ch1).unwrap();
    task.create_channel(ch2).unwrap();
    let channel2: AnalogInputChannelBase = task.get_channel("PXI1Slot2/ai1").unwrap();
    println!("AI Max 2: {}", channel2.ai_max().unwrap());
    //return a value

    return task.read_scalar(std::time::Duration::from_secs(1)).unwrap();
}

#[macro_export]
macro_rules! daqmx_call {
    ($l:expr) => {
        handle_error(unsafe { $l })
    };
}
