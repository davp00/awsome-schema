#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { ensureBinary } from "./lib.js";

const require = createRequire(import.meta.url);
const { version } = require("../package.json");

let binary;
try {
  binary = await ensureBinary(version);
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`awesome-schema: ${message}`);
  process.exit(1);
}

const child = spawnSync(binary, process.argv.slice(2), { stdio: "inherit" });
if (child.error) {
  console.error(`awesome-schema: failed to start ${binary}: ${child.error.message}`);
  process.exit(1);
}
process.exit(child.status ?? 1);
