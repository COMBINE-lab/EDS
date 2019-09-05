## What's EDS ?
EDS is an accronym for Efficient single cell binary Data Storage format for the cell-feature count matrices.

![EDS](https://github.com/COMBINE-lab/EDS/blob/master/eds.jpg)

## Why we need a new storage format ?
Recent advancements in single-cell technologies have seen rapid increase in the amount of data. Most single-cell studies generate a cell by feature (can be gene) count matrices, where the number of cells are now reaching towards millions. Traditional Single-cell quantification pipelines use matrix market exchange (mtx) format (sometimes gzipped) for sharing the count matrices. However, the textual representation of mtx format makes it bigger in size compared to a compressed binary format. Our quantification tool [alevin](https://combine-lab.github.io/alevin-tutorial/) dumps the output in EDS format which saves storage space.


## What are the caveats ?
There are other formats (such as [loom](https://github.com/linnarsson-lab/loompy)) which are designed for optimizing the query of the matrix. EDS is primarily designed to improve the storage efficiency rather than query and currently don't support random access to a cell (row).

## How to convert eds to mtx format ?
We have a simple rust code inside the `src-rs`, it can be installed using `cargo build --release` and can be used as `./target/release/eds convert -i <input gzipped file currently [eds.gz | mtx.gz]> --[mtx | eds | h5 | csv] -c <num_cells> -f <num_features>`.

## Benchmarks
* Size on disk.
![Disk Space](https://github.com/COMBINE-lab/EDS/blob/master/benchmarks/size.jpg)

* Matrix loading into memory time.
![Loading time](https://github.com/COMBINE-lab/EDS/blob/master/benchmarks/time.jpg)

* Memory required to load the matrix.
![Memory Usage](https://github.com/COMBINE-lab/EDS/blob/master/benchmarks/memory.jpg)

## Future 
- [ ] Support delayedArray R object
- [ ] Random access through `EDS index`

## Contributors
- Avi Srivastava
- Mike Love
- Rob Patro
