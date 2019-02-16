//! Serialization formats for cargo messages.

use std::borrow;
use std::path;

pub mod diagnostic;

#[cfg(feature = "test_unstable")]
pub mod test;

type CowPath<'a> = borrow::Cow<'a, path::Path>;
type CowStr<'a> = borrow::Cow<'a, str>;

/// A cargo message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
pub enum Message<'a> {
    /// The compiler generated an artifact
    #[serde(borrow)]
    CompilerArtifact(Artifact<'a>),
    /// The compiler wants to display a message
    #[serde(borrow)]
    CompilerMessage(FromCompiler<'a>),
    /// A build script successfully executed.
    #[serde(borrow)]
    BuildScriptExecuted(BuildScript<'a>),
    #[cfg(not(feature = "strict_unstable"))]
    #[doc(hidden)]
    #[serde(other)]
    Unknown,
}

/// A compiler-generated file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_unstable", serde(deny_unknown_fields))]
pub struct Artifact<'a> {
    /// The workspace member this artifact belongs to
    #[serde(borrow)]
    pub package_id: WorkspaceMember<'a>,
    /// The target this artifact was compiled for
    #[serde(borrow)]
    pub target: Target<'a>,
    /// The profile this artifact was compiled with
    #[serde(borrow)]
    pub profile: ArtifactProfile<'a>,
    /// The enabled features for this artifact
    #[serde(borrow)]
    pub features: Vec<CowStr<'a>>,
    /// The full paths to the generated artifacts
    #[serde(borrow)]
    pub filenames: Vec<CowPath<'a>>,
    /// If true, then the files were already generated
    pub fresh: bool,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// A single target (lib, bin, example, ...) provided by a crate
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "strict_unstable", serde(deny_unknown_fields))]
pub struct Target<'a> {
    /// Name as given in the `Cargo.toml` or generated from the file name
    #[serde(borrow)]
    pub name: CowStr<'a>,
    /// Kind of target ("bin", "example", "test", "bench", "lib")
    #[serde(borrow)]
    pub kind: Vec<CowStr<'a>>,
    /// Almost the same as `kind`, except when an example is a library instad of an executable.
    /// In that case `crate_types` contains things like `rlib` and `dylib` while `kind` is `example`
    #[serde(default)]
    #[serde(borrow)]
    pub crate_types: Vec<CowStr<'a>>,

    #[serde(default)]
    #[serde(rename = "required-features")]
    /// This target is built only if these features are enabled.
    /// It doesn't apply to `lib` targets.
    #[serde(borrow)]
    pub required_features: Vec<CowStr<'a>>,
    /// Path to the main source file of the target
    #[serde(borrow)]
    pub src_path: CowPath<'a>,
    /// Rust edition for this target
    #[serde(default = "edition_default")]
    #[serde(borrow)]
    pub edition: CowStr<'a>,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

fn edition_default() -> CowStr<'static> {
    "2015".into()
}

/// A workspace member. This is basically identical to `cargo::core::package_id::PackageId`, except
/// that this does not use `Arc` internally.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "strict_unstable", serde(deny_unknown_fields))]
pub struct WorkspaceMember<'a> {
    /// The raw package id as given by cargo
    #[serde(borrow)]
    raw: CowStr<'a>,
}

/// Profile settings used to determine which compiler flags to use for a
/// target.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_unstable", serde(deny_unknown_fields))]
pub struct ArtifactProfile<'a> {
    /// Optimization level. Possible values are 0-3, s or z.
    #[serde(borrow)]
    pub opt_level: CowStr<'a>,
    /// The amount of debug info. 0 for none, 1 for limited, 2 for full
    pub debuginfo: Option<u32>,
    /// State of the `cfg(debug_assertions)` directive, enabling macros like
    /// `debug_assert!`
    pub debug_assertions: bool,
    /// State of the overflow checks.
    pub overflow_checks: bool,
    /// Whether this profile is a test
    pub test: bool,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// Message left by the compiler
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_unstable", serde(deny_unknown_fields))]
pub struct FromCompiler<'a> {
    /// The workspace member this message belongs to
    #[serde(borrow)]
    pub package_id: WorkspaceMember<'a>,
    /// The target this message is aimed at
    #[serde(borrow)]
    pub target: Target<'a>,
    /// The message the compiler sent.
    #[serde(borrow)]
    pub message: diagnostic::Diagnostic<'a>,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}

/// Output of a Build Script execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_unstable", serde(deny_unknown_fields))]
pub struct BuildScript<'a> {
    /// The workspace member this build script execution belongs to
    #[serde(borrow)]
    pub package_id: WorkspaceMember<'a>,
    /// The libs to link
    #[serde(borrow)]
    pub linked_libs: Vec<CowStr<'a>>,
    /// The paths to search when resolving libs
    #[serde(borrow)]
    pub linked_paths: Vec<CowPath<'a>>,
    /// The paths to search when resolving libs
    #[serde(borrow)]
    pub cfgs: Vec<CowPath<'a>>,
    /// The environment variables to add to the compilation
    #[serde(borrow)]
    pub env: Vec<(CowStr<'a>, CowStr<'a>)>,
    #[doc(hidden)]
    #[serde(skip)]
    __do_not_match_exhaustively: (),
}
