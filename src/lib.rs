mod task;
mod ai_channels;
mod types;
pub mod error;

use ai_channels::*;

pub use task::Task;

pub fn get_value() -> f64 {

    let mut task = task::Task::new("test task").unwrap();
    println!("Task Name: {}", task.name().unwrap());
    task.add_channel(Box::new(VoltageInputChannel {})).unwrap();
    //return a value

    return task.read();
}

