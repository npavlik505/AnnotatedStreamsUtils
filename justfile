nv: 
	mkdir -p ~/apptainer

	rm -f nv.sif

	sudo apptainer build \
		nv.sif \
		"docker://nvcr.io/nvidia/nvhpc:22.1-devel-cuda_multi-ubuntu20.04"

base:
	mkdir -p ~/apptainer

	rm -f base.sif 
	time APPTAINER_TMPDIR=~/apptainer sudo -E apptainer build --nv base.sif base.apptainer
	du -sh base.sif

build:
	rm -f streams.sif
	time APPTAINER_TMPDIR=~/apptainer sudo -E apptainer build --nv streams.sif build.apptainer
	du -sh streams.sif

# build a config json file as input to the solver
config_output := "./output/input.json"
config:
	echo {{config_output}}
	cargo r -- config-generator {{config_output}} \
		--steps 100 \
		--x-divisions 600 \
		--json

run:
	cargo r -- run-local \
		--workdir ./output/ \
		--config ./output/input.json \
		--database $STREAMS_DIR/examples/supersonic_sbli/database_bl.dat \
		16

# get a shell inside the container
# requires the ./output directory to be created, and a ./streams.sif file to be made
shell:
	apptainer shell --nv --bind ./output/distribute_save:/distribute_save,./output/input:/input ./streams.sif
