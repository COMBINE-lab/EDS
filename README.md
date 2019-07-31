## What's EDS ?
EDS is an accronym for Efficient single cell Data Storage format for the cell-feature count matrix.

## Why we need a new storage format ?
Recent advancement in single-cell technologies has seen rapid rise in the amount of data. Most single-cell studies generate a cell by feature (can be gene) count matrices, where the number of cells are now reaching towards millions. Traditional Single-cell quantification pipelines use matrix market exchange (mtx) format (sometimes gzipped) for sharing the count matrices. However, the textual representation of mtx format makes it bigger in size compared to a compressed binary format saving space in the storage format. Our quantification tool alevin already dumps the output in this format.

## How to convert to and from mtx format ?
We have a simple rust code inside the `src-rs`, it can be installed using `cargo build --release` and can be used as `./release/EDS convert <mtx_file>` and `./release/EDS convert <eds_file>`

## Future 
[ ] Support delayedArray R object
