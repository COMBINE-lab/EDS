library(hdf5r)
library(Matrix)

data <- "neurons_900"
fpath <- paste0("/mnt/scratch1/avi/anton/alevin_r/EDS/benchmarks/data/", data, "/quants_mat.h5")

system.time({
infile <- hdf5r::H5File$new(filename = fpath, mode = 'r')
genome <- "/matrix"

counts <- infile[[paste0(genome, '/data')]]
indices <- infile[[paste0(genome, '/indices')]]
indptr <- infile[[paste0(genome, '/indptr')]]
shp <- infile[[paste0(genome, '/shape')]]

sparse.mat <- sparseMatrix(
  i = indices[] + 1,
  p = indptr[],
  x = as.numeric(x = counts[]),
  dims = shp[],
  giveCsparse = TRUE
)

})

str(sparse.mat)
