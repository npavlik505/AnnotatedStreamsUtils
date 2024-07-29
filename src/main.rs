// Declares the modules to be used in main.rs. The compiler searches src for these modules. 
// If a folder is declared, compiler searches in folder for mod.rs (e.g ./run/mod.rs) 
mod animate;
mod binary_to_vtk;
mod cases;
mod cli;
mod config_generator;
mod hdf5_to_vtk;
mod prelude;
mod probe;
mod probe_binary;
mod run;
mod spans_to_vtk;
mod utils;
mod vtk_to_mat;

// Imports everything from the prelude module, the Rust standard library.
use prelude::*;

// clap is a crate used for parsing command line arguements and is an external dependency specified in the cargo.toml file
// Note: clap can match command line arguments with rust enum variants or struct fields and is very flexible in how it does this...
// e.g. The command line argument --run-local can matched with RunLocal despite different capitilization and dashes.   
use clap::Parser;
// Brings Args and Command from cli into scope, which allows them to be used in this module
use cli::Args; // Args is a struct in cli.rs that contains code to interpret the command line interface (cli)
use cli::Command; // Command is an enum in cli that has variants Cases through Animate (seen in the main() function below)

// The main function which returns TBD, relates to handling errors
fn main() -> anyhow::Result<()> {

    // This is critical. Args is a struct in cli.rs with a field called Command, an enum variant. The enum variant that matches the terminal command...
    // is selected using rust's match trait. The selected enum variant has a nested struct that may also be populated by the terminal command...
    // such as in the case of running ConfigGenerator.
    // The end result: args is a struct with a field called mode which has the value of a variant of the Command enum that contains a struct with all the terminal subcommands from the justfile  
    let args = Args::parse();

    // Uses mathing. Whatever matches Command, from args.mode, is run. This executes a corresponding function from an imported module ...=> module::function(x) 
    match args.mode {
        Command::Cases(x) => cases::cases(x)?,
        Command::ConfigGenerator(x) => config_generator::config_generator(x)?,
        Command::RunContainer(x) => run::run_container(x)?,
        Command::RunLocal(x) => run::run_local(x)?, //Note: run is a folder therefore rust searches for mod.rs in specified folder (./src/run/mod.rs)
        Command::Probe(x) => probe::probe(x)?,
        Command::VtkToMat(x) => vtk_to_mat::vtk_to_mat(x)?,
        Command::SpansToVtk(x) => spans_to_vtk::spans_to_vtk(x)?,
        Command::HDF5ToVtk(x) => hdf5_to_vtk::hdf5_to_vtk(x)?,
        Command::Animate(x) => animate::animate(x)?,
    };

    Ok(())
}

#[derive(thiserror::Error, Debug, From)]
enum Error {
    #[error("{0}")]
    File(FileError),
    #[error("{0}")]
    Config(config_generator::ConfigError),
    #[error("{0}")]
    Sbli(cases::SbliError),
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
