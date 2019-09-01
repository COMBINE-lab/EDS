name=$1
cells=$2
feats=$3

RUST_BACKTRACE=1 RUST_LOG=trace /usr/bin/time ../../src-rs/target/release/eds convert -i $name/alevin/quants_mat.gz --csv -c $cells -f $feats
RUST_BACKTRACE=1 RUST_LOG=trace /usr/bin/time ../../src-rs/target/release/eds convert -i $name/alevin/quants_mat.gz --mtx -c $cells -f $feats
RUST_BACKTRACE=1 RUST_LOG=trace /usr/bin/time ../../src-rs/target/release/eds convert -i $name/alevin/quants_mat.gz --h5 -c $cells -f $feats
