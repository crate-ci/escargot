use std::ffi;
use std::process;

use cargo::Cargo;
use cargo::CURRENT_TARGET;
use error::*;
use msg::*;

/// The `test` subcommand.
pub struct CargoTest {
    cmd: process::Command,
}

impl CargoTest {
    /// Shortcut to create a `test` subcommand.
    pub fn new() -> Self {
        Cargo::new().test()
    }

    pub(crate) fn with_command(cmd: process::Command) -> Self {
        Self { cmd }
    }

    /// Compile, but don't run tests.
    pub fn no_run(self) -> Self {
        self.arg("--no-run")
    }

    /// Build artifacts in release mode, with optimizations.
    pub fn release(self) -> Self {
        self.arg("--release")
    }

    /// Build artifacts in release mode if the current process has, with optimizations.
    #[cfg(debug_assertions)]
    pub fn current_release(self) -> Self {
        self
    }

    /// Build artifacts in release mode if the current process has, with optimizations.
    #[cfg(not(debug_assertions))]
    pub fn current_release(self) -> Self {
        self.release()
    }

    /// Build for the target triple.
    pub fn target<S: AsRef<ffi::OsStr>>(self, triplet: S) -> Self {
        self.arg("--target").arg(triplet)
    }

    /// Build for the current process' triple.
    pub fn current_target(self) -> Self {
        self.target(CURRENT_TARGET)
    }

    /// Manually pass an argument that is unsupported.
    ///
    /// Caution: Passing in `--` can throw off the API.
    pub fn arg<S: AsRef<ffi::OsStr>>(mut self, arg: S) -> Self {
        self.cmd.arg(arg);
        self
    }

    /// Test the configured target, returning compiler messages.
    pub fn exec(self) -> CargoResult<MessageItr> {
        MessageItr::from_command(self.cmd)
    }
}

impl Default for CargoTest {
    fn default() -> Self {
        Self::new()
    }
}
