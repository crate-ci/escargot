use std::ffi;
use std::process;
use std::str;
use std::vec;

use failure;
use serde;
use serde_json;

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
        CargoBuild { cmd: self.cmd }
    }

    /// Run the `test` subcommand.
    pub fn test(mut self) -> CargoTest {
        self.cmd.arg("test").arg("--message-format=json");
        CargoTest { cmd: self.cmd }
    }
}

impl Default for Cargo {
    fn default() -> Self {
        Self::new()
    }
}

/// The `build` subcommand.
pub struct CargoBuild {
    cmd: process::Command,
}

impl CargoBuild {
    /// Shortcut to create a `build` subcommand.
    pub fn new() -> Self {
        Cargo::new().build()
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

    pub fn exec(self) -> Result<MessageItr, failure::Error> {
        MessageItr::from_command(self.cmd)
    }
}

impl Default for CargoBuild {
    fn default() -> Self {
        Self::new()
    }
}

/// The `test` subcommand.
pub struct CargoTest {
    cmd: process::Command,
}

impl CargoTest {
    /// Shortcut to create a `test` subcommand.
    pub fn new() -> Self {
        Cargo::new().test()
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

    pub fn exec(self) -> Result<MessageItr, failure::Error> {
        MessageItr::from_command(self.cmd)
    }
}

impl Default for CargoTest {
    fn default() -> Self {
        Self::new()
    }
}

/// Messages returned from a cargo sub-command.
pub struct MessageItr(vec::IntoIter<Message>);

impl MessageItr {
    fn from_command(mut cmd: process::Command) -> Result<MessageItr, failure::Error> {
        let output = cmd.output()?;
        if !output.status.success() {
            bail!("{}", String::from_utf8_lossy(&output.stderr));
        }

        let messages: Vec<Message> = str::from_utf8(&output.stdout)
            .expect("json to be UTF-8")
            .split('\n')
            .map(|s| Message {
                content: s.to_owned(),
            })
            .collect();

        Ok(Self {
            0: messages.into_iter(),
        })
    }
}

impl Iterator for MessageItr {
    type Item = Message;

    #[inline]
    fn next(&mut self) -> Option<Message> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }
}

/// An individual message from a cargo sub-command.
pub struct Message {
    content: String,
}

impl Message {
    /// Deserialize the message.
    pub fn convert<'a, T>(&'a self) -> Result<T, failure::Error>
    where
        T: serde::Deserialize<'a>,
    {
        let data = serde_json::from_str(self.content.as_str())?;
        Ok(data)
    }
}
