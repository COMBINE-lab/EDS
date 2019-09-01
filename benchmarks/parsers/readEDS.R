library(Rcpp)
library(Matrix)

sourceCpp("/mnt/scratch1/avi/anton/alevin_r/EDS/src-cpp/readEDS.cpp")

num.cells <- 931
num.genes <- 50686
data <- "neurons_900"
fpath <- paste0("/mnt/scratch1/avi/anton/alevin_r/EDS/benchmarks/data/", data, "/quants_mat.eds.gz")

system.time({
pos <- getSparseMatrix( num.genes, num.cells, fpath )
})

str(pos)
