use crate::Touch;

/// Blocking interface for touch devices.
pub trait TouchInputDevice {
    /// Error type from the underlying interface
    type Error;

    /// Read current touch points, blocking until data is available
    ///
    /// Returns an iterator of *all* touch points currently detected.
    /// Drivers must track touch IDs across calls to maintain correct phase information.
    fn touches(&mut self) -> Result<impl IntoIterator<Item = &Touch>, Self::Error>;
}

/// Async interface for event-driven operation of touch devices
pub trait AsyncTouchInputDevice {
    /// Error type from the underlying interface
    type Error;

    /// Asynchronously wait until touch points are available
    ///
    /// Returns an iterator of *all* touch points currently detected.
    /// Drivers must track touch IDs across calls to maintain correct phase information.
    fn touches(
        &mut self,
    ) -> impl Future<Output = Result<impl IntoIterator<Item = &Touch>, Self::Error>>;
}
