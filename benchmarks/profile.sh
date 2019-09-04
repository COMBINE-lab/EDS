datas=("neurons_450k_random" "pbmc_40k_random" "neurons_1m" "neurons_900" "neurons_2k" "pbmc_4k" "pbmc_8k" "neurons_9k" "pbmc_40k" "neurons_450k")
cells=(456400 43400 1000000 931 2022 4340 8381 9128 43400 456400)
feats=(50686 58278 27998 50686 50686 58278 58278 50686 58278 50686)

for id in {0..0}; do
	data=${datas[$id]}
	cell=${cells[$id]}
	feat=${feats[$id]}

	echo $data $cell $feat
	echo "EDS"
	/usr/bin/time Rscript --vanilla parsers/readEds.R $data $cell $feat &&
	
	echo "H5" &&
	/usr/bin/time Rscript --vanilla parsers/readH5.R $data &&

	echo "Mtx" &&
	/usr/bin/time Rscript --vanilla parsers/readMtx.R $data &&

	echo "loom" &&
	/usr/bin/time Rscript --vanilla parsers/readLoom.R $data &&

	echo "CSV" 
	/usr/bin/time Rscript --vanilla parsers/readCsv.R $data
done

