//! Integration tests to confirm multithreaded behaviour.
//!
use daqmx::channels::*;
use daqmx::tasks::*;
use daqmx::types::*;

#[test]
fn test_move_to_thread() {
    let mut task = Task::new("scalar").unwrap();
    let ch1 = VoltageChannelBuilder::new("PXI1Slot2/ai0").unwrap();
    task.create_channel(ch1).unwrap();
    task.configure_sample_clock_timing(
        None,
        1000.0,
        ClockEdge::Rising,
        SampleMode::FiniteSamples,
        100,
    )
    .unwrap();

    let mut buffer = [0.0; 100];

    task.start().unwrap();

    let join_handle = std::thread::spawn(move || {
        task.read(
            Timeout::Seconds(1.0),
            DataFillMode::GroupByChannel,
            Some(100),
            &mut buffer[..],
        )
        .unwrap();
    });

    join_handle.join().unwrap();
}

#[test]
/// This test will move the read to another thread but set stop from this thread.
/// This is a fairly commmon case for multithreading a task.
fn test_control_from_thread() {
    let mut task = Task::new("scalar").unwrap();
    let ch1 = VoltageChannelBuilder::new("PXI1Slot2/ai0").unwrap();
    task.create_channel(ch1).unwrap();
    task.configure_sample_clock_timing(
        None,
        1000.0,
        ClockEdge::Rising,
        SampleMode::ContinuousSamples,
        1000,
    )
    .unwrap();

    let mut buffer = [0.0; 100];

    task.set_read_auto_start(false).unwrap();
    task.start().unwrap();

    let mut thread_task = task.clone();

    let join_handle = std::thread::spawn(move || {
        for _ in 0..10 {
            let result = thread_task.read(
                Timeout::Seconds(1.0),
                DataFillMode::GroupByChannel,
                Some(100),
                &mut buffer[..],
            );

            if result.is_err() {
                return;
            }
        }

        //If we complete the iterations we weren't stopped. panic.
        panic!("Expected thread to be stopped by the task being stopped.");
    });

    println!("Sending stop");
    task.stop().unwrap();

    join_handle.join().unwrap();
}
