use crate::{cli, ops::forc_build};
use clap::Parser;
use forc_util::ForcResult;

forc_util::cli_examples! {
    [ Compile the current project => forc "build" => r#".*could not find `Forc.toml`.*"# ]
    [ Compile the current project with a different path => forc "build --path ../tests/" => r#".*could not find `Forc.toml`.*"# ]
    [ Compile the current project without updating dependencies => forc "build --locked" => r#".*could not find `Forc.toml`.*"# ]
}

/// Compile the current or target project.
///
/// The output produced will depend on the project's program type.
///
/// - `script`, `predicate` and `contract` projects will produce their bytecode in binary format `<project-name>.bin`.
///
/// - `script` projects will also produce a file containing the hash of the bytecode binary
/// `<project-name>-bin-hash` (using `fuel_cypto::Hasher`).
///
/// - `predicate` projects will also produce a file containing the **root** hash of the bytecode binary
/// `<project-name>-bin-root` (using `fuel_tx::Contract::root_from_code`).
///
/// - `contract` and `library` projects will also produce the public ABI in JSON format
/// `<project-name>-abi.json`.
#[derive(Debug, Default, Parser)]
#[clap(bin_name = "forc build", version, after_help = help())]
pub struct Command {
    #[clap(flatten)]
    pub build: cli::shared::Build,
    /// Also build all tests within the project.
    #[clap(long)]
    pub tests: bool,
}

pub(crate) fn exec(command: Command) -> ForcResult<()> {
    forc_build::build(command)?;
    Ok(())
}
