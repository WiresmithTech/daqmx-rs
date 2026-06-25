mod ai_channels;
mod properties;

pub use ai_channels::*;

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use ni_daqmx_sys::TaskHandle;

/// Represents a channel in a task.
///
/// The kind sets the available properties of the channel.
pub struct TaskChannel<K> {
    task: TaskHandle,
    name: CString,
    _kind: PhantomData<K>,
}


impl <K> TaskChannel<K> {
    
    pub(crate) fn new(task: TaskHandle, name: &str) -> Result<TaskChannel<K>, DaqmxError> {
        Ok(TaskChannel {
            task,
            name: CString::new(name)?,
            _kind: PhantomData,
        })
    }

    fn task(&self) -> TaskHandle {
        self.task
    }
    
    fn name(&self) -> &CStr {
        &self.name
    }
}
macro_rules! property {
    (get $name:ident: $ty:ty = $getter:path) => {
        pub fn $name(&self) -> Result<$ty> { self.property_get($getter) }
    };
    (get_set $name:ident / $set:ident : $ty:ty = $getter:path, $setter:path) => {
        pub fn $name(&self) -> Result<$ty> { self.property_get($getter) }
        pub fn $set(&self, value: $ty) -> Result<()> { self.property_set($setter, value) }
    };
    (get_string $name:ident = $getter:path) => {
        pub fn $name(&self) -> Result<String> { self.property_get_string($getter) }
    };
}

pub(crate) use property;
use crate::error::DaqmxError;
use crate::tasks::{AnalogInput, Task};
