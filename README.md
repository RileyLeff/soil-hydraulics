# soil-hydraulics 🐃 &emsp; ![Build Status] [![Codecov Badge]][Codecov Info] [![Apache Badge]][Apache Link] [![MIT Badge]][MIT Link]

Rust implementation of the soil hydraulic model described in
[Van Genuchten 1980](doi.org/10.2136/sssaj1980.03615995004400050002x).

## Compatibility

Builds on no_std with the error_in_core feature. Has std support behind feature flags. Requires nightly rust, for now. I'm sure it's possible to conditionally compile on stable + std (i.e. when error_in_core is not active) but don't have time to figure that out right now. If you need that, feel free to contribute.

## Dev Dependencies

The pre-commit config relies on [casey/just: 🤖 Just a command runner](https://github.com/casey/just)

## License

Licensed under either of

* [Apache License, Version 2.0][Apache Link]
* [MIT license][MIT LINK]

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[Build Status]: https://github.com/rileyleff/soil-hydraulics/actions/workflows/rust.yml/badge.svg
[Codecov Badge]: https://codecov.io/gh/RileyLeff/soil-hydraulics/graph/badge.svg?token=CEAG74DDK9
[Codecov Info]: https://codecov.io/gh/RileyLeff/soil-hydraulics
[MIT Badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[MIT Link]: https://opensource.org/licenses/MIT
[Apache Badge]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[Apache Link]: https://opensource.org/licenses/Apache-2.0
