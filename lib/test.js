var addon = require('../native');

var to_hash = "Give me the sha1 hash of this!";
console.log(to_hash);
console.log(addon.sha1(to_hash));

var file_to_hash = "./README.md"
console.log("Hash the README.md");
console.log(addon.sha1File(file_to_hash));
