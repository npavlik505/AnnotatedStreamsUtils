use crate::prelude::*;
use vtk::DataArray;
use vtk::ParseDataArray;
use vtk::VectorPoints;

#[derive(ParseDataArray, DataArray)]
/// Information available from a span-wise average of the flowfield
pub struct SpanVtkInformation {
    rho: VectorPoints,
    velocity: VectorPoints,
    energy: VectorPoints,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum BinaryToVtkError {
    #[error("Extra data present in the binary file ({0} points)")]
    ExtraData(usize),
}

/// load information from a .binary file to a format that can be processed in paraview
///
/// The .binary files are produced in write_probe_data. They consist of span averaged information
/// in the XY plane. At every point, the fortran code writes the 5 points of information that are
/// available (rho, u, v, w, energy).
///
/// In addition, the span averaged information is written for the entirity of each mpi process at
/// once without duplicating the allocations. This means that the data is oriented something like
/// this for a simple 2x2 grid with `NPROC = 2`:
///
///
/// ```
/// rho @ (0,0) + PROC 0
/// u   @ (0,0) + PROC 0
/// v   @ (0,0) + PROC 0
/// w   @ (0,0) + PROC 0
/// E   @ (0,0) + PROC 0
/// rho @ (0,1) + PROC 0
/// u   @ (0,1) + PROC 0
/// v   @ (0,1) + PROC 0
/// w   @ (0,1) + PROC 0
/// E   @ (0,1) + PROC 0
/// rho @ (1,0) + PROC 1 <---- second mpi procses starts here
/// u   @ (1,0) + PROC 1
/// v   @ (1,0) + PROC 1
/// w   @ (1,0) + PROC 1
/// E   @ (1,0) + PROC 1
/// rho @ (1,1) + PROC 1
/// u   @ (1,1) + PROC 1
/// v   @ (1,1) + PROC 1
/// w   @ (1,1) + PROC 1
/// E   @ (1,1) + PROC 1
/// ```
pub(crate) fn convert_binary_to_vtk_information(
    data: &[f64],
    config: &cli::ConfigGenerator,
) -> Result<SpanVtkInformation, BinaryToVtkError> {
    let mut data = data.into_iter();

    let mut rho_arr = Array4::zeros((config.x_divisions, config.y_divisions, 1, 1));
    let mut velocity_arr = Array4::zeros((config.x_divisions, config.y_divisions, 1, 3));
    let mut energy_arr = Array4::zeros((config.x_divisions, config.y_divisions, 1, 1));

    let nx_proc = config.x_divisions / config.mpi_x_split;
    let ny = config.y_divisions;

    for proc_number in 0..config.mpi_x_split {
        for i_proc in 0..nx_proc {
            for j in 0..ny {
                let rho = *data.next().unwrap();
                let u = *data.next().unwrap();
                let v = *data.next().unwrap();
                let w = *data.next().unwrap();
                let energy = *data.next().unwrap();

                // scale the current x value to the number of the process that we are
                // dealing with
                let i = (nx_proc * proc_number) + i_proc;
                *rho_arr.get_mut((i, j, 0, 0)).unwrap() = rho;

                *velocity_arr.get_mut((i, j, 0, 0)).unwrap() = u;
                *velocity_arr.get_mut((i, j, 0, 1)).unwrap() = v;
                *velocity_arr.get_mut((i, j, 0, 2)).unwrap() = w;

                *energy_arr.get_mut((i, j, 0, 0)).unwrap() = energy;
            }
        }
    }

    // If there is extra information left in the array then something bad has happened
    let (size_left, _) = data.size_hint();
    if data.next() != None {
        return Err(BinaryToVtkError::ExtraData(size_left));
    }

    Ok(SpanVtkInformation {
        rho: VectorPoints::new(rho_arr),
        velocity: VectorPoints::new(velocity_arr),
        energy: VectorPoints::new(energy_arr),
    })
}

/// helper function to convert an array of LE bytes to `f64`
pub(crate) fn bytes_to_float(bytes: &[u8]) -> Vec<f64> {
    bytes
        .chunks(8)
        .into_iter()
        .map(|x| {
            let mut arr = [0; 8];

            if x.len() != 8 {
                panic!("missing information");
            }

            x.into_iter()
                .enumerate()
                .for_each(|(idx, val)| arr[idx] = *val);

            f64::from_le_bytes(arr)
        })
        .collect()
}

#[test]
/// ensure that data written to a binary file conforms to the format that we expect
/// it to, and that there are no missing / extra bytes in the file that should
/// not be there
fn check_binary_with_config() {
    let data = bytes_to_float(include_bytes!(
        "../static/span_average_00010_average.binary"
    ));

    let config_bytes = include_bytes!("../static/span_average_input.json");
    let config: cli::ConfigGenerator = serde_json::from_slice(config_bytes).unwrap();

    let formatted_data = convert_binary_to_vtk_information(&data, &config);

    formatted_data.unwrap();
}
