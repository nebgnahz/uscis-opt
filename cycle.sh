#!/usr/bin/env bash
set -ex

# "https://3yqhf6gl55.execute-api.us-east-1.amazonaws.com/crawl/crawl",
declare -a urls=("https://uscis-opt.herokuapp.com/"
                 "https://uscis-opt2.herokuapp.com/"
                 "https://uscis-opt3.herokuapp.com/"
                 "https://uscis-opt4.herokuapp.com/"
                 "https://uscis-opt5.herokuapp.com/")

echo ${urls}

for n in {0..19}
do
    t="$(($1 + ${n} * 100))"
    # USCIS_URL="http://localhost:8888"
    USCIS_URL="${urls[${n} % ${#urls[@]}]}" cargo run --release ${t}
    # USCIS_URL=URLS cargo run --release ${t}

    # USCIS_URL="" cargo run --release ${t}
    # USCIS_URL="https://uscis-opt2.herokuapp.com/" cargo run --release ${t}
    # USCIS_URL="https://uscis-opt5.herokuapp.com/" cargo run --release ${t}
done
