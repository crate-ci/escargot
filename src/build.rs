use std::ffi;
use std::process;

use cargo::Cargo;
use cargo::CURRENT_TARGET;
use error::*;
use msg::*;

/// The `build` subcommand.
pub struct CargoBuild {
    cmd: process::Command,
}

impl CargoBuild {
    /// Shortcut to create a `build` subcommand.
    pub fn new() -> Self {
        Cargo::new().build()
    }

    pub(crate) fn with_command(cmd: process::Command) -> Self {
        Self { cmd }
    }

    /// Build only `name` binary.
    pub fn bin<S: AsRef<ffi::OsStr>>(self, name: S) -> Self {
        self.arg("--bin").arg(name)
    }

    /// Build only `name` example.
    pub fn example<S: AsRef<ffi::OsStr>>(self, name: S) -> Self {
        self.arg("--example").arg(name)
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
    pub fn current_taget(self) -> Self {
        self.target(CURRENT_TARGET)
    }

    /// Manually pass an argument that is unsupported.
    ///
    /// Caution: Passing in `--` can throw off the API.
    pub fn arg<S: AsRef<ffi::OsStr>>(mut self, arg: S) -> Self {
        self.cmd.arg(arg);
        self
    }

    pub fn exec(self) -> CargoResult<MessageItr> {
        MessageItr::from_command(self.cmd)
    }
}

impl Default for CargoBuild {
    fn default() -> Self {
        Self::new()
    }
}
