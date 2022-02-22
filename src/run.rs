use crate::prelude::*;

/// running routine for the solver once activated within the container
pub(crate) fn run(args: cli::RunSolver) -> Result<(), Error> {
    let start = std::time::Instant::now();

    let path = PathBuf::from("/input/input.json");
    let dist_save = PathBuf::from("/distribute_save");

    create_dirs(&dist_save)?;

    // copy input.json to the output
    fs::copy(&path, "/distribute_save/input.json").unwrap();
    fs::copy("/input/database_bl.dat", "/distribute_save/database_bl.dat").unwrap();

    // read in the config json file
    let file = fs::File::open(&path).map_err(|e| FileError::new(path, e))?;
    let mut config: cli::ConfigGenerator = serde_json::from_reader(file)?;
    config.output_path = PathBuf::from("/distribute_save/input.dat");

    // change the current working directory to the distribute_save directory. That way, all the
    // file that we need to run and work with will be output here
    let target_dir = PathBuf::from("/distribute_save");
    std::env::set_current_dir(&target_dir).map_err(|e| FileError::new(target_dir, e))?;

    // then, generate the actual config for an output to the solver
    crate::config_generator::config_generator(config.clone())?;

    // mpirun -np $NPROC /streams.exe
    let output = std::process::Command::new("mpirun")
        .arg("-np")
        .arg(args.nproc.to_string())
        .arg("/streams.exe")
        .output()
        .map_err(|e| FileError::new(PathBuf::from("/streams.exe"), e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}\n\nSTDERR:\n{}", stdout, stderr);

    postprocess(&args, &config)?;

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

/// create all the folders that data is written to in the solver
fn create_dirs(base: &Path) -> Result<(), FileError> {
    let csv = base.join("csv_data");
    fs::create_dir(&csv).map_err(|e| FileError::new(csv, e))?;

    let spans = base.join("spans");
    fs::create_dir(&spans).map_err(|e| FileError::new(spans, e))?;

    Ok(())
}

fn read_mesh_info(path: &Path, ghost_nodes: usize, values: usize) -> Result<Vec<f64>, FileError> {
    let file_data = fs::read_to_string(path).map_err(|e| FileError::new(path.to_path_buf(), e))?;

    let data = file_data
        .split('\n')
        .into_iter()
        .map(|row| row.trim().parse())
        .take_while(|x| x.is_ok())
        .map(|x| x.unwrap())
        .skip(ghost_nodes)
        .take(values)
        .collect();

    Ok(data)
}

/// information on the meshing `dx` `dy` `dz` from the streams output files
struct MeshInfo {
    x_data: Vec<f64>,
    y_data: Vec<f64>,
    z_data: Vec<f64>,
}

impl MeshInfo {
    fn from_base_path(base: &Path, config: &cli::ConfigGenerator) -> Result<Self, FileError> {
        let xg = base.join("x.dat");
        let yg = base.join("y.dat");
        let zg = base.join("z.dat");

        // TODO: update this in the config generation - but i doubt it will ever change
        let ghost_nodes = 3;

        let x_data = read_mesh_info(&xg, ghost_nodes, config.x_divisions)?;
        let y_data = read_mesh_info(&yg, ghost_nodes, config.y_divisions)?;
        let z_data = read_mesh_info(&zg, ghost_nodes, config.z_divisions)?;

        Ok(Self {
            x_data,
            y_data,
            z_data,
        })
    }
}

/// general parent postprocessing routine to be called after the solver has finished
fn postprocess(run_args: &cli::RunSolver, config: &cli::ConfigGenerator) -> Result<(), Error> {
    let data_location = PathBuf::from("/distribute_save");
    let mesh_info = MeshInfo::from_base_path(&data_location, config)?;

    // convert all the binary spans to vtk files
    convert_spans(&data_location, config, &mesh_info)?;

    Ok(())
}

/// Convert all .binary files in the ./spans directory to Vtk files using mesh information
fn convert_spans(
    data_location: &Path,
    config: &cli::ConfigGenerator,
    mesh_info: &MeshInfo,
) -> Result<(), Error> {
    let spans_folder = data_location.join("spans");

    let locations = vtk::Locations {
        x_locations: mesh_info.x_data.clone(),
        y_locations: mesh_info.y_data.clone(),
        z_locations: vec![0.0],
    };
    let spans = vtk::LocationSpans {
        x_start: 1,
        x_end: config.x_divisions,
        y_start: 1,
        y_end: config.y_divisions,
        z_start: 1,
        z_end: 1,
    };

    for file in walkdir::WalkDir::new(&spans_folder)
        .into_iter()
        .filter_map(|e| e.ok())
        // the first item will be the root folder we created
        // this makes sure we skip any item that is a directory
        .filter(|e| e.file_type().is_file())
    {
        let path = file.path();
        let file_name = path.file_stem().unwrap().to_string_lossy();
        let output_name = format!("{}.vtr", file_name);
        let output_path = spans_folder.join(output_name);

        // read the data to something we can write a vtk with

        let mut file = fs::File::open(path).map_err(|e| FileError::new(path.to_owned(), e))?;

        let mut buffer = Vec::with_capacity(8 * config.x_divisions * config.y_divisions * 5);
        file.read_to_end(&mut buffer).unwrap();
        let float_bytes = binary_to_vtk::bytes_to_float(&buffer);

        let data = binary_to_vtk::convert_binary_to_vtk_information(&float_bytes, config)?;

        let vtk = vtk::VtkData {
            data,
            locations: locations.clone(),
            spans: spans.clone(),
        };
        let writer = io::BufWriter::new(
            fs::File::create(&output_path)
                .map_err(|e| FileError::new(output_path.to_owned(), e))?,
        );
        vtk::write_vtk(writer, vtk, true)?;

        fs::remove_file(path).unwrap()
    }

    Ok(())
}

#[test]
fn read_mesh_file() {
    let file = PathBuf::from("./static/x.dat");
    assert_eq!(file.exists(), true);

    let ghost = 3;
    let nx = 840;

    let x = read_mesh_info(&file, ghost, nx).unwrap();
    dbg!(&x);
    assert_eq!(x.len(), 840);
}
