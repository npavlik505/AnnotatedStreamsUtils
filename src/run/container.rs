use crate::prelude::*;

use super::create_dirs;
use super::postprocess;

/// running routine for the solver once activated within the container
pub(crate) fn run_container(args: cli::RunContainer) -> anyhow::Result<()> {
    let start = std::time::Instant::now();

    let path = PathBuf::from("/input/input.json");
    let dist_save = PathBuf::from("/distribute_save");
    let input_dat = PathBuf::from("/input/input.dat");

    // initialize some base directories within the folder we will work in
    create_dirs(&dist_save)?;

    // copy input.json to the output
    fs::copy(&path, "/distribute_save/input.json").unwrap();
    fs::copy("/input/database_bl.dat", "/distribute_save/database_bl.dat").unwrap();

    // read in the config json file
    let file = fs::File::open(&path)
        .with_context(|| format!("failed to open input.json file at {}", path.display()))?;

    let config: Config = serde_json::from_reader(file)?;

    // change the current working directory to the distribute_save directory. That way, all the
    // file that we need to run and work with will be output here
    let target_dir = PathBuf::from("/distribute_save");
    std::env::set_current_dir(&target_dir).with_context(|| {
        format!(
            "could not change current working directory to {}",
            target_dir.display()
        )
    })?;

    // then, generate the actual config for an output to the solver
    crate::config_generator::_config_generator(&config, input_dat)?;

    //
    // setup shell environment and run the solver
    //

    let sh = xshell::Shell::new()?;

    let nproc = args.nproc.to_string();
    let exec = xshell::cmd!(sh, "mpirun -np {nproc} /streams.exe");

    let output = exec
        .output()
        .with_context(|| "execution of solver failed and no output is available")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}\n\nSTDERR:\n{}", stdout, stderr);

    postprocess(&config)?;

    let end = start.elapsed();
    let hours = end.as_secs() / 3600;
    let minutes = (end.as_secs() / 60) - (hours * 60);
    let seconds = end.as_secs() - (hours * 3600) - (minutes * 60);
    println!(
        "runtime information (hhhh:mm:ss): {:04}:{:02}:{02}",
        hours, minutes, seconds
    );

    Ok(())
}
