rm nv.sif
sudo singularity build \
	nv.sif \
	"docker://nvcr.io/nvidia/nvhpc:22.1-devel-cuda_multi-ubuntu20.04"

rm base.sif
time SINGULARITY_TMPDIR=~/singularity sudo -E singularity build --nv base.sif base.singularity
du -sh base.sif
