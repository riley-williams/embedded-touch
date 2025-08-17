//! Common traits and types for touch screen drivers (and mice)

#![no_std]

use core::fmt::Debug;

use fixed::{traits::ToFixed, types::U17F15};
use fixed_macro::types::{I17F15, U17F15};

/// Blocking interface for touch devices.
pub trait TouchInputDevice {
    /// Error type from the underlying interface
    type Error;

    /// Read current touch points, blocking until data is available
    ///
    /// Returns an iterator of touch points currently detected.
    /// Drivers must track touch IDs across calls to maintain correct phase information.
    fn touches(&mut self) -> Result<impl IntoIterator<Item = Touch>, Error<Self::Error>>;
}

/// Async interface for event-driven operation of touch devices
pub trait AsyncTouchInputDevice {
    /// Error type from the underlying interface
    type Error;

    /// Asynchronously wait until touch points are available
    ///
    /// Returns an iterator of touch points currently detected.
    /// Drivers must track touch IDs across calls to maintain correct phase information.
    fn touches(
        &mut self,
    ) -> impl Future<Output = Result<impl IntoIterator<Item = Touch>, Error<Self::Error>>>;
}

/// Represents a single touch point on the screen
#[derive(Debug, Clone, PartialEq)]
pub struct Touch {
    /// Unique ID for tracking this touch point across frames
    ///
    /// The ID should remain stable for a given finger/stylus while it remains in contact
    /// but can be reused for new touches after sending [`Phase::Ended`] or [`Phase::Cancelled`]
    pub id: u8,

    /// X coordinate in screen pixels
    pub x: u16,

    /// Y coordinate in screen pixels
    pub y: u16,

    /// Current phase of this touch interaction
    pub phase: Phase,

    /// The tool used for this touch point
    pub tool: Tool,

    /// Optional proximity distance (implementation-specific units)
    pub proximity: Option<u16>,
}

/// Phase of a touch interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Touch just started
    Started,
    /// Touch moved from previous position
    Moved,
    /// Touch ended normally
    Ended,
    /// Touch was cancelled (e.g., palm rejection triggered)
    Cancelled,
    /// Touch is hovering above the screen without contact
    Hovering,
}

/// Tool/instrument used for touch interaction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    /// Finger or unknown tool
    Finger,
    /// Virtual pointing device (e.g., mouse cursor)
    Pointer {
        /// The button pressed on the virtual pointer
        button: PointerButton,
    },
    /// Passive or active stylus
    Stylus {
        /// Pressure, in grams
        pressure: Option<u16>,
        /// Tilt angle
        ///
        /// 0 degrees is a vector normal to the screen, and 90 degrees is parallel to the screen.
        tilt: Option<UnitAngle>,
        /// Azimuth angle
        ///
        /// 0 degrees points up to the top of the screen in its default orientation.
        azimuth: Option<UnitAngle>,
    },
}

/// The button state of a virtual pointer device
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerButton {
    /// No button pressed, e.g., mouse hover state
    None,
    /// Primary mouse button, typically left
    Primary,
    /// Secondary mouse button, typically right
    Secondary,
    /// Tertiary mouse button, typically middle or wheel
    Tertiary,
}

/// Error types for touch operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error<E> {
    /// I2C/SPI communication error
    Interface(E),
    /// Data corruption detected (e.g., checksum failure)
    DataCorruption,
    /// Device not responding or not initialized
    DeviceError,
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::Interface(error)
    }
}

/// An angle in the range [0, 2π) radians
///
/// The angle is stored as a [`fixed::types::U1F15`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitAngle(fixed::types::U1F15);

impl UnitAngle {
    /// Create a new angle from an angle in π * radians.
    ///
    /// Angles outside the range [0, 2) are wrapped.
    ///
    /// This method does not result in loss of precision if the argument is
    /// [`fixed::types::U1F15`]
    #[must_use]
    pub fn from_pi_radians(value: impl ToFixed) -> Self {
        UnitAngle(value.wrapping_to_fixed())
    }

    /// Create a new angle from an angle in radians
    ///
    /// Angles outside the range [0, 2π) are wrapped.
    #[must_use]
    pub fn from_radians(value: impl ToFixed) -> Self {
        let fixed_radians = value.to_fixed::<fixed::types::U17F15>();
        let pi_radians = fixed_radians / U17F15!(3.14159265359);
        UnitAngle(pi_radians.wrapping_to_fixed())
    }

    /// Create a new angle from an angle in degrees
    ///
    /// Angles outside the range [0, 360) are wrapped.
    #[must_use]
    pub fn from_degrees(value: impl ToFixed) -> Self {
        let fixed_degrees = value.to_fixed::<fixed::types::I17F15>();
        let radians = fixed_degrees / I17F15!(180);
        UnitAngle(radians.wrapping_to_fixed())
    }

    /// Returns the angle in π radians, in the range [0, 2)
    ///
    /// This method does not result in loss of precision from the original value.
    #[must_use]
    pub fn as_pi_radians(&self) -> fixed::types::U1F15 {
        self.0
    }

    #[must_use]
    pub fn as_radians_f32(&self) -> f32 {
        (self.0.to_fixed::<U17F15>() * U17F15!(3.14159265359)).to_num::<f32>()
    }

    #[must_use]
    pub fn as_degrees_f32(&self) -> f32 {
        (self.0.to_fixed::<U17F15>() * U17F15!(180.0)).to_num::<f32>()
    }
}

#[cfg(test)]
mod tests {
    use core::f32;

    use super::*;

    #[test]
    #[expect(clippy::cast_precision_loss)]
    fn angle_from_pi_radians() {
        let angle = UnitAngle::from_pi_radians(0.0);
        assert_eq!(angle.as_pi_radians(), fixed::types::U1F15::from_num(0.0));
        assert!(angle.as_radians_f32().abs() < 0.00001);
        assert!(angle.as_degrees_f32().abs() < 0.00001);

        for i in -8..8 {
            let offset = (i * 2) as f32;
            let angle = UnitAngle::from_pi_radians(1.0 + offset);
            assert_eq!(angle.as_pi_radians(), fixed::types::U1F15::from_num(1.0));
            assert!((angle.as_radians_f32() - 1.0 * f32::consts::PI).abs() < 0.00001);
            assert!((angle.as_degrees_f32() - 180.0).abs() < 0.00001);
        }
    }

    #[test]
    #[expect(clippy::cast_precision_loss)]
    fn sweep_360_degrees() {
        for i in -1080..1080 {
            let angle = UnitAngle::from_degrees(i);

            let unit_degrees = (i + 360 * 20) % 360;
            let radians = unit_degrees as f32 * f32::consts::PI / 180.0;

            assert!(
                (angle.as_degrees_f32() - unit_degrees as f32).abs() < 0.01,
                "Expected {} to be nearly {unit_degrees}",
                angle.as_degrees_f32()
            );
            assert!(
                (angle.as_radians_f32() - radians).abs() < 0.001,
                "Expected {}  to be nearly {radians}",
                angle.as_radians_f32()
            );
        }
    }
}
