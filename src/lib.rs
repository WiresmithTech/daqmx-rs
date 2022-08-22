pub mod channels;
pub mod error;
mod tasks;
mod types;

pub use tasks::Task;

use crate::channels::VoltageChannelBuilder;
use crate::channels::VoltageInputChannel;
use crate::tasks::InputTask;
use crate::types::*;

pub fn get_value() -> f64 {
    let mut task = Task::new("test task").unwrap();
    println!("Task Name: {}", task.name().unwrap());

    let ch1 = VoltageChannelBuilder::new("PXI1Slot2/ai0").unwrap();
    let ch2 = VoltageChannelBuilder::new("PXI1Slot2/ai1").unwrap();
    task.create_channel(ch1).unwrap();
    task.create_channel(ch2).unwrap();
    let channel2: VoltageInputChannel = task.get_channel("PXI1Slot2/ai1").unwrap();
    println!("AI Max 2: {}", channel2.ai_max().unwrap());
    //return a value

    let mut buffer = [0.0; 2];
    task.read(
        Timeout::Seconds(1.0),
        DataFillMode::GroupByChannel,
        Some(1),
        &mut buffer[..],
    )
    .unwrap();

    return buffer[0];
}

#[macro_export]
macro_rules! daqmx_call {
    ($l:expr) => {
        handle_error(unsafe { $l })
    };
}
