use std::path;
use std::process;

use error::*;
use format;
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
/// Relevant features
/// - `print` for logged output to be printed instead, generally for test writing.
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
    bin_path: path::PathBuf,
}

impl CargoRun {
    pub(crate) fn with_messages(
        msgs: MessageIter,
        is_bin: bool,
        is_example: bool,
    ) -> CargoResult<Self> {
        let kind = match (is_bin, is_example) {
            (true, true) => {
                return Err(CargoError::new(ErrorKind::CommandFailed)
                    .set_context("Ambiguous which binary is intended, multiple selected"));
            }
            (false, true) => "example",
            _ => "bin",
        };
        let bin_path = extract_binary_path(msgs, kind)?;
        Ok(Self { bin_path })
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
        process::Command::new(self.path())
    }
}

fn extract_filenames<'a>(msg: &'a format::Message, desired_kind: &str) -> Option<&'a path::Path> {
    match msg {
        format::Message::CompilerArtifact(art) => {
            if art.target.crate_types == ["bin"] && art.target.kind == [desired_kind] {
                Some(art.filenames.iter().next().expect("files must exist"))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(not(feature = "print"))]
fn log_message(msg: &format::Message) {
    match msg {
        format::Message::CompilerArtifact(ref art) => {
            trace!(
                "Building {} {}",
                art.package_id.name(),
                art.package_id.version()
            );
        }
        format::Message::CompilerMessage(ref comp) => {
            let content = comp
                .message
                .rendered
                .as_ref()
                .map(|s| s.as_ref())
                .unwrap_or(comp.message.message.as_ref());
            match comp.message.level {
                format::diagnostic::DiagnosticLevel::Ice => error!("{}", content),
                format::diagnostic::DiagnosticLevel::Error => error!("{}", content),
                format::diagnostic::DiagnosticLevel::Warning => warn!("{}", content),
                format::diagnostic::DiagnosticLevel::Note => info!("{}", content),
                format::diagnostic::DiagnosticLevel::Help => info!("{}", content),
                #[cfg(not(feature = "strict_unstable"))]
                _ => warn!("Unknown message: {:#?}", msg),
            }
        }
        format::Message::BuildScriptExecuted(ref script) => {
            trace!(
                "Ran script from {} {}",
                script.package_id.name(),
                script.package_id.version()
            );
        }
        #[cfg(not(feature = "strict_unstable"))]
        _ => {
            warn!("Unknown message: {:#?}", msg);
        }
    }
}

#[cfg(feature = "print")]
fn log_message(msg: &format::Message) {
    match msg {
        format::Message::CompilerArtifact(ref art) => {
            println!(
                "Building {} {}",
                art.package_id.name(),
                art.package_id.version()
            );
        }
        format::Message::CompilerMessage(ref comp) => {
            let content = comp
                .message
                .rendered
                .as_ref()
                .map(|s| s.as_ref())
                .unwrap_or(comp.message.message.as_ref());
            match comp.message.level {
                format::diagnostic::DiagnosticLevel::Ice => println!("{}", content),
                format::diagnostic::DiagnosticLevel::Error => println!("{}", content),
                format::diagnostic::DiagnosticLevel::Warning => println!("{}", content),
                format::diagnostic::DiagnosticLevel::Note => println!("{}", content),
                format::diagnostic::DiagnosticLevel::Help => println!("{}", content),
                #[cfg(not(feature = "strict_unstable"))]
                _ => warn!("Unknown message: {:#?}", msg),
            }
        }
        format::Message::BuildScriptExecuted(ref script) => {
            println!(
                "Ran script from {} {}",
                script.package_id.name(),
                script.package_id.version()
            );
        }
        #[cfg(not(feature = "strict_unstable"))]
        _ => {
            println!("Unknown message: {:#?}", msg);
        }
    }
}

fn extract_binary_path(msgs: MessageIter, kind: &str) -> Result<path::PathBuf, CargoError> {
    let mut bins = Vec::with_capacity(1);
    for msg in msgs {
        let msg = msg?;
        let msg = msg.decode()?;
        log_message(&msg);
        if let Some(path) = extract_filenames(&msg, kind) {
            bins.push(path.to_path_buf());
        }
    }
    if bins.is_empty() {
        return Err(CargoError::new(ErrorKind::CommandFailed).set_context("No binaries in crate"));
    } else if bins.len() != 1 {
        return Err(CargoError::new(ErrorKind::CommandFailed)
            .set_context(format!("Ambiguous which binary is intended: {:?}", bins)));
    }
    Ok(bins.into_iter().next().expect("already validated"))
}
