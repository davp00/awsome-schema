import { existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));

/** Package root: e2e/typescript */
export const packageRoot = resolve(here, "..");

export const envPath = join(packageRoot, ".e2e-env.json");
export const fixtureSchemaPath = join(packageRoot, "fixtures", "client.schema");
export const tmpDir = join(packageRoot, ".tmp");
export const generatedDir = join(packageRoot, "generated");

export function findWorkspaceRoot(start = packageRoot): string {
  let dir = start;
  for (;;) {
    if (existsSync(join(dir, "Cargo.toml")) && existsSync(join(dir, "crates"))) {
      return dir;
    }
    const parent = dirname(dir);
    if (parent === dir) {
      throw new Error("could not find workspace root (Cargo.toml)");
    }
    dir = parent;
  }
}

export type E2eEnv = {
  skipped: boolean;
  reason?: string;
  url?: string;
  ns: string;
  db: string;
  user: string;
  pass: string;
  ownedContainer: boolean;
};
