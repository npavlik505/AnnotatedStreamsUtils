base:
	rm -f nv.sif
	mkdir ~/apptainer

	sudo apptainer build \
		nv.sif \
		"docker://nvcr.io/nvidia/nvhpc:22.1-devel-cuda_multi-ubuntu20.04"

	rm base.sif
	time APPTAINER_TMPDIR=~/apptainer sudo -E apptainer build --nv base.sif base.apptainer
	du -sh base.sif

build:
	rm -f streams.sif
	time APPTAINER_TMPDIR=~/apptainer sudo -E apptainer build --nv streams.sif build.apptainer
	du -sh streams.sif
