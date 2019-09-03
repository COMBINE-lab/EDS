datas=("neurons_1m neurons_900" "neurons_2k" "pbmc_4k" "pbmc_8k" "neurons_9k" "pbmc_40k" "neurons_450k")
cells=(1000000 931 2022 4340 8381 9128 43400 456400)
feats=(27998 50686 50686 58278 58278 50686 58278 50686)

for id in {0..7}; do
	data=${datas[$id]}
	cell=${cells[$id]}
	feat=${feats[$id]}

	echo $data
	echo "EDS"
	Rscript --vanilla parsers/readEds.R $data $cell $feat &&
	/usr/bin/time Rscript --vanilla parsers/readEds.R $data $cell $feat &&
	
	echo "H5" &&
	Rscript --vanilla parsers/readH5.R $data &&
	/usr/bin/time Rscript --vanilla parsers/readH5.R $data &&

	echo "Mtx" &&
	Rscript --vanilla parsers/readMtx.R $data &&
	/usr/bin/time Rscript --vanilla parsers/readMtx.R $data &&

	echo "loom" &&
	Rscript --vanilla parsers/readLoom.R $data &&
	/usr/bin/time Rscript --vanilla parsers/readLoom.R $data &&

	echo "CSV" 
	/usr/bin/time Rscript --vanilla parsers/readCsv.R $data
done

