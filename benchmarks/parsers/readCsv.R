args = commandArgs(trailingOnly=TRUE)

data <- args[1]
fpath <- paste0("/mnt/scratch1/avi/anton/alevin_r/EDS/benchmarks/data/", data, "/quants_mat.csv.gz")

system.time({
csv <- read.table( gzfile( fpath ), sep="," )
})

print(dim(csv))
