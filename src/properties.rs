use crate::daqmx_call;
use crate::error::{DaqmxError, handle_error, string_property_size_error};
use crate::tasks::Task;
use ni_daqmx_sys::TaskHandle;

/// Handles conversion between the DAQmx API and the Rust type.
pub trait PropertyValue: Sized {
    /// The value used at the DAQmx API.
    type Raw: Default;
    fn from_raw(raw: Self::Raw) -> Result<Self, DaqmxError>;
    fn into_raw(self) -> Self::Raw;
}

macro_rules! identity_property {
    ($($t:ty),*) => {$(
        impl PropertyValue for $t {
            type Raw = $t;
            fn from_raw(raw: $t) -> Result<Self, DaqmxError> { Ok(raw) }
            fn into_raw(self) -> $t { self }
        }
    )*};
}

identity_property!(f64, i32, u32, u64);
#[macro_export]
macro_rules! property {
    ($(#[$meta:meta])* get $name:ident: $ty:ty = $getter:path) => {
        $(#[$meta])*
        pub fn $name(&self) -> $crate::error::Result<$ty> {
            self.property_get($getter)
        }
    };
    ($(#[$meta:meta])* get_set $name:ident / $set:ident : $ty:ty = $getter:path, $setter:path) => {
        $crate::property!($(#[$meta])* get $name: $ty = $getter);

        #[doc = concat!("Sets the value read by [`Self::", stringify!($name), "`].")]
        pub fn $set(&self, value: $ty) -> $crate::error::Result<()> {
            self.property_set($setter, value)
        }
    };
    ($(#[$meta:meta])* get_set_reset $name:ident / $set:ident / $reset:ident : $ty:ty = $getter:path, $setter:path, $resetter:path) => {
        $crate::property!($(#[$meta])* get_set $name / $set: $ty = $getter, $setter);

        #[doc = concat!("Resets [`Self::", stringify!($name), "`] to its default value.")]
        pub fn $reset(&self) -> $crate::error::Result<()> {
            self.property_reset($resetter)
        }
    };
    ($(#[$meta:meta])* get_string $name:ident = $getter:path) => {
        $(#[$meta])*
        pub fn $name(&self) -> $crate::error::Result<String> {
            self.property_get_string($getter)
        }
    };
}
