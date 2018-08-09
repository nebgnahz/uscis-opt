.PHONY: upload

upload:
	aws s3 sync raw-data s3://uscis-opt

download:
	aws s3 sync s3://uscis-opt raw-data
