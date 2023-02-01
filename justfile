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
		--x-divisions 100 \
		--y-divisions 1200 \
		--z-divisions 100 \
		--json \
		--x-length 3.0 \
		--y-length 6.0 \
		--mpi-x-split 4 \
		--span-average-io-steps 10 \
		--python-flowfield-steps 1000 \
		--use-python \
		--sensor-threshold 0.1 \
		constant \
			--amplitude 1.0 \
			--slot-end 66 \
			--slot-start 33

	cat {{config_output}}

jet_validation_base_path := "./distribute/jet_validation/"

jet_validation_number := "07"
jet_validation_batch_name := "jet_validation_" + jet_validation_number
jet_valiation_output_folder := jet_validation_base_path + jet_validation_batch_name

jet_validation:
	echo {{jet_valiation_output_folder}}

	# 600, 208, 100
	#--steps 10000 \

	cargo r -- cases jet-validation \
		{{jet_valiation_output_folder}} \
		--batch-name {{jet_validation_batch_name}} \
		--solver-sif ./streams.sif \
		--steps 10000 \
		--database-bl $STREAMS_DIR/examples/supersonic_sbli/database_bl.dat \
		--matrix @karlik:matrix.org

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
