# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

#### Fixes

- Get `print` feature compiling
- Keep stdout clean with `print` by printing to stderr

## [0.5.5] - 2021-11-08

#### Features

- Add some missing traits

## [0.5.4] - 2021-11-08

#### Features

- Add `CargoBuild::args`

## [0.5.3] - 2021-11-08

#### Features

- Add `CargoBuild::examples`
- Add extra message fields

## [0.5.2] - 2021-03-27

#### Features

* New `env` method for passing environment variables to the `cargo` calls.

## [0.5.1] - 2021-02-01

#### Features

* Support `build-finished` message

## [0.5.0] - 2019-04-08

#### Features

*   Expose creating CommandMessages ([49d8767e](https://github.com/crate-ci/escargot/commit/49d8767e0122edebd0078e1ea1781a2eaf727ee5))
* **tests:**  Unstable support for running tests ([31293d79](https://github.com/crate-ci/escargot/commit/31293d796e2587cbc31bfff87af0fa4b22575de0))

#### Breaking Changes

*   Rename MessageIter -> CommandMessages ([f4742d8e](https://github.com/crate-ci/escargot/commit/f4742d8e1eb6b2bc242f24a5f0ceb0f9fb517070), breaks [#](https://github.com/crate-ci/escargot/issues/))

#### Bug Fixes

*   Gracefully handle upcoming cargo features ([a00f2408](https://github.com/crate-ci/escargot/commit/a00f240831ddc71b1846005df4917111e3690a82))

## [0.4.0] - 2018-12-31

#### Features

*   Serialization formats ([45f6e17a](https://github.com/crate-ci/escargot/commit/45f6e17a857baae7239c1a85ef6f7ccfa4baf35b))
*   Stream messages ([343027d4](https://github.com/crate-ci/escargot/commit/343027d40cdeb94b820ecb0a8fbb145fcf3f19c7), breaks [#](https://github.com/crate-ci/escargot/issues/))
*   Support CARGO env variable ([89021bec](https://github.com/crate-ci/escargot/commit/89021bec77cbef36a18e84917515b6ca3ebcc889), closes [#12](https://github.com/crate-ci/escargot/issues/12))
* **build:**
  *  Xargo support ([82c9c845](https://github.com/crate-ci/escargot/commit/82c9c845fe30e07bf29e1da6e5d2e884b3c5cc2b), closes [#16](https://github.com/crate-ci/escargot/issues/16))
  *  Support features ([e3575d37](https://github.com/crate-ci/escargot/commit/e3575d37399708080344de41a1344e52e97a9368), closes [#10](https://github.com/crate-ci/escargot/issues/10))
  *  target-dir support ([d7885f40](https://github.com/crate-ci/escargot/commit/d7885f40e498bf7653a89be51460978496161f76), closes [#17](https://github.com/crate-ci/escargot/issues/17))

#### Bug Fixes

*   Remove cargo-test support ([7aecd540](https://github.com/crate-ci/escargot/commit/7aecd5403f8c614aff685ed27b3305b5648c4dd6))

#### Breaking Changes

*   Nest lesser details ([e0133dbb](https://github.com/crate-ci/escargot/commit/e0133dbb1c01c5ac983b9376ae1c8e71dacaa42e), breaks [#](https://github.com/crate-ci/escargot/issues/))
*   Stream messages ([343027d4](https://github.com/crate-ci/escargot/commit/343027d40cdeb94b820ecb0a8fbb145fcf3f19c7), breaks [#](https://github.com/crate-ci/escargot/issues/))
*   MessageItr -> MessageIter ([07c4b257](https://github.com/crate-ci/escargot/commit/07c4b25740898b75af7b5d291be04ac737c5cd6c), closes [#9](https://github.com/crate-ci/escargot/issues/9), breaks [#](https://github.com/crate-ci/escargot/issues/))

## [0.3.1] - 2018-08-07

#### Bug Fixes

* **run:**  Example support ([99029550](https://github.com/crate-ci/escargot/commit/990295504ebd195f330e7b3e19b01e86a7b401f7), closes [#7](https://github.com/crate-ci/escargot/issues/7))

## [0.3.0] - 2018-08-05

#### Features

*   Emulate run subcommand ([df4607a8](https://github.com/crate-ci/escargot/commit/df4607a8170a27d746e7c259e05c478a02d570e5))

#### Breaking Changes

*  `current_target` spelling is corrected ([df4607a8](https://github.com/crate-ci/escargot/commit/df4607a8170a27d746e7c259e05c478a02d570e5))
*  Removed parts of `CargoError` ([df4607a8](https://github.com/crate-ci/escargot/commit/df4607a8170a27d746e7c259e05c478a02d570e5))

## [0.2.0] - 2018-06-27

#### Breaking Changes

*   Define concrete CargoError ([445cb391](https://github.com/crate-ci/escargot/commit/445cb39156b63ce1894d40b31805273d995e185c), breaks [#](https://github.com/crate-ci/escargot/issues/))

<!-- next-url -->
[Unreleased]: https://github.com/crate-ci/escargot/compare/v0.5.5...HEAD
[0.5.5]: https://github.com/crate-ci/escargot/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/crate-ci/escargot/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/crate-ci/escargot/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/crate-ci/escargot/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/crate-ci/escargot/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/crate-ci/escargot/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/crate-ci/escargot/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/crate-ci/escargot/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/crate-ci/escargot/compare/v0.2.0...v0.3.0
