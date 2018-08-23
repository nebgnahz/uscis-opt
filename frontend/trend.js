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

function query() {
    var num = parseInt(document.getElementById("num").value);
    var range = parseInt(document.getElementById("range").value);

    var url = window.location.protocol + "//" + window.location.host +
        "/vis/query.html" +
        "?num=" + num + "&count=" + range;
    window.location.href = url;
}

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

    Promise.all(files).then(function(files) {
            var merged = [].concat.apply([], files);
            return merged;
        }).then(function(data) {
            data = data
                .filter(
                    (i) => parseInt(i.id) >= num && parseInt(i.id) < num + range)
                .filter((i) => i['is_i765'] == "true");
            return data;
        }).then(function(data) {
            var remaining = data
                .map((i) => getStatus(i))
                .map((i) => i == "received" ? 1 : 0)
                .reduce((i, s) => s + i);
            var produce_date = data.map((i) => i['produced']);
            var frequency = produce_date.reduce(
                (acc, val) => acc.set(val, 1 + (acc.get(val) || 0)), new Map());
            frequency.delete("");

            var columns = [];
            frequency.forEach((k, v, m) => columns.push([new Date(v), k]));
            columns.sort((a, b) => {
                return -1 * (a[0] > b[0] ? 1 : (a[0] < b[0]) ? -1 : 0);
            });

            var trace1 = {
                x: columns.map((i) => i[0].toISOString().substr(0, 10)),
                y: columns.map((i) => i[1]),
                type: 'bar',
                name: 'Produced'
            };

            var last_day = new Date(columns[0][0].getTime());
            last_day.setDate(last_day.getDate() + 1);
            columns.unshift([last_day, remaining]);
            var by_far = 0;
            columns.map((i) => {
                i[1] = i[1] + by_far;
                by_far = i[1];
            });

            var trace2 = {
                x: columns.map((i) => i[0].toISOString().substr(0, 10)),
                y: columns.map((i) => i[1]),
                type: 'bar',
                name: 'Remaining'
            };

            var data = [trace1, trace2];
            var layout = {barmode: 'group'};
            Plotly.newPlot('trend', data, layout);
        })
        .catch((err) => {
            alert("Failed to load the data for " + num + " with range " + range);
        });
}
