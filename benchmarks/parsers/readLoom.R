library(loomR)

args = commandArgs(trailingOnly=TRUE)
# inparts taked from https://satijalab.org/loomR/loomR_tutorial.html
data <- args[1]
fpath <- paste0("/mnt/scratch1/avi/anton/alevin_r/EDS/benchmarks/data/", data, "/quants_mat.loom")

system.time({ 
lfile <- connect(filename = fpath, mode = "r+")
full.matrix <- lfile$matrix[, ]
})

dim( full.matrix )
