pub mod ai_channels;
pub mod ao_channels;
mod properties;
mod scales;

use crate::property;
pub use ai_channels::AnalogInputKind;
pub use ao_channels::AnalogOutputKind;
use ni_daqmx_sys::TaskHandle;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;

pub trait ChannelKind {}

/// Represents a channel in a task.
///
/// The kind sets the available properties of the channel.
pub struct TaskChannel<K: ChannelKind> {
    task: TaskHandle,
    name: CString,
    _kind: PhantomData<K>,
}

impl<K: ChannelKind> TaskChannel<K> {
    property!(get_string physical_channel = ni_daqmx_sys::DAQmxGetPhysicalChanName);
    pub(crate) fn new(task: TaskHandle, name: CString) -> TaskChannel<K> {
        TaskChannel {
            task,
            name,
            _kind: PhantomData,
        }
    }

    fn task(&self) -> TaskHandle {
        self.task
    }

    fn name(&self) -> &CStr {
        &self.name
    }
}

/// A builder pattern for various channel kinds.
///
/// This will initialize the channel with defaults for the given physical channel and
/// then allow you to override elements needed to create the channel.
///
/// Default values aim to mimic LabVIEW's defaults for the channel kind.
///
/// If the parameter you want to set is not in the builder, you may need to create
/// the channel on a task and then retrieve its [`TaskChannel`].
pub trait ChannelBuilder: Sized {
    type Kind: ChannelKind;
    fn new<S: Into<Vec<u8>>>(physical_channel: S) -> crate::error::Result<Self>;

    fn name<S: Into<Vec<u8>>>(self, name: S) -> crate::error::Result<Self>;
    fn add_to_task(self, task: TaskHandle) -> crate::error::Result<TaskChannel<Self::Kind>>;
}
