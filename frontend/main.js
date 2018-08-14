const params = getUrlParams();

if (params.num != undefined) {
    document.getElementById("num").value = parseInt(params.num);
}

if (params.count != undefined) {
    document.getElementById("range").value = parseInt(params.count);
}

update();

var num_input = document.getElementById("num");
var range_input = document.getElementById("range");
num_input.addEventListener("keyup", (event) => {
    event.preventDefault();
    if (event.keyCode === 13) {
        update();
    }
});
range_input.addEventListener("keyup", (event) => {
    event.preventDefault();
    if (event.keyCode === 13) {
        update();
    }
});

function getStatus(i) {
    for (const [key, value] of Object.entries(i)) {
        if (key != 'last_update' && key != 'last_crawl' &&
            value == i['last_update']) {
            return key;
        }
    }
    return "unknown";
}

function csv_file(i) {
    return "../raw-data/" + i + ".csv";
}

function txt_file(i) {
    return "../raw-data/" + i + ".txt";
}

function update() {
    var num = parseInt(document.getElementById("num").value);
    var range = parseInt(document.getElementById("range").value);

    var base = Math.floor(num / 100) * 100;
    var nfiles = Math.ceil((num + range) / 100) - base / 100;

    const sequences = Array(nfiles).fill().map((_, i) => base + i * 100);
    const files = sequences.map((i) => d3.csv(csv_file(i)));

    var date = txt_file(base);
    d3.text(date)
        .then((data) => {
            const d = new Date(Date.parse(data + "+0000"));
            d3.select('#crawl_date')
                .html("Crawled on " + d);
        });

    Promise.all(files).then(function(files) {
            var merged = [].concat.apply([], files);
            return merged;
        }).then(function(data) {
            data = data.filter((i) =>
                               parseInt(i.id) >= num && parseInt(i.id) < num + range);
            return data;
        }).then(function(data) {
            for (var i = 0; i < data.length; i++) {
                if (data[i].received == '') {
                    for (var j = 0; j < data.length; j++) {
                        if (data[j].received != '') {
                            data[i].received = data[j].received;
                            break;
                        }
                    }
                }
            }
            return data;
        }).then(function(data) {

            d3.select("#table").html("");
            tabulate(d3.select("#table"), data, Object.keys(data[0]));
            return data;
        })
        .then(function(data) {
            var count = data.filter((i) => i['is_i765'] == "true").map((i) => getStatus(i));
            var init = new Map([
                ['received', 0],
                ['delivered', 0],
                ['produced', 0],
                ['pickedup', 0],
                ['mailed', 0],
                ['returned', 0],
                ['rejected', 0],
                ['rfe', 0],
                ['update', 0],
                ['other', 0],
                ['unknown', 0]
            ]);

            var frequency = count.reduce((acc, val) => acc.set(val, 1 + (acc.get(val) || 0)), init);
            var columns = [];
            frequency.forEach((k, v, m) => columns.push([v, k]));
            columns.sort((a, b) => {
                return a[0] < b[0] ? -1 : (a[0] > b[0]) ? 1 : 0;
            })
            var chart = c3.generate({
                bindto: '#chart',
                data: {
                    columns: columns,
                    type: 'pie',
                },
                pie: {
                    label: {
                        format: function(value, ratio, id) {
                            return value + ', ' + Math.floor(ratio * 100) + '%';
                        }
                    }
                }
            });
        })
        .catch((err) => {
            alert("Failed to load the data for " + num + " with range " + range);
        });
}
