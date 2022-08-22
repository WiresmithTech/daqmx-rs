pub mod channels;
pub mod error;
pub mod tasks;
pub mod types;

pub use tasks::Task;

use crate::channels::VoltageChannelBuilder;
use crate::channels::VoltageInputChannel;
use crate::tasks::InputTask;
pub use crate::types::*;

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
    task.configure_sample_clock_timing(
        None,
        1000.0,
        ClockEdge::default(),
        SampleMode::FiniteSamples,
        5,
    )
    .unwrap();

    let mut buffer = [0.0; 10];
    task.read(
        Timeout::Seconds(1.0),
        DataFillMode::GroupByChannel,
        None,
        &mut buffer[..],
    )
    .unwrap();

    println!("{:?}", buffer);
    return buffer[0];
}

#[macro_export]
macro_rules! daqmx_call {
    ($l:expr) => {
        handle_error(unsafe { $l })
    };
}
