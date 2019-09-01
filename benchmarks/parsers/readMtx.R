library(Matrix)

args = commandArgs(trailingOnly=TRUE)
data <- args[1]
fpath <- paste0("/mnt/scratch1/avi/anton/alevin_r/EDS/benchmarks/data/", data, "/quants_mat.mtx.gz")

system.time({
mm <- readMM( gzfile( fpath ) )
})

print(dim(mm))
