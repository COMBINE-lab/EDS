import gzip
import loompy
import numpy as np
import sys
from scipy.io import mmread

data = sys.argv[1]
mtx_file = "/mnt/scratch1/avi/anton/alevin_r/EDS/benchmarks/data/" + data + "/quants_mat.mtx.gz"
out_file = "/mnt/scratch1/avi/anton/alevin_r/EDS/benchmarks/data/" + data + "/quants_mat.loom"

data = mmread( gzip.open(mtx_file) )
(cells, feats) = data.shape

data = data.T

row_names = {}
row_names['rname'] = np.array(range(feats))

col_names = {}
col_names['cname'] = np.array( range(cells) )

loompy.create(out_file, data, row_names, col_names)
