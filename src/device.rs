use core::fmt::Write;

use serde::{Deserialize, Serialize};
use serde_json::error::Category;
use serde_json::value::Value;

use crate::route::Routes;
use crate::ShortString;

/// A device response payload for a determined action.
#[derive(Serialize, Deserialize)]
pub struct DevicePayload(Option<Value>);

impl DevicePayload {
    /// Creates an empty [`DevicePayload`].
    pub fn empty() -> Self {
        Self(None)
    }

    /// Creates a new [`DevicePayload`].
    pub fn new(value: impl Serialize) -> core::result::Result<Self, DeviceError> {
        serde_json::to_value(value)
            .map(|value| Self(Some(value)))
            .map_err(DeviceError::from_serialize)
    }
}

/// Kinds of erroneous device responses.
#[derive(Serialize, Deserialize)]
pub enum DeviceErrorKind {
    /// The device response for a determined action is not valid because
    /// retrieved data is not correct.
    Invalid,
    /// A device response for a determined action is wrong because an internal
    /// device error occurred.
    Wrong,
}

/// A device error response.
#[derive(Serialize, Deserialize)]
pub struct DeviceError {
    /// Kind of erroneous device response.
    pub kind: DeviceErrorKind,
    /// Information about the error.
    pub info: ShortString,
}

impl DeviceError {
    /// Creates a new [`DeviceError`] where the error is given as
    /// a string slice.
    pub fn from_str(kind: DeviceErrorKind, info: &str) -> Self {
        Self {
            kind,
            info: ShortString::new(info).unwrap_or(ShortString::empty()),
        }
    }

    // Creates a new [`DeviceError`] from a serialization error.
    fn from_serialize(error: serde_json::Error) -> Self {
        let category = match error.classify() {
            Category::Io => "IO",
            Category::Syntax => "Syntax",
            Category::Data => "Data",
            Category::Eof => "Eof",
        };

        let mut info = ShortString::empty();
        Self {
            kind: DeviceErrorKind::Wrong,
            info: if write!(
                info,
                "Error `{}` (line {}, column {})",
                category,
                error.line() as u16,
                error.column() as u16,
            )
            .is_ok()
            {
                info
            } else {
                ShortString::empty()
            },
        }
    }
}

/// Device data.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceData<'a> {
    /// Device kind.
    pub kind: DeviceKind,
    /// Main device route.
    #[serde(rename = "main route")]
    pub main_route: &'a str,
    #[serde(borrow)]
    /// All device routes.
    pub routes: Routes<'a>,
}

/// A device kind.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceKind {
    /// Light.
    Light,
}

/// A trait to serialize device data.
pub trait DeviceSerializer {
    /// Serializes device data.
    fn serialize_data(&self) -> DeviceData;
}
