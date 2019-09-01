name=$1
cells=$2
feats=$3

bin="/mnt/scratch1/avi/anton/alevin_r/EDS/src-rs/target/release/eds"
dpath="/mnt/scratch1/avi/anton/alevin_r/EDS/benchmarks/data/"$name"/quants_mat.eds.gz"
RUST_BACKTRACE=1 RUST_LOG=trace $bin convert -i $dpath --csv -c $cells -f $feats
RUST_BACKTRACE=1 RUST_LOG=trace $bin convert -i $dpath --mtx -c $cells -f $feats
RUST_BACKTRACE=1 RUST_LOG=trace $bin convert -i $dpath --h5 -c $cells -f $feats
