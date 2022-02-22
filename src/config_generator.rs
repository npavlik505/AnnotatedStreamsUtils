use crate::prelude::*;
use cli::ConfigGenerator;

#[derive(thiserror::Error, Debug, From)]
pub(crate) enum ConfigError {
    #[error("There was an error with the z-divisions / mpi-x-split values chosen. The domain cannot be evenly split: {0}")]
    MpiSplitX(MpiSplitX),
    #[error("Domain requires too much memory: {0}")]
    Memory(Memory),
    #[error("{0}")]
    Custom(String),
}

#[derive(Debug, Display, Constructor)]
#[display(
    fmt = "x divisions: {} mpi divisions: {} remainder: {}",
    x_div,
    split,
    remainder
)]
pub(crate) struct MpiSplitX {
    x_div: usize,
    split: usize,
    remainder: usize,
}

#[derive(Debug, Display, Constructor)]
#[display(fmt = "invalid number of mpi splits: {}", split)]
pub(crate) struct MpiSplitZero {
    split: usize,
}

#[derive(Debug, Display, Constructor)]
#[display(
    fmt = "gpu memory capacity: {}, memory required for simulation: {}",
    gpu_memory_required,
    required_memory
)]
pub(crate) struct Memory {
    gpu_memory_required: Megabytes,
    required_memory: Megabytes,
}

#[derive(Debug, Display, PartialEq, PartialOrd)]
#[display(fmt = "{} Mb", _0)]
pub(crate) struct Megabytes(pub(crate) usize);

impl ConfigGenerator {
    /// check all the parameters of the input file to guarantee that the given input
    /// file will (likely) work in the solver without runtime error
    ///
    /// `max_gpu_mem` must only be specified if you are running the config on a gpu system
    pub(crate) fn validate(&self, max_gpu_mem: Option<Megabytes>) -> Result<(), ConfigError> {
        // make sure that the number of divisions with mpi is acceptable
        let split_remainder = self.x_divisions % self.mpi_x_split;
        if split_remainder != 0 {
            return Err(MpiSplitX::new(self.x_divisions, self.mpi_x_split, split_remainder).into());
        }

        if let Some(gpu_mem) = max_gpu_mem {
            self.check_gpu_mem(gpu_mem)?;
        }

        // from config file
        let nymax_wr = 201;
        if self.y_divisions < nymax_wr {
            return Err(ConfigError::Custom(format!(
                "y-divisions ({}) must be greater than {}",
                self.y_divisions, nymax_wr
            )));
        }

        // from config file
        let rly_wr = 2.5;
        if self.y_length < rly_wr {
            return Err(ConfigError::Custom(format!(
                "y-length ({}) must be greater than {}",
                self.y_length, rly_wr
            )));
        }

        if self.y_divisions % self.mpi_x_split != 0 {
            return Err(ConfigError::Custom(format!(
                "nymax (y-divisions) @ {} must be divisible by mpi-x-split @ {} (remainder: {})",
                self.y_divisions,
                self.mpi_x_split,
                self.y_divisions % self.mpi_x_split
            )));
        }

        Ok(())
    }

    /// check that there is enough memory available on the gpu to run the simulation
    ///
    /// this code is a replication of the memory checking code in fortran
    fn check_gpu_mem(&self, max_gpu_mem: Megabytes) -> Result<(), ConfigError> {
        // fortran memory checking code :
        // gpu_used_mem = 43._mykind      ! Number of 3D arrays on GPU
        // correction_factor = 1.5_mykind ! Safety margin
        // gpu_used_mem = gpu_used_mem+correction_factor
        // gpu_used_mem = gpu_used_mem*real((nx+2*ng),mykind)*real((ny+2*ng),mykind)*real((nz+2*ng),mykind)
        // gpu_used_mem = gpu_used_mem*storage_size(1._mykind)/8._mykind/(1024._mykind**2)
        let n_ghost = 3;
        let mut gpu_used_mem = 43.;
        // number of bytes for floating point
        let n_bytes = 8usize;
        gpu_used_mem += 1.5;
        gpu_used_mem *= ((self.x_divisions + (2 * n_ghost))
            * (self.y_divisions + (2 * n_ghost))
            * (self.z_divisions + (2 * n_ghost))) as f64;
        gpu_used_mem *= (n_bytes as f64) / (1024. * 1024.);
        let gpu_mem_required = Megabytes(gpu_used_mem as usize);

        if gpu_mem_required > max_gpu_mem {
            return Err(Memory::new(gpu_mem_required, max_gpu_mem).into());
        }

        Ok(())
    }
}

