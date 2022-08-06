mod channels;
pub mod error;
mod tasks;
mod types;

pub use tasks::Task;

pub fn get_value() -> f64 {
    let mut task = Task::new("test task").unwrap();
    println!("Task Name: {}", task.name().unwrap());
    task.create_voltage_channel("PXI1Slot2/ai0").unwrap();
    //return a value

    return task.read_scalar(std::time::Duration::from_secs(1)).unwrap();
}
