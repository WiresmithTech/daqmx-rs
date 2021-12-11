mod tasks;
mod channels;
mod types;
pub mod error;

use channels::*;

pub use tasks::{Task, AnalogInputTask, InputTask};

pub fn get_value() -> f64 {

    let mut task = AnalogInputTask::new("test task").unwrap();
    println!("Task Name: {}", task.name().unwrap());
    task.add_channel(Box::new(VoltageInputChannel {})).unwrap();
    //return a value

    return task.read();
}

