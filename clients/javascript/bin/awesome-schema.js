#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { resolve } from "node:path";
import { compileClientFile } from "./compile-client.cjs";
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

const child = spawnSync(binary, process.argv.slice(2), { encoding: "utf8" });
if (child.error) {
  console.error(`awesome-schema: failed to start ${binary}: ${child.error.message}`);
  process.exit(1);
}
if (child.stdout) process.stdout.write(child.stdout);
if (child.stderr) process.stderr.write(child.stderr);
if ((child.status ?? 1) === 0) {
  const match = child.stdout?.match(/Wrote TypeScript client to `([^`]+)`/);
  if (match) compileClientFile(resolve(match[1]));
}
process.exit(child.status ?? 1);
