import { readFileSync } from "node:fs";
import { Surreal } from "surrealdb";

import { asSurrealLike } from "awesome-schema/runtime";
import { envPath, type E2eEnv } from "./paths.js";

export function readE2eEnv(): E2eEnv {
  return JSON.parse(readFileSync(envPath, "utf8")) as E2eEnv;
}

export async function connectSurreal(env: E2eEnv): Promise<Surreal> {
  if (env.skipped || !env.url) {
    throw new Error(env.reason ?? "e2e skipped: no SurrealDB endpoint");
  }

  const db = new Surreal();
  await db.connect(env.url);
  await db.signin({ username: env.user, password: env.pass });
  await db.use({ namespace: env.ns, database: env.db });
  return db;
}

export async function openGeneratedClient(env: E2eEnv) {
  const db = await connectSurreal(env);
  const { createClient } = await import("../generated/index.js");
  return { db, client: createClient(asSurrealLike(db)) };
}

export function recordIdString(id: unknown): string {
  if (typeof id === "string") return id;
  if (id && typeof id === "object" && "toString" in id) {
    return String(id);
  }
  throw new Error(`unexpected record id: ${JSON.stringify(id)}`);
}
