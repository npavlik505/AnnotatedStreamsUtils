mod binary_to_vtk;
mod cli;
mod config_generator;
mod prelude;
mod probe;
mod probe_binary;
mod run;
mod sbli;
mod utils;
mod vtk_to_mat;
mod spans_to_vtk;

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
        Command::Probe(x) => probe::probe(x),
        Command::VtkToMat(x) => vtk_to_mat::vtk_to_mat(x),
        Command::SpansToVtk(x) => spans_to_vtk::spans_to_vtk(x)
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
    #[error("{0}")]
    ProbeBinary(probe_binary::ProbeBinaryError),
    #[error("Could not write the file using mat5: {0}")]
    Mat5(mat5::Error),
}

#[derive(Display, Debug, Constructor)]
#[display(fmt = "error with file: {}. Error code: {}", "path.display()", error)]
struct FileError {
    path: PathBuf,
    error: io::Error,
}
