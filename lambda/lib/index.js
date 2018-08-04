const native = require('../native');

exports.handler = (event, context, callback) => callback(null, native.hello());
console.log(native.hello());
