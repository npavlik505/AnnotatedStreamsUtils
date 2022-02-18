rm streams.sif
time SINGULARITY_TMPDIR=~/singularity sudo -E singularity build --nv streams.sif build.singularity
du -sh streams.sif

