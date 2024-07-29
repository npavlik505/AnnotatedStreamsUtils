use crate::prelude::*;
pub(crate) use anyhow::Result;
// imports cmd and Shell from the xshell create. These allow shell commands to be executed within code
use xshell::{cmd, Shell};

struct Solver {
    #[allow(dead_code)]
    working_dir: PathBuf,
    input: PathBuf,
    dist_save: PathBuf,
}

impl Solver {
    fn new(working_dir: PathBuf) -> Result<Self> {
        //
        // create folders for input
        // working_dir contains the path to the working directory and .join("input") adds input to that path.
        // If a directory already exists at that path, the command deletes it.
        let input = working_dir.join("input");
        if input.exists() {
            std::fs::remove_dir_all(&input).with_context(|| {
                format!(
                    "failed to remove full directory at {} for input",
                    input.display()
                )
            })?;
        }

        // Creates a directory at input
        std::fs::create_dir(&input).with_context(|| {
            format!(
                "failed to create directory at {} for input",
                input.display()
            )
        })?;

        //
        // create folders for distribute save
        // Similar to input (directly above), these 15 lines ensures dist_save is assigned a path to an empty directory
        let dist_save = working_dir.join("distribute_save");
        if dist_save.exists() {
            std::fs::remove_dir_all(&dist_save).with_context(|| {
                format!(
                    "failed to remove full directory at {} for distribute save",
                    dist_save.display()
                )
            })?;
        }
        std::fs::create_dir(&dist_save).with_context(|| {
            format!(
                "failed to create directory at {} for distribute save",
                dist_save.display()
            )
        })?;

        // The variable s is assigned the contents of the Solver struct
        let s = Solver {
            working_dir,
            input,
            dist_save,
        };

        Ok(s)
    }

    fn load_input_file(&self, host_path: &Path, container_name: &str) -> Result<()> {
        // Adds the container_name parameter to the path to input.
        // As seen in the run_local function below this creates ./input/input.json and ./input/database_bl.dat path 
        let container_path = self.input.join(container_name);
        // Copies the file specified by host_path to container_path location
        // As seen in run_local this actually adds the input.json and database_bl.dat to the paths just created above...
        // which are neede to run streams 
        std::fs::copy(host_path, &container_path).with_context(|| {
            format!(
                "failed to copy {} to {}",
                host_path.display(),
                container_path.display()
            )
        })?;

        Ok(())
    }

    // 
    fn run(&self, nproc: usize, python_mount: String) -> Result<()> {
        // Creates a new shell
        let sh = Shell::new()?;

        // assigns the dist_save path to results_path, which will hold the data produced by streams,...
        // and assigns input path, which contains the input.json and database_bl.dat necessary to run streams, to input_path. 
        let results_path = &self.dist_save;
        let input_path = &self.input;
        // Turns the nproc type, which will be provided as a paramter to the run_local function, to string type
        // Note: nproc, found in cli.rs, is the number of processors the program is allowed to use
        let nproc = nproc.to_string();

        // runs the shell command "apptainer run ..."
        // --nv runs the apptainer on GPU.  --bind provides files from the host computer, located at {input_path},...
        // to the apptainer at /input, if {python_mount} is specified it is passed to the apptainer as well,...
        // and the results located in distribute_save on the apptainer are passed to {results_path} on the host computer.
        // Lastly, note that distribute, found in the %apprun section streams.sif, executes a shell command that passes "run-container" back to main.rs 
        let exec = cmd!(sh, "apptainer run --nv --bind {results_path}:/distribute_save,{input_path}:/input{python_mount} --app distribute ./streams.sif {nproc}")
            // ignore the output status so we get more STDOUT information?
            .ignore_status();

        exec.run()?;

        Ok(())
    }
}

// The run_local command that is called from mod.rs
pub(crate) fn run_local(args: cli::RunLocal) -> Result<()> {
    // sif_file is assigned the path to streams.sif, the container housing the STREAmS solver
    let sif_file = PathBuf::from("./streams.sif");

    // If the streams.sif isn't found the following error will display
    if !sif_file.exists() {
        anyhow::bail!("streams.sif does not exist in the current directory. Are you sure you are running from the ./streams-utils folder");
    }

    // Using new() from an implementation of the Solver struct, defined earlier in this module, creates empty folders in the necessary directories
    let solver = Solver::new(args.workdir)?;

    // Using load_input_path() from an implementation of the Solver struct, populates the newly created input folder with the appropriate files (i.e. the json file and database.bl file)
    solver.load_input_file(&args.config, "input.json")?;
    solver.load_input_file(&args.database, "database_bl.dat")?;

    // if a directory was specified to run the solver then we format it to a binding for the
    // `apptainer run` command, otherwise an empty string will not change the output
    let python_mount = if let Some(mount_path) = args.python_mount {
        format!(
            ",{}:/runtimesolver",
            mount_path.to_string_lossy().into_owned()
        )
    } else {
        "".to_string()
    };

    // 
    solver.run(args.nproc, python_mount)?;

    Ok(())
}
