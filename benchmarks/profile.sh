datas=("neurons_900" "neurons_2k" "neurons_9k" "pbmc_4k" "pbmc_8k" "pbmc_40k" "neurons_450k")
cells=(931 2022 4340 8381 9128 43400 456400)
feats=(50686 50686 50686 58278 58278 58278 50686)

for id in {0..6}; do
	data=${datas[$id]}
	cell=${cellss[$id]}
	feat=${feats[$id]}

	echo $data
	#echo "EDS"
	#/usr/bin/time Rscript --vanilla parsers/readEds.R $data $cell $feat &&

	#echo "CSV" &&
	#/usr/bin/time Rscript --vanilla parsers/readCsv.R $data &&

	#echo "Mtx" &&
	#/usr/bin/time Rscript --vanilla parsers/readMtx.R $data &&

	#echo "loom" &&
	#/usr/bin/time Rscript --vanilla parsers/readLoom.R $data &&

	#echo "H5" &&
	#/usr/bin/time Rscript --vanilla parsers/readH5.R $data
done

