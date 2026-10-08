#!/usr/bin/env node
import { createRequire } from "node:module";
import { ensureBinary } from "./lib.js";

const require = createRequire(import.meta.url);
const { version } = require("../package.json");

if (process.env.AWESOME_SCHEMA_BIN) {
  process.exit(0);
}

try {
  await ensureBinary(version);
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  console.warn(`awesome-schema: CLI binary not installed (${message}).`);
  console.warn("Set AWESOME_SCHEMA_BIN to a local binary, or install after a GitHub release exists.");
}
