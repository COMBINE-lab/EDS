## What's EDS ?
EDS is an accronym for Efficient single cell binary Data Storage format for the cell-feature count matrices.

## Why we need a new storage format ?
Recent advancements in single-cell technologies have seen rapid increase in the amount of data. Most single-cell studies generate a cell by feature (can be gene) count matrices, where the number of cells are now reaching towards millions. Traditional Single-cell quantification pipelines use matrix market exchange (mtx) format (sometimes gzipped) for sharing the count matrices. However, the textual representation of mtx format makes it bigger in size compared to a compressed binary format. Our quantification tool [alevin](https://combine-lab.github.io/alevin-tutorial/) dumps the output in EDS format which saves storage space.


## What are the caveats ?
There are other formats (such as [loom](https://github.com/linnarsson-lab/loompy)) which are designed for optimizing the query of the matrix. EDA is primarily designed to improve the storage efficiency rather than query and currently don't support random access to a cell (row).

## How to convert eds to mtx format ?
We have a simple rust code inside the `src-rs`, it can be installed using `cargo build --release` and can be used as `./target/release/eds convert -e <eds_file>`.

## Benchamrks
Currently benchmarked on very small datasets of just 300 cells and 60603 gene features the comparisons trades off as follows.  
![300 Cells](https://github.com/COMBINE-lab/EDS/blob/master/benchmarks/inital.png)

## Future 
- [ ] Benchmarks
- [ ] Support delayedArray R object
- [ ] Random access through `EDA index`

## Contributors
- Avi Srivastava
- Mike Love
- Rob Patro
