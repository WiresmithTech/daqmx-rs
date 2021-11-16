mod task;
mod ai_channels;

use ai_channels::*;

pub fn get_value() -> f64 {

    let mut task = task::Task::new("test task");
    task.add_channel(Box::new(VoltageInputChannel {}));
    //return a value

    return task.read();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
