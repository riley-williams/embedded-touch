//! Common traits and types for touch screen drivers (and mice)

#![no_std]

use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use fixed::{traits::ToFixed, types::U17F15};
use fixed_macro::types::{I17F15, U17F15};

pub mod traits;

/// Represents a single touch point on the screen
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Touch {
    /// Unique ID for tracking this touch point across frames
    ///
    /// The ID should remain stable for a given finger/stylus while it remains in contact
    /// but can be reused for new touches after sending [`Phase::Ended`] or [`Phase::Cancelled`]
    pub id: u8,

    /// Coordinates of the interaction in units of screen pixels
    pub location: TouchPoint,

    /// Current phase of this touch interaction
    pub phase: Phase,

    /// The tool used for this touch point
    pub tool: Tool,
}

impl Touch {
    /// Create a new touch point
    #[must_use]
    pub fn new(id: u8, location: TouchPoint, phase: Phase, tool: Tool) -> Self {
        Self {
            id,
            location,
            phase,
            tool,
        }
    }
}

/// Phase of a touch interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    /// Touch just started
    Started,
    /// Touch moved from previous position
    Moved,
    /// Touch ended normally
    Ended,
    /// Touch was cancelled (e.g., palm rejection triggered)
    Cancelled,
    /// Touch is hovering above the screen without contact, with an optional
    /// proximity (implementation-specific units)
    Hovering(Option<u16>),
}

/// Tool/instrument used for touch interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// An angle in the range [0, 2π) radians
///
/// The angle is stored as a [`fixed::types::U1F15`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    #[inline]
    pub fn as_pi_radians(&self) -> fixed::types::U1F15 {
        self.0
    }

    #[must_use]
    #[inline]
    pub fn as_radians_f32(&self) -> f32 {
        (self.0.to_fixed::<U17F15>() * U17F15!(3.14159265359)).to_num::<f32>()
    }

    #[must_use]
    #[inline]
    pub fn as_degrees_f32(&self) -> f32 {
        (self.0.to_fixed::<U17F15>() * U17F15!(180.0)).to_num::<f32>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TouchPoint {
    pub x: i32,
    pub y: i32,
}

impl TouchPoint {
    /// Create a new touch point
    #[must_use]
    pub fn new(x: impl Into<i32>, y: impl Into<i32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Add for TouchPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        TouchPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for TouchPoint {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for TouchPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        TouchPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for TouchPoint {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl core::ops::Neg for TouchPoint {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
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
