use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

/// utilities for working with the streams solver
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub(crate) mode: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Command {
    /// run case generation for SBLI cases
    Sbli(SbliCases),
    /// generate a config file (input.dat) for use in the solver
    ConfigGenerator(ConfigGenerator),
}

#[derive(Parser, Debug, Clone)]
/// Fields that are configurable for generating input.dat files for the solver
pub(crate) struct ConfigGenerator {
    /// path to write the resulting config file to
    pub(crate) output_path: PathBuf,

    /// (friction) Reynolds number (Reynolds in input file)
    #[clap(long, default_value_t = 250.0)]
    pub(crate) reynolds_number: f64,

    /// Mach number (Mach in input file, rm in code)
    #[clap(long, default_value_t = 2.28)]
    pub(crate) mach_number: f64,

    /// Shock angle (degrees) (deflec_shock in input file)
    #[clap(long, default_value_t = 8.0)]
    pub(crate) shock_angle: f64,

    /// total length in the x direction
    #[clap(long, default_value_t = 30.)]
    pub(crate) x_length: f64,

    /// total length in the x direction
    #[clap(long, default_value_t = 900)]
    pub(crate) x_divisions: usize,

    /// total length in the y direction
    #[clap(long, default_value_t = 6.)]
    pub(crate) y_length: f64,

    /// total length in the y direction
    #[clap(long, default_value_t = 205)]
    pub(crate) y_divisions: usize,

    /// total length in the z direction
    #[clap(long, default_value_t = 3.8)]
    pub(crate) z_length: f64,

    /// total length in the z direction
    #[clap(long, default_value_t = 150)]
    pub(crate) z_divisions: usize,

    /// number of MPI divisions along the x axis. The config generated
    /// will have 1 mpi division along the z axis as some extensions
    /// to the code assume there are no z divisions.
    ///
    /// The value supplied to this argument MUST be used for the -np 
    /// argument in `mpirun`
    #[clap(long, default_value_t = 4)]
    pub(crate) mpi_x_split: usize,

    #[clap(long)]
    /// skip writing the actual config file
    pub(crate) dry: bool,
}

impl ConfigGenerator {
    /// create a default config to be written to a given path
    fn with_path(output_path: PathBuf) -> Self {
        // commented values in here are the default values from the solver file
        // that we are overwriting
        Self {
            output_path,
            reynolds_number: 250.0,
            mach_number: 2.28,
            shock_angle: 8.0,
            //x_length: 70.0,
            x_length: 30.0,
            //x_divisions: 2048,
            x_divisions: 900,
            //y_length: 12.,
            y_length: 6.,
            //y_divisions: 400,
            y_divisions: 200,
            //z_length: 6.5,
            z_length: 3.8,
            //z_divisions: 256,
            z_divisions: 150,
            mpi_x_split: 4,
            dry: false,
        }
    }
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct SbliCases {
    #[clap(long)]
    /// input databse file to use
    database_bl: PathBuf,
}
