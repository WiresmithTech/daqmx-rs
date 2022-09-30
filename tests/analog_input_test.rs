//! Integration tests for covering the analog input tasks and channels.
//!
use std::ffi::CString;

use daqmx::channels::*;
use daqmx::scales::LinearScale;
use daqmx::scales::PreScaledUnits;
use daqmx::tasks::*;
use daqmx::types::*;

#[test]
fn test_scalar_read() {
    let mut task = Task::new("scalar").unwrap();
    let ch1 = VoltageChannelBuilder::new("PXI1Slot2/ai0").unwrap();
    task.create_channel(ch1).unwrap();
    let _ = task.read_scalar(Timeout::Seconds(1.0)).unwrap();
    drop(task);
}

#[test]
fn test_buffered_read() {
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
    task.read(
        Timeout::Seconds(1.0),
        DataFillMode::GroupByChannel,
        Some(100),
        &mut buffer[..],
    )
    .unwrap();
}

#[test]
fn test_stop() {
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

    task.set_read_auto_start(false).unwrap();
    task.start().unwrap();
    task.read(
        Timeout::Seconds(1.0),
        DataFillMode::GroupByChannel,
        Some(100),
        &mut buffer[..],
    )
    .unwrap();

    //now stop and confirm read response.
    task.stop().unwrap();
    let read_result = task.read(
        Timeout::Seconds(1.0),
        DataFillMode::GroupByChannel,
        Some(100),
        &mut buffer[..],
    );

    assert!(matches!(
        read_result,
        Err(daqmx::error::DaqmxError::DaqmxError(-200473, _))
    ))
}

#[test]
fn test_voltage_input_builder() {
    let mut ch1 = VoltageChannelBuilder::new("PXI1Slot2/ai1").unwrap();
    ch1.name("my name").unwrap();
    ch1.scale = VoltageScale::Volts;
    ch1.max = 10.0;
    ch1.min = -10.0;
    ch1.terminal_config = AnalogTerminalConfig::RSE;

    let mut task = Task::new("").unwrap();
    task.create_channel(ch1).unwrap();

    let configured: VoltageInputChannel = task.get_channel("my name").unwrap();
    assert_eq!(
        configured.physical_channel().unwrap(),
        "PXI1Slot2/ai1".to_owned()
    );
    assert_eq!(configured.ai_max().unwrap(), 10.0);
    assert_eq!(configured.ai_min().unwrap(), -10.0);
    assert_eq!(
        configured.ai_terminal_config().unwrap(),
        AnalogTerminalConfig::RSE
    );
    assert_eq!(configured.scale().unwrap(), VoltageScale::Volts);
}

#[test]
fn test_voltage_input_builder_custom_scale() {
    //create custom scale first.
    let scale = LinearScale::new("TestScale", 1.0, 0.0, PreScaledUnits::Volts, "test").unwrap();
    let mut ch1 = VoltageChannelBuilder::new("PXI1Slot2/ai1").unwrap();
    ch1.name("my name").unwrap();
    ch1.scale = VoltageScale::CustomScale(Some(CString::new("TestScale").expect("Name Error")));
    ch1.max = 10.0;
    ch1.min = -10.0;
    ch1.terminal_config = AnalogTerminalConfig::RSE;

    let mut task = Task::new("").unwrap();
    task.create_channel(ch1).unwrap();

    let configured: VoltageInputChannel = task.get_channel("my name").unwrap();

    assert_eq!(
        configured.scale().unwrap(),
        VoltageScale::CustomScale(Some(CString::new("TestScale").expect("Name Error")))
    );
}
