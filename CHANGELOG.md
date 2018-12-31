<a name="0.4.0"></a>
## 0.4.0 (2018-12-31)


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



<a name="0.3.1"></a>
## 0.3.1 (2018-08-07)


#### Bug Fixes

* **run:**  Example support ([99029550](https://github.com/crate-ci/escargot/commit/990295504ebd195f330e7b3e19b01e86a7b401f7), closes [#7](https://github.com/crate-ci/escargot/issues/7))



<a name="0.3.0"></a>
## 0.3.0 (2018-08-05)


#### Features

*   Emulate run subcommand ([df4607a8](https://github.com/crate-ci/escargot/commit/df4607a8170a27d746e7c259e05c478a02d570e5))

#### Breaking Changes

*  `current_target` spelling is corrected ([df4607a8](https://github.com/crate-ci/escargot/commit/df4607a8170a27d746e7c259e05c478a02d570e5))
*  Removed parts of `CargoError` ([df4607a8](https://github.com/crate-ci/escargot/commit/df4607a8170a27d746e7c259e05c478a02d570e5))

<a name="0.2.0"></a>
## 0.2.0 (2018-06-27)


#### Breaking Changes

*   Define concrete CargoError ([445cb391](https://github.com/crate-ci/escargot/commit/445cb39156b63ce1894d40b31805273d995e185c), breaks [#](https://github.com/crate-ci/escargot/issues/))
