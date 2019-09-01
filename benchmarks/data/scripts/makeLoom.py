import gzip
import loompy
import pandas as pd
import numpy as np

data = "pbmc_8k"
csv_file = "./" + data + "/quants_mat.csv.gz"
out_file = "./" + data + "/quants_mat.loom"

data = pd.read_csv( gzip.open(csv_file) )

data = data.T
data.drop("Unnamed: 0", axis=0, inplace=True)
datas = np.array(data.values, dtype=float)

rows = pd.DataFrame(data.index)
rows.columns = ['rname']
row_names = rows.to_dict("list")
row_names['rname'] = np.array(row_names['rname'])

cols = pd.DataFrame(data.columns)
cols.columns = ['cname']
col_names = cols.to_dict("list")
col_names['cname'] = np.array(col_names['cname'])

loompy.create(out_file, datas, row_names, col_names)
