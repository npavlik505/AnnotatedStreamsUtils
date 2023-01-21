nv: 
	mkdir -p $APPTAINER_TMPDIR

	rm -f nv.sif

	sudo apptainer build \
		nv.sif \
		"docker://nvcr.io/nvidia/nvhpc:22.1-devel-cuda_multi-ubuntu20.04"

base:
	mkdir -p $APPTAINER_TMPDIR

	rm -f base.sif 
	echo $APPTAINER_TMPDIR
	time sudo -E apptainer build --nv base.sif base.apptainer
	du -sh base.sif

build:
	rm -f streams.sif
	echo $APPTAINER_TMPDIR
	time sudo -E apptainer build --nv streams.sif build.apptainer
	du -sh streams.sif

# build a config json file as input to the solver
config_output := "./output/input.json"
streams_flow_type := "boundary-layer"

config:
	echo {{config_output}}

	# 600, 208, 100

	cargo r -- config-generator {{config_output}} \
		{{streams_flow_type}} \
		--steps 50000 \
		--mach-number 0.0 \
		--x-divisions 300 \
		--y-divisions 208 \
		--z-divisions 100 \
		--json \
		--x-length 3.0 \
		--y-length 5.0 \
		--mpi-x-split 4 \
		--span-average-io-steps 10 \
		--python-flowfield-steps 1000 \
		--slot-start 100 \
		--slot-end 200 \
		--sbli-blowing-bc 1 \
		--use-python

	cat {{config_output}}

run:
	cargo r -- run-local \
		--workdir ./output/ \
		--config ./output/input.json \
		--database $STREAMS_DIR/examples/supersonic_sbli/database_bl.dat \
		--python-mount $STREAMS_DIR/python \
		16

# get a shell inside the container
# requires the ./output directory (with its associated folders) to be created, 
# and a ./streams.sif file to be made
shell:
	apptainer shell --nv --bind ./output/distribute_save:/distribute_save,./output/input:/input ./streams.sif

# get a shell inside the container
# and bind your $STREAMS_DIR environment variable to the folder
# /streams
local:
	apptainer shell --nv --bind $STREAMS_DIR:/streams ./base.sif

vtk:
	cargo r --release -- hdf5-to-vtk ./output/distribute_save
