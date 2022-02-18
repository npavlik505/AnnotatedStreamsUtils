# streams-utils

Utility commands for building, running, and distributing cases from the `streams` solver

## Building streams

Running the streams solver on CPU hardware is prohibitively slow and should not be attempted. However,
since the `nvcc` fortran compiler is _much_ slower than `gfortran` (cpu) compiler, it is reasonable
to typecheck and run the solver for a few steps on the CPU to verify a simple implementation.

Running on the CPU has the following line in `src/Makefile`:

```
COMPILE = "gnu"
```

Running on the GPU has the following line in `src/Makefile`:

```
COMPILE = "nvfortran"
```

The nvidia HPC compiler `nvcc` used to run on the gpu is extensively painful to install manually. Instead,
we can use `singularity` containers to build the code with precompiled and installed `nvcc` compilers
in the container. First, simply download the base `sif` file from nvidia. This only needs to be done
once:

```
sh ./prepare-build.sh
```

Then, you can build your local copy of streams in a container with 

```
sh ./build-singularity.sh
```

Note that compiling singularity containers on non-linux machines is arduous and not recommended.

## Compiling streams-uitils

`streams-utils` is itself a command line utility for generating `input.dat` config files, parameter sweeps, 
and `distribute-jobs.yaml` files for running on the in-house cluster. You will need a recent 
version of the rust compiler `rustc` and its associated `cargo` utility. They can be
downloaded [here](https://www.rust-lang.org/tools/install). Then, simply:


```
cargo install --path .
```

and then you have access to `streams-utils`:

```
streams-utils
```
