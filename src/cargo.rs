use std::ffi;
use std::process;
use std::str;

use build::CargoBuild;
use test::CargoTest;

/// The current process' target triple.
pub const CURRENT_TARGET: &str = include_str!(concat!(env!("OUT_DIR"), "/current_target.txt"));

/// Top-level command.
#[derive(Debug)]
pub struct Cargo {
    cmd: process::Command,
}

impl Cargo {
    /// Create a top-level command.
    pub fn new() -> Self {
        Self {
            cmd: process::Command::new("cargo"),
        }
    }

    /// Manually pass an argument that is unsupported.
    ///
    /// Caution: Passing in a sub-command or `--` can throw off the API.
    pub fn arg<S: AsRef<ffi::OsStr>>(mut self, arg: S) -> Self {
        self.cmd.arg(arg);
        self
    }

    /// Run the `build` subcommand.
    pub fn build(mut self) -> CargoBuild {
        self.cmd.arg("build").arg("--message-format=json");
        CargoBuild::with_command(self.cmd)
    }

    /// Run the `test` subcommand.
    pub fn test(mut self) -> CargoTest {
        self.cmd.arg("test").arg("--message-format=json");
        CargoTest::with_command(self.cmd)
    }
}

impl Default for Cargo {
    fn default() -> Self {
        Self::new()
    }
}
