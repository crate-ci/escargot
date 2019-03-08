use std::path;
use std::process;

use error::*;
use format;
use msg::*;

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
/// extern crate escargot;
/// extern crate assert_fs;
///
/// let temp = assert_fs::TempDir::new().unwrap();
/// let run = escargot::CargoBuild::new()
///     .test("test")
///     .manifest_path("tests/fixtures/test/Cargo.toml")
///     .target_dir(temp.path())
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
        extract_binary_paths(msgs).map(|p| p.map(|p| Self { bin_path: p }))
    }

    /// Path to the specified binary.
    ///
    /// This is to support alternative ways of launching the binary besides [`Command`].
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate escargot;
    /// extern crate assert_fs;
    ///
    /// let temp = assert_fs::TempDir::new().unwrap();
    /// let run: Vec<_> = escargot::CargoBuild::new()
    ///     .tests()
    ///     .current_release()
    ///     .current_target()
    ///     .manifest_path("tests/fixtures/test/Cargo.toml")
    ///     .target_dir(temp.path())
    ///     .run_tests()
    ///     .unwrap()
    ///     .collect();
    /// assert_eq!(run.len(), 3);
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    pub fn path(&self) -> &path::Path {
        &self.bin_path
    }

    /// Run the build artifact.
    pub fn command(&self) -> process::Command {
        let mut cmd = process::Command::new(self.path());
        cmd.arg("-Z").arg("unstable-options").arg("--format=json");
        cmd
    }

    /// Run the configured test, returning test events.
    pub fn exec(&self) -> CargoResult<CommandMessages> {
        CommandMessages::with_command(self.command())
    }
}

fn extract_bin<'a>(msg: &'a format::Message) -> Option<&'a path::Path> {
    match msg {
        format::Message::CompilerArtifact(art) => {
            if art.profile.test {
                Some(art.filenames.iter().next().expect("files must exist"))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn transpose<T, E>(r: Result<Option<T>, E>) -> Option<Result<T, E>> {
    match r {
        Ok(Some(x)) => Some(Ok(x)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    }
}

fn extract_binary_paths(
    msgs: CommandMessages,
) -> impl Iterator<Item = Result<path::PathBuf, CargoError>> {
    msgs.filter_map(move |m| {
        let m = m.and_then(|m| {
            let m = m.decode()?;
            format::log_message(&m);
            let p = extract_bin(&m).map(|p| p.to_path_buf());
            Ok(p)
        });
        transpose(m)
    })
}
