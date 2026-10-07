import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GenericContainer, type StartedTestContainer, Wait } from "testcontainers";

import {
  envPath,
  findWorkspaceRoot,
  fixtureSchemaPath,
  tmpDir,
  type E2eEnv,
} from "./src/paths.js";

const SURREAL_IMAGE = "surrealdb/surrealdb:v3.3.0";
const SURREAL_PORT = 8000;

export default async function globalSetup() {
  const ns = process.env.SURREAL_NS ?? "test";
  const dbName = process.env.SURREAL_DB ?? "main";
  const user = process.env.SURREAL_USER ?? "root";
  const pass = process.env.SURREAL_PASS ?? "root";

  let url = process.env.SURREALDB_URL?.trim() || undefined;
  let ownedContainer = false;
  let container: StartedTestContainer | undefined;

  if (!url) {
    try {
      container = await new GenericContainer(SURREAL_IMAGE)
        .withCommand([
          "start",
          "--log",
          "info",
          "--user",
          "root",
          "--pass",
          "root",
          "--bind",
          "0.0.0.0:8000",
          "memory",
        ])
        .withExposedPorts(SURREAL_PORT)
        .withWaitStrategy(
          Wait.forLogMessage(/Started web server|Listening on/i).withStartupTimeout(120_000),
        )
        .start();
      const host = container.getHost();
      const port = container.getMappedPort(SURREAL_PORT);
      url = `ws://${host}:${port}`;
      ownedContainer = true;
    } catch (error) {
      const reason = `skipping typescript e2e: Docker unavailable (${formatError(error)})`;
      console.warn(reason);
      writeEnv({
        skipped: true,
        reason,
        ns,
        db: dbName,
        user,
        pass,
        ownedContainer: false,
      });
      return async () => {};
    }
  }

  mkdirSync(tmpDir, { recursive: true });
  const schemaPath = join(tmpDir, "client.schema");
  const fixture = readFileSync(fixtureSchemaPath, "utf8");
  writeFileSync(
    schemaPath,
    fixture
      .replace(/url\s*=\s*"[^"]*"/, `url      = "${url}"`)
      .replace(/namespace\s*=\s*"[^"]*"/, `namespace = "${ns}"`)
      .replace(/database\s*=\s*"[^"]*"/, `database  = "${dbName}"`)
      .replace(/username\s*=\s*"[^"]*"/, `username  = "${user}"`)
      .replace(/password\s*=\s*"[^"]*"/, `password  = "${pass}"`),
    "utf8",
  );

  const workspaceRoot = findWorkspaceRoot();
  runCli(workspaceRoot, ["--schema", schemaPath, "db", "push"]);
  runCli(workspaceRoot, ["--schema", schemaPath, "generate", "--target", "typescript"]);

  writeEnv({
    skipped: false,
    url,
    ns,
    db: dbName,
    user,
    pass,
    ownedContainer,
  });

  return async () => {
    if (ownedContainer && container) {
      await container.stop();
    }
  };
}

function writeEnv(env: E2eEnv) {
  writeFileSync(envPath, `${JSON.stringify(env, null, 2)}\n`, "utf8");
}

function runCli(workspaceRoot: string, args: string[]) {
  const result = spawnSync("cargo", ["run", "-p", "cli", "--quiet", "--", ...args], {
    cwd: workspaceRoot,
    encoding: "utf8",
    env: process.env,
  });
  if (result.status !== 0) {
    const detail = [result.stdout, result.stderr].filter(Boolean).join("\n");
    throw new Error(`cargo run -p cli -- ${args.join(" ")} failed:\n${detail}`);
  }
}

function formatError(error: unknown): string {
  if (error instanceof Error) return error.message;
  return String(error);
}
