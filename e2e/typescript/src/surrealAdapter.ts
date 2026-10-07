import {
  RecordId,
  StringRecordId,
  Table,
  type Surreal,
} from "surrealdb";

/** Minimal shape expected by generated TypeScript client helpers. */
export type SurrealLike = {
  query<T = unknown>(sql: string, vars?: Record<string, unknown>): Promise<T>;
  select<T = unknown>(thing: string): Promise<T>;
  create<T = unknown>(thing: string, data?: Record<string, unknown>): Promise<T>;
  merge<T = unknown>(thing: string, data?: Record<string, unknown>): Promise<T>;
  delete<T = unknown>(thing: string): Promise<T>;
};

const RECORD_ID_RE = /^[A-Za-z_][A-Za-z0-9_]*:[^\s]+$/;

/**
 * Adapt SurrealDB JS v2 to the generated `SurrealLike` surface.
 * v2 uses fluent `.content()` / `.merge()` builders and typed `RecordId` values.
 */
export function asSurrealLike(db: Surreal): SurrealLike {
  return {
    async query<T = unknown>(sql: string, vars?: Record<string, unknown>): Promise<T> {
      return (await db.query(sql, encodeDeep(vars) as Record<string, unknown> | undefined)) as T;
    },
    async select<T = unknown>(thing: string): Promise<T> {
      return (await db.select(toResource(thing))) as T;
    },
    async create<T = unknown>(thing: string, data?: Record<string, unknown>): Promise<T> {
      return (await db
        .create(toResource(thing))
        .content((encodeDeep(data ?? {}) as Record<string, unknown>) ?? {})) as T;
    },
    async merge<T = unknown>(thing: string, data?: Record<string, unknown>): Promise<T> {
      return (await db
        .update(toResource(thing))
        .merge((encodeDeep(data ?? {}) as Record<string, unknown>) ?? {})) as T;
    },
    async delete<T = unknown>(thing: string): Promise<T> {
      return (await db.delete(toResource(thing))) as T;
    },
  };
}

function toResource(thing: string): Table | RecordId | StringRecordId {
  if (thing.includes(":")) {
    return new StringRecordId(thing);
  }
  return new Table(thing);
}

function encodeDeep(value: unknown): unknown {
  if (typeof value === "string" && RECORD_ID_RE.test(value)) {
    return new StringRecordId(value);
  }
  if (Array.isArray(value)) {
    return value.map(encodeDeep);
  }
  if (value && typeof value === "object" && !(value instanceof StringRecordId) && !(value instanceof RecordId) && !(value instanceof Table)) {
    return Object.fromEntries(
      Object.entries(value as Record<string, unknown>).map(([key, nested]) => [
        key,
        encodeDeep(nested),
      ]),
    );
  }
  return value;
}
