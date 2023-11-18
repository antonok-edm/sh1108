# SH1108 driver

[![Crates.io](https://img.shields.io/crates/v/sh1108.svg)](https://crates.io/crates/sh1108)
[![Docs.rs](https://docs.rs/sh1108/badge.svg)](https://docs.rs/sh1108)

[![SH1108 display module showing information about the Rust driver crate](doc/readme_banner.webp?raw=true)](examples/demo.rs)

I2C/SPI driver for the SH1108 OLED display written in 100% Rust.

Derived from the [`SSD1309 driver`](https://github.com/antonok-edm/ssd1309).

## Implementation note

It's important to use correct reset logic for the SH1108, unlike with some other display drivers.
The `GraphicsMode::reset` method is a good way to ensure this is accomplished.

## Usage

Check the [documentation](https://docs.rs/sh1108) and [examples](examples/).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
