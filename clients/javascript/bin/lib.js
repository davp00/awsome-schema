import { createWriteStream, existsSync, mkdirSync, chmodSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join } from "node:path";
import { pipeline } from "node:stream/promises";

const RELEASE_REPO = "davp00/awesome-schema";

/** Rust target triple for this machine, or null when no binary is published. */
export function rustTarget(platform = process.platform, arch = process.arch) {
  if (platform === "linux" && arch === "x64") return "x86_64-unknown-linux-gnu";
  if (platform === "darwin" && arch === "arm64") return "aarch64-apple-darwin";
  if (platform === "darwin" && arch === "x64") return "x86_64-apple-darwin";
  if (platform === "win32" && arch === "x64") return "x86_64-pc-windows-msvc";
  return null;
}

export function assetName(target) {
  const ext = target.includes("windows") ? ".exe" : "";
  return `awesome-schema-${target}${ext}`;
}

export function cachePath(version, target) {
  const base = process.platform === "win32"
    ? join(process.env.LOCALAPPDATA ?? join(homedir(), "AppData", "Local"), "awesome-schema")
    : join(process.env.XDG_CACHE_HOME ?? join(homedir(), ".cache"), "awesome-schema");
  return join(base, version, assetName(target));
}

export function releaseUrl(version, target) {
  const tag = version.startsWith("v") ? version : `v${version}`;
  return `https://github.com/${RELEASE_REPO}/releases/download/${tag}/${assetName(target)}`;
}

export async function downloadBinary(version, target, dest) {
  const response = await fetch(releaseUrl(version, target));
  if (!response.ok || !response.body) {
    throw new Error(`download ${releaseUrl(version, target)} failed: ${response.status}`);
  }
  mkdirSync(dirname(dest), { recursive: true });
  await pipeline(response.body, createWriteStream(dest));
  if (process.platform !== "win32") chmodSync(dest, 0o755);
}

export async function ensureBinary(version) {
  if (process.env.AWESOME_SCHEMA_BIN) return process.env.AWESOME_SCHEMA_BIN;
  const target = rustTarget();
  if (!target) {
    throw new Error(`no awesome-schema binary for ${process.platform} ${process.arch}`);
  }
  const dest = cachePath(version, target);
  if (!existsSync(dest)) await downloadBinary(version, target, dest);
  return dest;
}
