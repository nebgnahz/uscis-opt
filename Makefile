.PHONY: upload

upload:
	(cd data/raw-data/; ls *.csv | sed 's/^.*/<a href="&">&<\/a><br\/>/' > raw.html)
	aws s3 sync data s3://uscis-opt

download:
	aws s3 sync s3://uscis-opt data
