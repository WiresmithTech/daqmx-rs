//! Integration tests for covering the analog input tasks and channels.
//!
use daqmx::channels::ai_channels::AnalogChannelBuilder;
use daqmx::channels::ao_channels::voltage::{
    Voltage, VoltageOutputChannelBuilder, VoltageOutputScale,
};
use daqmx::channels::*;
use daqmx::scales::LinearScale;
use daqmx::scales::PreScaledUnits;
use daqmx::tasks::*;
use daqmx::types::*;
use std::ffi::CString;
use std::sync::Arc;

#[test]
fn test_scalar_read() {
    let mut task = Task::<AnalogOutput>::new("scalar").unwrap();
    let ch1 = VoltageOutputChannelBuilder::new("PXI1Slot2/ao0").unwrap();
    task.create_channel(ch1).unwrap();
    let _ = task.write_scalar(Timeout::WaitForever, true, 2.0).unwrap();
    drop(task);
}

#[test]
fn test_buffered_write() {
    let mut task = Task::<AnalogOutput>::new("buffered").unwrap();
    let ch1 = VoltageOutputChannelBuilder::new("PXI1Slot2/ao0").unwrap();
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

    task.write(
        Timeout::Seconds(1.0),
        false,
        DataFillMode::GroupByChannel,
        buffer.len(),
        &mut buffer[..],
    )
    .unwrap();

    task.set_regeneration_mode(RegenerationMode::Allowed)
        .unwrap();
    assert_eq!(task.regeneration_mode().unwrap(), RegenerationMode::Allowed);
    task.set_write_relative_to(WriteRelativeTo::CurrentWritePosition)
        .unwrap();
    assert_eq!(
        task.write_relative_to().unwrap(),
        WriteRelativeTo::CurrentWritePosition
    );
    task.set_offset(2).unwrap();
    assert_eq!(task.offset().unwrap(), 2);

    task.start().unwrap();
    task.stop().unwrap();
}

#[test]
fn test_stop() {
    let mut task = Task::<AnalogOutput>::new("scalar").unwrap();
    let ch1 = VoltageOutputChannelBuilder::new("PXI1Slot2/ao0").unwrap();
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

    task.write(
        Timeout::Seconds(1.0),
        true,
        DataFillMode::GroupByChannel,
        buffer.len(),
        &mut buffer[..],
    )
    .unwrap();

    //now stop and confirm next write fails.
    task.stop().unwrap();
    let write_result = task.write(
        Timeout::Seconds(1.0),
        false,
        DataFillMode::GroupByChannel,
        buffer.len(),
        &mut buffer[..],
    );

    assert!(
        matches!(
            write_result,
            Err(daqmx::error::DaqmxError::DaqmxError(-200288, _))
        ),
        "{:?}",
        write_result
    );
}

#[test]
fn test_voltage_output_builder() {
    let ch1 = VoltageOutputChannelBuilder::new("PXI1Slot2/ao1")
        .unwrap()
        .name("my name")
        .unwrap()
        .scale(VoltageOutputScale::Volts)
        .max(10.0)
        .min(-10.0);

    let mut task = Task::<AnalogOutput>::new("").unwrap();
    task.create_channel(ch1).unwrap();

    let configured: TaskChannel<Voltage> = task.get_channel("my name").unwrap();
    assert_eq!(
        configured.physical_channel().unwrap(),
        "PXI1Slot2/ao1".to_owned()
    );
    assert_eq!(configured.ao_max().unwrap(), 10.0);
    assert_eq!(configured.ao_min().unwrap(), -10.0);
    assert_eq!(configured.scale().unwrap(), VoltageOutputScale::Volts);
}

#[test]
fn test_voltage_input_builder_custom_scale() {
    //create custom scale first.
    let _scale = LinearScale::new("TestScale", 1.0, 0.0, PreScaledUnits::Volts, "test").unwrap();
    let ch1 = VoltageOutputChannelBuilder::new("PXI1Slot2/ao1")
        .unwrap()
        .name("my name")
        .unwrap()
        .scale(VoltageOutputScale::new_custom("TestScale").unwrap())
        .max(10.0)
        .min(-10.0);

    let mut task = Task::<AnalogOutput>::new("").unwrap();
    task.create_channel(ch1).unwrap();

    let configured: TaskChannel<Voltage> = task.get_channel("my name").unwrap();

    assert_eq!(
        configured.scale().unwrap(),
        VoltageOutputScale::CustomScale(Some(Arc::new(
            CString::new("TestScale").expect("Name Error")
        )))
    );
}
