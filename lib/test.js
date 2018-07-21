var addon = require('../native');

var to_hash = "Give me the sha1 hash of this!";
console.log(to_hash);
console.log(addon.sha1(to_hash));
