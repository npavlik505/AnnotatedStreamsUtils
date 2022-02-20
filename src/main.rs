mod binary_to_vtk;
mod cli;
mod config_generator;
mod prelude;
mod run;
mod sbli;

use prelude::*;

use clap::Parser;
use cli::Args;
use cli::Command;

fn main() {
    let args = Args::parse();

    let out = match args.mode {
        Command::Sbli(x) => sbli::sbli_cases(x),
        Command::ConfigGenerator(x) => config_generator::config_generator(x),
        Command::RunSolver(x) => run::run(x),
    };

    if let Err(e) = out {
        println!("Error: {}", e);
    }
}

#[derive(thiserror::Error, Debug, From)]
enum Error {
    #[error("{0}")]
    File(FileError),
    #[error("{0}")]
    Config(config_generator::ConfigError),
    #[error("{0}")]
    Sbli(sbli::SbliError),
    #[error("{0}")]
    SerializationYaml(distribute::serde_yaml::Error),
    #[error("{0}")]
    SerializationJson(serde_json::Error),
    #[error("{0}")]
    BinaryVtkError(binary_to_vtk::BinaryToVtkError),
    #[error("{0}")]
    Vtk(vtk::Error),
}

#[derive(Display, Debug, Constructor)]
#[display(fmt = "error with file: {}. Error code: {}", "path.display()", error)]
struct FileError {
    path: PathBuf,
    error: io::Error,
}
