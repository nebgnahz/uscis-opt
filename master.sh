#!/usr/bin/env bash
set -e

FROM=1890230500
TO=1890231000
URL="https://3yqhf6gl55.execute-api.us-east-1.amazonaws.com/crawl/crawl?start=${FROM}&end=${TO}"
TODAY=`date +%Y-%m-%d`
FILE="data/quick_summary/${TODAY}_${FROM}-${TO}.log"
echo ${TODAY}
echo ${FILE}

echo "ID,State,Description" > ${FILE}
curl -s ${URL} | jq -r '.crawled | .[]' >> ${FILE}

xsv select State ${FILE} | xsv frequency | xsv table
