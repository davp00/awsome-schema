import {
  RecordId,
  StringRecordId,
  Table,
  Uuid,
  type LiveMessage,
  type LiveSubscription,
  type Surreal,
  type SurrealTransaction,
} from "surrealdb";

import type {
  LiveHandle,
  SurrealLike,
  SurrealOpsLike,
  SurrealTransactionLike,
} from "./runtime.js";

export type { Surreal } from "surrealdb";

const RECORD_ID_RE = /^[A-Za-z_][A-Za-z0-9_]*:[^\s]+$/;
const ADAPTED = Symbol.for("awesome-schema.surreal-session");

type SurrealQueryable = Pick<Surreal, "query" | "select" | "create" | "update" | "delete">;

/**
 * Adapt a SurrealDB JS v2 connection to the operations the generated client calls.
 * `createClient` does this itself. A second call returns the same session so a
 * caller that also wraps does not invoke `create().content` on the adapter.
 */
export function asSurrealLike(db: Surreal): SurrealLike {
  if (isAdapted(db)) return db;
  const session: SurrealLike = {
    ...bindOps(db),
    async beginTransaction(): Promise<SurrealTransactionLike> {
      const txn = await db.beginTransaction();
      return asSurrealTransaction(txn);
    },
    async live<T>(table: string): Promise<LiveHandle<T>> {
      const subscription = await db.live(new Table(table));
      await whenReady(subscription);
      return wrapLive<T>(subscription);
    },
    async liveOf<T>(id: unknown): Promise<LiveHandle<T>> {
      const subscription = await db.liveOf(asUuid(id));
      return wrapLive<T>(subscription);
    },
  };
  Object.defineProperty(session, ADAPTED, { value: true });
  return session;
}

function isAdapted(value: object): value is SurrealLike {
  return (value as { [ADAPTED]?: boolean })[ADAPTED] === true;
}

function asSurrealTransaction(txn: SurrealTransaction): SurrealTransactionLike {
  return {
    ...bindOps(txn),
    commit: () => txn.commit(),
    cancel: () => txn.cancel(),
  };
}

function bindOps(db: SurrealQueryable): SurrealOpsLike {
  return {
    async query<T = unknown>(sql: string, vars?: Record<string, unknown>): Promise<T> {
      return (await db.query(sql, encodeDeep(vars) as Record<string, unknown> | undefined)) as T;
    },
    async select<T = unknown>(thing: string): Promise<T> {
      return (await db.select(toRecordId(thing))) as T;
    },
    async create<T = unknown>(thing: string, data?: Record<string, unknown>): Promise<T> {
      return (await db
        .create(toTable(thing))
        .content((encodeDeep(data ?? {}) as Record<string, unknown>) ?? {})) as T;
    },
    async merge<T = unknown>(thing: string, data?: Record<string, unknown>): Promise<T> {
      return (await db
        .update(toRecordId(thing))
        .merge((encodeDeep(data ?? {}) as Record<string, unknown>) ?? {})) as T;
    },
    async delete<T = unknown>(thing: string): Promise<T> {
      return (await db.delete(toRecordId(thing))) as T;
    },
  };
}

/** Generated select/update/delete helpers pass `table:id` via normalizeThing. */
function toRecordId(thing: string): StringRecordId {
  return new StringRecordId(thing);
}

/** Generated create helpers pass a table name. */
function toTable(thing: string): Table {
  return new Table(thing);
}

function wrapLive<T>(subscription: LiveSubscription<Record<string, unknown>>): LiveHandle<T> {
  return {
    subscribe(listener) {
      subscription.subscribe((message: LiveMessage) => {
        if (message.action === "KILLED" || message.value === undefined) return;
        listener(message.action, message.value as T);
      });
    },
    kill: () => subscription.kill(),
  };
}

async function whenReady(
  subscription: LiveSubscription<Record<string, unknown>>,
): Promise<void> {
  const ready = (
    subscription as LiveSubscription<Record<string, unknown>> & {
      ready?: () => Promise<void>;
    }
  ).ready;
  if (typeof ready === "function") {
    await ready.call(subscription);
  }
}

function asUuid(id: unknown): Uuid {
  if (id instanceof Uuid) return id;
  if (typeof id === "string") return new Uuid(id);
  throw new Error("liveOf: expected a live query id");
}

/**
 * Encode `table:id` strings for the JS SDK.
 *
 * Surreal 4 rejects `StringRecordId("table:id-with-hyphens")` unless the id is
 * escaped as `table:⟨id-with-hyphens⟩`. Surreal 3.3 accepts both forms.
 */
export function encodeRecordIdString(thing: string): StringRecordId {
  const idx = thing.indexOf(":");
  if (idx <= 0) return new StringRecordId(thing);
  const table = thing.slice(0, idx);
  let id = thing.slice(idx + 1);
  if (!(id.startsWith("⟨") && id.endsWith("⟩")) && !/^[A-Za-z0-9_]+$/.test(id)) {
    id = `⟨${id}⟩`;
  }
  return new StringRecordId(`${table}:${id}`);
}

function encodeDeep(value: unknown): unknown {
  if (typeof value === "string" && RECORD_ID_RE.test(value)) {
    return encodeRecordIdString(value);
  }
  if (Array.isArray(value)) {
    return value.map(encodeDeep);
  }
  if (
    value &&
    typeof value === "object" &&
    !(value instanceof StringRecordId) &&
    !(value instanceof RecordId) &&
    !(value instanceof Table)
  ) {
    return Object.fromEntries(
      Object.entries(value as Record<string, unknown>).map(([key, nested]) => [
        key,
        encodeDeep(nested),
      ]),
    );
  }
  return value;
}
