const { compileGeneratedClient, findGeneratedClient } = require("./bin/compile-client.cjs");

if (!findGeneratedClient(__dirname)) {
  throw new Error("Run awesome-schema generate");
}

compileGeneratedClient(__dirname);
module.exports = require(".awesome-schema/client/index.cjs");
