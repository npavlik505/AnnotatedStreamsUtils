nv: 
	mkdir -p ~/apptainer

	rm -f nv.sif

	sudo apptainer build \
		nv.sif \
		"docker://nvcr.io/nvidia/nvhpc:22.1-devel-cuda_multi-ubuntu20.04"

base:
	mkdir -p ~/apptainer

	rm -f base.sif 
	export APPTAINER_TMPDIR=~/apptainer
	time sudo -E apptainer build --nv base.sif base.apptainer
	du -sh base.sif

build:
	rm -f streams.sif
	export APPTAINER_TMPDIR=~/apptainer
	time sudo -E apptainer build --nv streams.sif build.apptainer
	du -sh streams.sif

# build a config json file as input to the solver
config_output := "./output/input.json"
config:
	echo {{config_output}}
	cargo r -- config-generator {{config_output}} \
		--steps 50000 \
		--x-divisions 600 \
		--json \
		--span-average-io-steps 50 \
		--use-python

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
	apptainer shell --nv --bind $STREAMS_DIR:/streams ./streams.sif
