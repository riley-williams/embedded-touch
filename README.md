# embedded-touch

A `no_std` crate providing common interfaces for touchscreen input devices.

## Features

- Blocking and async interfaces
- Stylus support, including pressure, tilt, and azimuth
- Fixed-point arithmetic by default
- No heap allocation

## Usage

Implement the `TouchScreen` trait for blocking operation:

```rust
use embedded_touch::{TouchScreen, Touch, Error};

struct MyTouchDriver { /* ... */ }

impl TouchScreen for MyTouchDriver {
    type Error = MyError;

    fn touches(&mut self) -> Result<impl IntoIterator<Item = Touch>, Error<Self::Error>> {
        // Read touch data from hardware
    }
}
```

Or implement `AsyncTouchScreen` for event-driven operation:

```rust
use embedded_touch::{AsyncTouchScreen, Touch, Error};

impl AsyncTouchScreen for MyTouchDriver {
    type Error = MyError;

    async fn touches(&mut self) -> Result<impl IntoIterator<Item = Touch>, Error<Self::Error>> {
        // Asynchronously wait for touch events
    }
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
