function generateUrl() {
    var num = parseInt(document.getElementById("num").value);
    var range = parseInt(document.getElementById("range").value);
    var base = window.location.protocol + "//" + window.location.host + window.location.pathname;
    var url = base + "?num=" + num + "&count=" + range;
    d3.select('#url').html(url);
}

function getUrlParams() {
    var vars = {};
    var parts = window.location.href.replace(/[?&]+([^=&]+)=([^&]*)/gi, function(m, key, value) {
        vars[key] = value;
    });
    return vars;
}

function tabulate(div, data, columns) {
    var table = div.append('table')
    var thead = table.append('thead')
    var tbody = table.append('tbody');

    // append the header row
    thead.append('tr')
        .selectAll('th')
        .data(columns).enter()
        .append('th')
        .text(function (column) { return column; });

    // create a row for each object in the data
    var rows = tbody.selectAll('tr')
        .data(data)
        .enter()
        .append('tr');

    // create a cell in each row for each column
    var cells = rows.selectAll('td')
        .data(function (row) {
            return columns.map(function (column) {
                return {column: column, value: row[column]};
            });
        })
        .enter()
        .append('td')
        .text(function (d) { return d.value; });

    $('table').columntoggle();
    return table;
}
