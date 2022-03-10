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
pub(crate) struct CopyFile {
    source: PathBuf,
    dest: PathBuf,
    error: io::Error,
}

pub(crate) fn sbli_cases(mut args: SbliCases) -> Result<(), Error> {
    check_options_copy_files(&mut args)?;
    // remove mutability on args
    let args = args;

    match args.mode {
        cli::SbliMode::Sweep => sweep_cases(args)?,
        cli::SbliMode::CheckBlowingCondition => check_blowing_condition(args)?,
        cli::SbliMode::CheckProbes=> check_probes(args)?
    };


    Ok(())
}

/// helper function to add a bunch of cases to the cases vector
/// with two simple callbacks (one to create the name, one to update
/// the value in the config that is desired)
fn create_cases<T, V, Val>(
    create_case_name: T,
    update_config: V,
    cases: &mut Vec<cli::ConfigGenerator>,
    values: &[Val],
    output_directory: &Path,
) where
    T: Fn(usize, &Val) -> String,
    V: Fn(&mut cli::ConfigGenerator, Val),
    Val: Copy,
{
    let steps = 50_000;
    let span_average_steps = 5;
    let blowing_bc = 0;

    for (idx, update_value) in values.into_iter().enumerate() {
        //let case_name = format!("reynolds_number_{idx}.json");
        let case_name = create_case_name(idx, update_value);
        let case_path = output_directory.join(case_name);
        let mut config = cli::ConfigGenerator::with_path(case_path);
        update_config(&mut config, *update_value);
        config.steps = steps;
        config.span_average_io_steps = span_average_steps;
        config.sbli_blowing_bc = blowing_bc;
        cases.push(config)
    }
}

/// verify the cli options passed are valid
///
/// this includes the file paths are valid, and whether or not to canonicalize the paths
fn check_options_copy_files(args: &mut cli::SbliCases) -> Result<(), Error> {
    // if we are not copying over the .sif file (it takes up lots of space)
    // then lets make sure that the path specified is global and not relative
    if !args.copy_sif {
        args.solver_sif = args
            .solver_sif
            .canonicalize()
            .map_err(|e| FileError::new(args.solver_sif.clone(), e))?;
    }

    if !args.database_bl.exists() {
        return Err(SbliError::DatabaseBlMissing(args.database_bl.clone()).into());
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
    fs::copy(&args.database_bl, &dest_dir)
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

/// generate a sweep over combinations of shock angles and mach numbers
fn sweep_cases(args: SbliCases) -> Result<(), Error> {
    // angle of the shock (degrees)
    let shock_angle = [6., 8., 10.];

    // mach numbers (rm)
    let mach_numbers = [2., 2.25, 2.5];

    let mut permutations = Vec::new();
    let mut cases = Vec::new();

    for angle in shock_angle {
        for mach in mach_numbers {
            permutations.push((angle, mach));
        }
    }

    let path_format = |idx, values: &(f64, f64)| {
        let (shock_angle, mach_number) = values;
        format!("shock_{}_mach_{}.json", shock_angle, mach_number)
    };

    let adj_value = |config: &mut cli::ConfigGenerator, (angle, mach)| {
        config.mach_number = mach;
        config.shock_angle = angle;
    };

    create_cases(
        path_format,
        adj_value,
        &mut cases,
        &permutations,
        &args.output_directory,
    );

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

/// validate that the blowing boundary condition on the bottom plate of the 
/// simulation is working correctly
fn check_blowing_condition(args: SbliCases) -> Result<(), Error> {
    let mut case = cli::ConfigGenerator::with_path(args.output_directory.join("check_blowing_condition.json"));
    
    case.steps = 50_000;
    case.sbli_blowing_bc = 1;

    let gpu_memory = Some(crate::config_generator::Megabytes(11 * 10usize.pow(3)));
    case.validate(gpu_memory)?;

    let file = fs::File::create(&case.output_path)
        .map_err(|e| FileError::new(case.output_path.clone(), e))?;

    // write the case data to a file so that the actual input file can be generated later
    serde_json::to_writer(file, &case)?;

    distribute_gen(&args, vec![case.output_path])?;

    Ok(())
}

/// validate that the probe data is being collected as we expect it to be 
fn check_probes(args: SbliCases) -> Result<(), Error> {
    let mut case = cli::ConfigGenerator::with_path(args.output_directory.join("check_probes.json"));
    
    case.steps = 100;
    case.probe_io_steps = 1;
    case.span_average_io_steps = 0;

    let gpu_memory = Some(crate::config_generator::Megabytes(11 * 10usize.pow(3)));
    case.validate(gpu_memory)?;

    let file = fs::File::create(&case.output_path)
        .map_err(|e| FileError::new(case.output_path.clone(), e))?;

    // write the case data to a file so that the actual input file can be generated later
    serde_json::to_writer(file, &case)?;

    distribute_gen(&args, vec![case.output_path])?;

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
