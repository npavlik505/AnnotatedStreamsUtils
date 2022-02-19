use crate::prelude::*;

/// running routine for the solver once activated within the container
pub(crate) fn run(args: cli::RunSolver) -> Result<(), Error> {
    let path = PathBuf::from("/input/input.json");

    // read in the config json file
    let file = fs::File::open(&path)
        .map_err(|e| FileError::new(path, e))?;
    let mut config : cli::ConfigGenerator = serde_json::from_reader(file)?;
    config.output_path = PathBuf::from("/distribute_save/input.dat");

    // change the current working directory to the distribute_save directory. That way, all the
    // file that we need to run and work with will be output here
    let target_dir = PathBuf::from("/distribute_save");
    std::env::set_current_dir(&target_dir)
        .map_err(|e| FileError::new(target_dir, e))?;

    // then, generate the actual config for an output to the solver
    crate::config_generator::config_generator(config)?;
    todo!()
}
