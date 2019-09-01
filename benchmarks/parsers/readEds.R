library(Rcpp)
library(Matrix)

args = commandArgs(trailingOnly=TRUE)
sourceCpp("/mnt/scratch1/avi/anton/alevin_r/EDS/src-cpp/readEDS.cpp")

data <- args[1]
num.cells <- as.integer(args[2])
num.genes <- as.integer(args[3])
fpath <- paste0("/mnt/scratch1/avi/anton/alevin_r/EDS/benchmarks/data/", data, "/quants_mat.eds.gz")

system.time({
pos <- getSparseMatrix( num.genes, num.cells, fpath )
})

str(pos)
