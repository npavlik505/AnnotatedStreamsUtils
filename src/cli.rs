use crate::prelude::*;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
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
    /// run the solver once inside the apptainer container
    RunContainer(RunContainer),
    /// run an the apptainer solver locally
    RunLocal(RunLocal),
    /// parse probe data to .mat files
    Probe(ParseProbe),
    /// convert a span average VTK file to a .mat file for analysis
    VtkToMat(VtkToMat),
    /// convert a partial solver folder with span binaries to VTK files.
    /// usually this is performed automatically by the `run-solver` subcommand
    SpansToVtk(SpansToVtk),
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
    #[clap(long, default_value_t = 27.)]
    pub(crate) x_length: f64,

    /// total length in the x direction
    #[clap(long, default_value_t = 800)]
    pub(crate) x_divisions: usize,

    /// total length in the y direction
    #[clap(long, default_value_t = 6.)]
    pub(crate) y_length: f64,

    /// total length in the y direction
    #[clap(long, default_value_t = 208)]
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

    #[clap(long, default_value_t = 50_000)]
    /// number of steps for the solver to take
    pub(crate) steps: usize,

    #[clap(long, default_value_t = 0)]
    /// number of steps between writing probe information.
    /// (0 => never)
    /// (n >0 => every n steps)
    pub(crate) probe_io_steps: usize,

    #[clap(long, default_value_t = 100)]
    /// number of steps between span average flowfields
    /// (0 => never)
    /// (n >0 => every n steps)
    pub(crate) span_average_io_steps: usize,

    #[clap(long, default_value_t = 0)]
    /// whether or not to use blowing boundary condition on the bottom surface
    /// in the sbli case
    ///
    /// (0) => no (default BC)
    /// (1) => yes (currently no configuration for the location of the blowing)
    pub(crate) sbli_blowing_bc: usize,

    #[clap(long)]
    /// enable exporting 3D flowfields to VTK files
    ///
    /// If not present, no 3D flowfields will be written
    pub(crate) snapshots_3d: bool,

    #[clap(long)]
    /// save output to json format
    pub(crate) json: bool,

    #[clap(long)]
    /// run the solver with python bindings instead of fortran mode
    pub(crate) use_python: bool,
}

impl ConfigGenerator {
    /// create a default config to be written to a given path
    pub(crate) fn with_path(output_path: PathBuf) -> Self {
        // commented values in here are the default values from the solver file
        // that we are overwriting
        Self {
            output_path,
            reynolds_number: 250.0,
            mach_number: 2.28,
            shock_angle: 8.0,
            //x_length: 70.0,
            x_length: 27.0,
            //x_divisions: 2048,
            x_divisions: 800,
            //y_length: 12.,
            y_length: 6.,
            //y_divisions: 400,
            y_divisions: 208,
            //z_length: 6.5,
            z_length: 3.8,
            //z_divisions: 256,
            z_divisions: 150,
            mpi_x_split: 4,
            dry: false,
            steps: 50_000,
            probe_io_steps: 0,
            span_average_io_steps: 100,
            sbli_blowing_bc: 0,
            snapshots_3d: true,
            json: false,
            use_python: false
        }
    }

    pub(crate) fn into_serializable(self) -> crate::config_generator::Config {
        let Self {
            reynolds_number,
            mach_number,
            shock_angle,
            x_length,
            x_divisions,
            y_length,
            y_divisions,
            z_length,
            z_divisions,
            mpi_x_split,
            steps,
            probe_io_steps,
            span_average_io_steps,
            sbli_blowing_bc,
            snapshots_3d,
            use_python,
            ..
        } = self;

        crate::config_generator::Config {
            reynolds_number,
            mach_number,
            shock_angle,
            x_length,
            x_divisions,
            y_length,
            y_divisions,
            z_length,
            z_divisions,
            mpi_x_split,
            steps,
            probe_io_steps,
            span_average_io_steps,
            sbli_blowing_bc,
            snapshots_3d,
            use_python
        }
    }
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct SbliCases {
    #[clap(value_enum)]
    /// mode to run the case generation with
    pub(crate) mode: SbliMode,

    /// the location where all `distribute` files will
    /// be written
    pub(crate) output_directory: PathBuf,

    /// a matrix_id that you want to ping after the jobs are
    /// finished. Should look like: `@user_id:matrix.org`
    #[clap(long)]
    pub(crate) matrix: Option<distribute::OwnedUserId>,

    #[clap(long)]
    /// input databse file to use
    pub(crate) database_bl: PathBuf,

    /// path to the streams .sif file you wish to use
    /// to run this batch
    #[clap(long)]
    pub(crate) solver_sif: PathBuf,

    #[clap(long)]
    /// copy the .sif file to the output directory so
    /// that the run can be replicated later. if not
    /// passed the distribute-jobs.yaml file will reference
    /// the solver .sif file that may change at a later time
    pub(crate) copy_sif: bool,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
pub(crate) enum SbliMode {
    /// generate sweeps for reynolds number, shock angle, and mach number
    Sweep,
    /// validate the blowing boundary condition case
    CheckBlowingCondition,
    /// ensure that the probes are working properly
    CheckProbes,
    /// run a single case
    OneCase,
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct RunContainer {
    /// the number of processes that this program is allowed to use
    pub(crate) nproc: usize,
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct RunLocal {
    /// the number of processes that this program is allowed to use
    pub(crate) nproc: usize,

    #[clap(long)]
    /// working dir to run the solver in
    pub(crate) workdir: PathBuf,

    #[clap(long)]
    /// input.json file to load into the solver
    pub(crate) config: PathBuf,

    #[clap(long)]
    /// path to database.bl file required to run streams
    pub(crate) database: PathBuf,

    #[clap(long)]
    /// mount some python code into the container to run instead of the 
    /// code contained in the solver image
    pub(crate) python_mount: Option<PathBuf>,
}

#[derive(Parser, Debug, Clone, Constructor)]
pub(crate) struct ParseProbe {
    /// mode to run the case generation with
    pub(crate) probe_directory: PathBuf,

    /// location where .mat files will be written
    pub(crate) output_directory: PathBuf,

    /// config json file that was used to generate probe data
    #[clap(long)]
    pub(crate) config: PathBuf,
}

#[derive(Parser, Debug, Clone, Constructor)]
pub(crate) struct VtkToMat {
    /// all the input files to write to the output directory
    pub(crate) input_files: Vec<PathBuf>,

    #[clap(long)]
    pub(crate) config: PathBuf,

    /// .mat file that is exported
    #[clap(long)]
    pub(crate) output_file: PathBuf,
}

#[derive(Parser, Debug, Clone, Constructor)]
pub(crate) struct SpansToVtk {
    /// the path to the solver results. Should contain the input.json file, x.dat, y.dat, z.dat
    /// as well as a ./spans/ folder containing .binary files to convert
    pub(crate) solver_results: PathBuf,

    #[clap(long)]
    /// remove the old binary files after converting to
    pub(crate) clean_binary: bool,
}
