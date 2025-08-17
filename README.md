# embedded-touch

The primary goal of this crate is to provide a common interface for UI libraries like
[buoyant](https://crates.io/crates/buoyant) to interact with touchscreen input device
drivers (e.g. `FT6113`).

## Features

- Blocking and async interfaces
- Stylus support, including pressure, tilt, and azimuth
- Fixed-point arithmetic by default
- No heap allocation

## Usage

Implement the `TouchInputDevice` trait for blocking operation:

```rust
use embedded_touch::{TouchInputDevice, Touch, Error};

struct MyTouchDriver { /* ... */ }

impl TouchInputDevice for MyTouchDriver {
    type Error = MyError;

    fn touches(&mut self) -> Result<impl IntoIterator<Item = Touch>, Error<Self::Error>> {
        // Read touch data from hardware
    }
}
```

Or implement `AsyncTouchInputDevice` for event-driven operation:

```rust
use embedded_touch::{AsyncTouchInputDevice, Touch, Error};

impl AsyncTouchInputDevice for MyTouchDriver {
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
