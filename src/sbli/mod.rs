use crate::prelude::*;
use cli::SbliCases;

#[derive(thiserror::Error, Debug)]
pub(crate) enum SbliError {
    #[error("The output path {} already exists. Its contents would be overwritten by this run.", .0.display())]
    OutputPathExists(PathBuf),
    #[error("The database_bl file {} does not exist", .0.display())]
    DatabaseBlMissing(PathBuf),
    #[error("{0}")]
    Copy(CopyFile),
}

#[derive(Debug, Display, Constructor)]
#[display(
    fmt = "failed to copy file from {} to {}. Error: {}",
    "source.display()",
    "dest.display()",
    error
)]
pub(crate) struct CopyFile{
    source: PathBuf,
    dest: PathBuf,
    error: io::Error
}

pub(crate) fn sbli_cases(mut args: SbliCases) -> Result<(), Error> {
    // friction reynolds numbers
    let reynolds_numbers = [200., 225., 250., 275., 300.];

    // angle of the shock (degrees)
    let shock_angle = [4., 6., 8., 10., 12.];

    // mach numbers (rm)
    let mach_numbers = [2., 2.2, 2.4, 2.6, 2.8, 3.0];

    check_options_copy_files(&mut args)?;
    // remove mutability on args
    let args = args;

    let steps = 100;
    let mut cases = vec![];

    for (idx, re) in reynolds_numbers.into_iter().enumerate() {
        let case_name = format!("reynolds_number_{idx}.json");
        let case_path = args.output_directory.join(case_name);
        let mut config = cli::ConfigGenerator::with_path(case_path);
        config.reynolds_number = re;
        config.steps = steps;
        cases.push(config)
    }

    for (idx, angle) in shock_angle.into_iter().enumerate() {
        let case_name = format!("shock_angle_{idx}.json");
        let case_path = args.output_directory.join(case_name);
        let mut config = cli::ConfigGenerator::with_path(case_path);
        config.shock_angle = angle;
        config.steps = steps;
        cases.push(config)
    }

    for (idx, mach) in mach_numbers.into_iter().enumerate() {
        let case_name = format!("mach_number_{idx}.json");
        let case_path = args.output_directory.join(case_name);
        let mut config = cli::ConfigGenerator::with_path(case_path);
        config.mach_number = mach;
        config.steps = steps;
        cases.push(config)
    }

    // pull all of the input.dat files that we need to write to a distribute file
    let input_files: Vec<PathBuf> = cases
        .iter()
        .map(|config| config.output_path.clone())
        .collect();

    for case in cases {
        // first, make sure that the case itself is valid
        let gpu_memory = Some(crate::config_generator::Megabytes(11 * 10usize.pow(3)));
        case.validate(gpu_memory)?;

        let file = fs::File::create(&case.output_path)
            .map_err(|e| FileError::new(case.output_path.clone(), e))?;

        // write the case data to a file so that the actual input file can be generated later
        serde_json::to_writer(file, &case)?;
    }

    distribute_gen(&args, input_files)?;

    Ok(())
}

/// verify the cli options passed are valid
///
/// this includes the file paths are valid, and whether or not to canonicalize the paths
fn check_options_copy_files(args: &mut cli::SbliCases) -> Result<(), Error> {
    // if we are not copying over the .sif file (it takes up lots of space)
    // then lets make sure that the path specified is global and not relative
    if !args.copy_sif {
        args.solver_sif = args.solver_sif
            .canonicalize()
            .map_err(|e| FileError::new(args.solver_sif.clone(), e))?;
    }

    if !args.database_bl.exists() {
        return Err(SbliError::DatabaseBlMissing(args.database_bl.clone()).into())
    }

    // error if the directory already exists, otherwise create the directory
    if args.output_directory.exists() {
        return Err(SbliError::OutputPathExists(args.output_directory.clone()).into());
    } else {
        fs::create_dir(&args.output_directory)
            .map_err(|e| FileError::new(args.output_directory.clone(), e))?;
    }

    // copy the database_bl file to the output folder we have created
    let dest_dir = args.output_directory.join("database_bl.dat");
    fs::copy(&args.database_bl, &dest_dir )
        .map_err(|e| CopyFile::new(args.database_bl.clone(), dest_dir.clone(), e))
        .map_err(|e| SbliError::Copy(e))?;
    args.database_bl = dest_dir;

    // copy the sif file to the output folder (if requested)
    if args.copy_sif {
        let dest_dir = args.output_directory.join("streams.sif");
        fs::copy(&args.solver_sif, &dest_dir)
            .map_err(|e| CopyFile::new(args.solver_sif.clone(), dest_dir, e))
            .map_err(|e| SbliError::Copy(e))?;
    }

    Ok(())
}

/// create a distribute-jobs.yaml file from the input configuration files
fn distribute_gen(args: &cli::SbliCases, input_files: Vec<PathBuf>) -> Result<(), Error> {
    let capabilities = vec!["gpu", "apptainer"]
        .into_iter()
        .map(|x| x.into())
        .collect();

    let batch_name = args
        .output_directory
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let meta = distribute::Meta {
        batch_name,
        namespace: "streams_sbli".into(),
        matrix: args.matrix.clone(),
        capabilities,
    };

    // initialization specification
    let mounts = vec![];
    let init = distribute::singularity::Initialize::new(
        args.solver_sif.clone(),
        vec![distribute::common::File {
            path: args.database_bl.clone(),
            alias: Some("database_bl.dat".into()),
        }],
        mounts,
    );

    let jobs = input_files
        .into_iter()
        .map(|file| {
            let job_name = file.file_stem().unwrap().to_string_lossy().to_string();
            distribute::singularity::Job::new(
                job_name,
                vec![distribute::common::File {
                    path: file,
                    alias: Some("input.json".into()),
                }],
            )
        })
        .collect();

    let singularity = distribute::singularity::Description::new(init, jobs);
    let jobs = distribute::Jobs::Singularity { meta, singularity };

    let jobs_path = args.output_directory.join("distribute-jobs.yaml");
    let file = fs::File::create(&jobs_path).map_err(|e| FileError::new(jobs_path, e))?;

    jobs.to_writer(file)?;

    Ok(())
}