/// create a streams config file to be used in the solver
pub(crate) fn config_generator(args: cli::ConfigGenerator) -> Result<(), Error> {
    // 11 gb to megabytes
    // 11 gb is what is available on the 2080 TI available in the lab
    let gpu_memory = Some(Megabytes(11 * 10usize.pow(3)));

    // validate that the parameters can be run on the gpu
    args.validate(gpu_memory)?;

    let output = format!(
        r#"!=============================================================
!
! ███████╗████████╗██████╗ ███████╗ █████╗ ███╗   ███╗███████╗
! ██╔════╝╚══██╔══╝██╔══██╗██╔════╝██╔══██╗████╗ ████║██╔════╝
! ███████╗   ██║   ██████╔╝█████╗  ███████║██╔████╔██║███████╗
! ╚════██║   ██║   ██╔══██╗██╔══╝  ██╔══██║██║╚██╔╝██║╚════██║
! ███████║   ██║   ██║  ██║███████╗██║  ██║██║ ╚═╝ ██║███████║
! ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝
!
! Supersonic TuRbulEnt Accelerated navier stokes Solver
!
! input file
!
!=============================================================

 flow_type (0==>channel, 1==>BL, 2==>SBLI)
   2   

  Lx(rlx)             Ly(rly)         Lz(rlz)
  {lx}          {ly}         {lz}
 
  Nx(nxmax)     Ny(nymax)     Nz(nzmax)
  {nx}          {ny}        {nz}
 
 Ny_wr(nymax_wr)     Ly_wr(rly_wr)      dy+_w  jbgrid
  201                   2.5             .7       0

 ng  visc_ord  ep_ord  weno_par (1==>ord_1,2==>ord_3, 3==>ord_5, 4==>ord_7)
  3     6      6       3
 
 MPI_x_split     MPI_z_split
 {mpi_x_split}               1 

 sensor_threshold   xshock_imp   deflec_shock    pgrad (0==>constant bulk)
  0.1               15.             {angle}              0.
      
 restart   num_iter   cfl   dt_control  print_control  io_type
   0        {steps}      .75      1       1              2
      
 Mach      Reynolds (friction)  temp_ratio   visc_type   Tref (dimensional)   turb_inflow
 {mach}      {re}                   1.            2         160.                0.75
  
 stat_control  xstat_num
  500           10

 xstat_list
   10. 20. 30. 35. 40. 45. 50. 55. 60. 65.
 
 dtsave dtsave_restart  enable_plot3d   enable_vtk
  5.       50.                0              1

  rand_type
   -1

 save_probe_steps save_span_average_steps
    {probe_steps}         {span_average_steps}

 sbli_blowing_bc
    {sbli_blowing_bc}
   "#,
        lx = args.x_length,
        nx = args.x_divisions,
        ly = args.y_length,
        ny = args.y_divisions,
        lz = args.z_length,
        nz = args.z_divisions,
        mach = args.mach_number,
        re = args.reynolds_number,
        angle = args.shock_angle,
        mpi_x_split = args.mpi_x_split,
        steps = args.steps,
        probe_steps = args.probe_io_steps,
        span_average_steps = args.span_average_io_steps,
        sbli_blowing_bc = args.sbli_blowing_bc
    );

    if !args.dry {
        // write the contents to the file
        fs::write(&args.output_path, output.as_bytes())
            .map_err(|e| FileError::new(args.output_path, e))?;
    }

    Ok(())
}
