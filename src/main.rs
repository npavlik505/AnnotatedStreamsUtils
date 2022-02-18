mod cli;
mod config_generator;
mod prelude;
mod sbli;

use prelude::*;

use clap::Parser;
use cli::Args;
use cli::Command;

fn main() {
    let args = Args::parse();

    let out = match args.mode {
        Command::Sbli(x) => todo!(),
        Command::ConfigGenerator(x) => config_generator::config_generator(x),
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
}

#[derive(Display, Debug, Constructor)]
#[display(fmt = "error with file: {}. Error code: {}", "path.display()", error)]
struct FileError {
    path: PathBuf,
    error: io::Error,
}
