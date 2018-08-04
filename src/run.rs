use std::path;
use std::process;

use error::*;
use msg::*;

/// The `run` subcommand (emulated).
pub struct CargoRun {
    bin: path::PathBuf,
}

impl CargoRun {
    pub(crate) fn with_messages(msgs: MessageItr) -> CargoResult<Self> {
        let bin = extract_binary_path(msgs)?;
        Ok(Self { bin })
    }

    pub fn path(&self) -> &path::Path {
        &self.bin
    }

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
