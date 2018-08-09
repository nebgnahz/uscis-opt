var cheerio = require('cheerio');
var request = require('request');
var async = require('async');
var stringify = require('csv-stringify');
var URL = 'https://egov.uscis.gov/casestatus/mycasestatus.do?appReceiptNum=RECEIPT_NUM';
var PREFIX = 'YSC';

var receiptNumbers = [];
var results = [];

console.log("Start global execution");

exports.handler = function(event, context, callback) {
    receiptNumbers = [];
    results = [];

    let start = event.start === undefined ? '1890220001' : event.start;
    let end = event.end === undefined ? '1890220003' : event.end;
    crawl(parseInt(start), parseInt(end), function(response) {
        stringify(response, function (err, output) {
            if (err) {
                console.error(err);
            }
            callback(null, output);
        });
    });
};

function crawl(start, end, callback) {
    console.log('crawling from ' + start + ' to ' + end);
    for (var i = start; i < end; i++) {
        receiptNumbers.push(PREFIX + i);
    }

    async.eachLimit(receiptNumbers, 100, retrieveReceiptNumber, function (err) {
        if (err) {
            console.error(err);
        }
        callback(results);
    });
}

function retrieveReceiptNumber(receiptNumber, callback) {
    request({
        url: URL.replace('RECEIPT_NUM', receiptNumber),
        rejectUnauthorized: false
    },
    function (err, resp, body) {
        if (err) {
            console.error(err);
        }
        var $ = cheerio.load(body);
        var title = $('.appointment-sec').find('.text-center').find('h1').text();
        var description = $('.appointment-sec').find('.text-center').find('p').text();
        var violation = $('label[for=accessviolation]').text();
        if (title.length == 0) {
            if (violation.length > 0) {
                console.log('access violation');
            }
            callback();
        } else {
            var row = [{
                'id': receiptNumber,
                'title': title,
                'description': description
            }];
            stringify(row, function (err, output) {
                if (err) {
                    console.error(err);
                }
                results.push(output);
                callback();
            });
        }
    });
}

var event = {};
event.start = 1890230006;
event.end = 1890230008;
exports.handler(event, null, function(a, b) { console.log(b); });
// crawl(189230006, 189230009, console.log);
