use std::path;
use std::process;

use error::*;
use msg::*;

/// The `run` subcommand (emulated).
///
/// Created via [`CargoBuild::run`].
///
/// Benefits over spawning `cargo run`:
/// - Able to cache binary path, avoiding cargo overhead.
/// - Independent of CWD.
/// - stdout/stderr are clean of `cargo run` output.
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
///
/// [`CargoBuild::run`]: struct.CargoBuild.html#method.run
pub struct CargoRun {
    bin: path::PathBuf,
}

impl CargoRun {
    pub(crate) fn with_messages(msgs: MessageItr) -> CargoResult<Self> {
        let bin = extract_binary_path(msgs)?;
        Ok(Self { bin })
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
    ///
    /// [Command]: https://doc.rust-lang.org/std/process/struct.Command.html
    pub fn path(&self) -> &path::Path {
        &self.bin
    }

    /// Run the build artifact.
    pub fn command(&self) -> process::Command {
        process::Command::new(self.path())
    }
}

#[derive(Deserialize)]
struct MessageTarget<'a> {
    #[serde(borrow)]
    crate_types: Vec<&'a str>,
    #[serde(borrow)]
    kind: Vec<&'a str>,
}

#[derive(Deserialize)]
struct MessageFilter<'a> {
    #[serde(borrow)]
    reason: &'a str,
    target: MessageTarget<'a>,
    filenames: Vec<path::PathBuf>,
}

fn extract_filenames(msg: &Message, kind: &str) -> Option<path::PathBuf> {
    let filter: MessageFilter = msg.convert().ok()?;
    if filter.reason != "compiler-artifact"
        || filter.target.crate_types != ["bin"]
        || filter.target.kind != [kind]
    {
        None
    } else {
        Some(
            filter
                .filenames
                .into_iter()
                .next()
                .expect("files must exist"),
        )
    }
}

fn extract_binary_path(msgs: MessageItr) -> Result<path::PathBuf, CargoError> {
    let bins: Vec<_> = msgs.filter_map(|m| extract_filenames(&m, "bin")).collect();
    if bins.is_empty() {
        return Err(CargoError::new(ErrorKind::CommandFailed).set_context("No binaries in crate"));
    } else if bins.len() != 1 {
        return Err(CargoError::new(ErrorKind::CommandFailed)
            .set_context(format!("Ambiguous which binary is intended: {:?}", bins)));
    }
    Ok(bins.into_iter().next().expect("already validated"))
}
