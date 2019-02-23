use std::path;
use std::process;

use error::*;
use msg::*;
use run;

/// The `test` subcommand (emulated).
///
/// Created via [`CargoBuild::run_tests`].
///
/// Benefits over spawning `cargo test`:
/// - Able to cache binary path, avoiding cargo overhead.
/// - Independent of CWD.
/// - stdout/stderr are clean of `cargo test` output.
///
/// Required feature: `test_unstable` since the format parsed is unstable.
///
/// Relevant features
/// - `print` for logged output to be printed instead, generally for test writing.
///
/// # Example
///
/// ```rust
/// let run = escargot::CargoBuild::new()
///     .test("build")
///     .current_release()
///     .current_target()
///     .run_tests().unwrap()
///     .next().unwrap().unwrap();
/// println!("artifact={}", run.path().display());
/// ```
///
/// [`CargoBuild::run_tests`]: struct.CargoBuild.html#method.run_tests
pub struct CargoTest {
    bin_path: path::PathBuf,
}

impl CargoTest {
    pub(crate) fn with_messages(
        msgs: CommandMessages,
    ) -> impl Iterator<Item = Result<Self, CargoError>> {
        run::extract_binary_paths(msgs, "test").map(|p| p.map(|p| Self { bin_path: p }))
    }

    /// Path to the specified binary.
    ///
    /// This is to support alternative ways of launching the binary besides [`Command`].
    ///
    /// # Example
    ///
    /// ```rust
    /// let run = escargot::CargoBuild::new()
    ///     .bin("bin_fixture")
    ///     .current_release()
    ///     .current_target()
    ///     .run()
    ///     .unwrap();
    /// println!("artifact={}", run.path().display());
    /// ```
    /// or
    /// ```rust
    /// let run = escargot::CargoBuild::new()
    ///     .example("example_fixture")
    ///     .current_release()
    ///     .current_target()
    ///     .run()
    ///     .unwrap();
    /// println!("artifact={}", run.path().display());
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    pub fn path(&self) -> &path::Path {
        &self.bin_path
    }

    /// Run the build artifact.
    pub fn command(&self) -> process::Command {
        let mut cmd = process::Command::new(self.path());
        cmd.arg("-Z unstable-options").arg("--format json");
        cmd
    }

    /// Run the configured test, returning test events.
    pub fn exec(&self) -> CargoResult<CommandMessages> {
        CommandMessages::with_command(self.command())
    }
}
