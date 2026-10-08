/** Surreal record id branded by table name. */
export type RecordId<Table extends string = string> = string & { readonly __table?: Table };

export type FieldSelectMeta =
  | { kind: "scalar"; filter: "string" | "number" | "boolean" | "datetime" | "json" | "array" | "id" }
  | { kind: "stored" | "computed"; targetTable: string; list: boolean }
  | { kind: "edge"; edgeTable: string; dir: "out" | "in"; list: boolean };

export type FieldWriteMeta =
  | { kind: "scalar" }
  | { kind: "stored"; targetTable: string; list: boolean; optional: boolean }
  | { kind: "computed"; targetTable: string; list: boolean; backLinkField: string; backLinkOptional: boolean }
  | { kind: "edge"; edgeTable: string; dir: "out" | "in"; list: boolean; inTable: string; outTable: string; payloadFields: string[] };

let selectMetaByTable: Record<string, Record<string, FieldSelectMeta>> = {};
let writeMetaByTable: Record<string, Record<string, FieldWriteMeta>> = {};

/** Point the shared runtime at the schema registries emitted with a generated client. */
export function bindSchemaMeta(
  selectMeta: Record<string, Record<string, FieldSelectMeta>>,
  writeMeta: Record<string, Record<string, FieldWriteMeta>>,
): void {
  selectMetaByTable = selectMeta;
  writeMetaByTable = writeMeta;
}

export function recordId<Table extends string>(
  table: Table,
  key: string,
): RecordId<Table> {
  return `${table}:${key}` as RecordId<Table>;
}

export function parseRecordId(
  value: string,
): { table: string; key: string } | null {
  const i = value.indexOf(":");
  if (i <= 0) return null;
  return { table: value.slice(0, i), key: value.slice(i + 1) };
}

export function asRecordId<Table extends string>(
  table: Table,
  value: string,
): RecordId<Table> {
  return (value.includes(":") ? value : `${table}:${value}`) as RecordId<Table>;
}

export function normalizeThing(table: string, id: string): string {
  return id.includes(":") ? id : `${table}:${id}`;
}

export async function firstRow<T>(result: unknown): Promise<T | undefined> {
  if (Array.isArray(result)) {
    const first = result[0];
    if (Array.isArray(first)) return first[0] as T | undefined;
    return first as T | undefined;
  }
  return result as T | undefined;
}

export async function asRows<T>(result: unknown): Promise<T[]> {
  if (Array.isArray(result)) {
    const first = result[0];
    if (Array.isArray(first)) return first as T[];
    return result as T[];
  }
  if (result == null) return [];
  return [result as T];
}

export function selectedFetchKeys(select: Record<string, boolean | undefined> | undefined): string[] {
  if (!select) return [];
  return Object.entries(select)
    .filter(([, enabled]) => enabled)
    .map(([key]) => key);
}

/** Core Surreal operations used by generated helpers (session or open transaction). */
export type SurrealOpsLike = {
  query<T = unknown>(sql: string, vars?: Record<string, unknown>): Promise<T>;
  select<T = unknown>(thing: string): Promise<T>;
  create<T = unknown>(thing: string, data?: Record<string, unknown>): Promise<T>;
  merge<T = unknown>(thing: string, data?: Record<string, unknown>): Promise<T>;
  delete<T = unknown>(thing: string): Promise<T>;
};

/** Interactive transaction handle (WebSocket / embedded). */
export type SurrealTransactionLike = SurrealOpsLike & {
  commit(): Promise<void>;
  cancel(): Promise<void>;
};

/** Session connection; `beginTransaction` is required for `$transaction`. */
export type LiveAction = "CREATE" | "UPDATE" | "DELETE";

export type LiveHandle<T> = {
  subscribe(listener: (action: LiveAction, result: T) => void): void;
  kill(): Promise<void>;
};

export type SurrealLike = SurrealOpsLike & {
  beginTransaction(): Promise<SurrealTransactionLike>;
  live<T>(table: string): Promise<LiveHandle<T>>;
  liveOf<T>(id: unknown): Promise<LiveHandle<T>>;
};

/** Surreal RETURN mode for createMany / updateMany / deleteMany (exclusive with select). */
export type MutationReturn = "NONE" | "BEFORE" | "AFTER" | "DIFF";

/** Result of *Many when using select and/or return (select wins typed projection). */
export type ManyReturnResult<
  S,
  R extends MutationReturn | undefined,
  Full,
  Payload,
> = [S] extends [undefined]
  ? R extends "DIFF"
    ? unknown[]
    : R extends "AFTER" | "BEFORE"
      ? Full[]
      : { count: number }
  : Payload[];

/** Result of single update when using select and/or return. */
export type SingleUpdateResult<
  S,
  R extends MutationReturn | undefined,
  Full,
  Payload,
> = [S] extends [undefined]
  ? R extends "DIFF"
    ? unknown
    : R extends "NONE"
      ? void
      : Full | undefined
  : Payload | undefined;

/** Result of single delete when using select and/or return (default void). */
export type SingleDeleteResult<
  S,
  R extends MutationReturn | undefined,
  Full,
  Payload,
> = [S] extends [undefined]
  ? R extends undefined
    ? void
    : R extends "DIFF"
      ? unknown
      : R extends "NONE"
        ? void
        : Full | undefined
  : Payload;

/** Result of upsert when using select and/or return. */
export type SingleUpsertResult<
  S,
  R extends MutationReturn | undefined,
  Full,
  Payload,
> = [S] extends [undefined]
  ? R extends "DIFF"
    ? unknown
    : R extends "NONE"
      ? void
      : R extends "BEFORE"
        ? Full | undefined
        : Full
  : Payload;

export function isNestedWriteBag(value: unknown): value is Record<string, unknown> {
  return !!value && typeof value === "object" && !Array.isArray(value) && (
    "create" in value || "connect" in value || "disconnect" in value
  );
}

export function assertNoNestedWrites(
  data: Record<string, unknown>,
  table: string,
  context: string,
): void {
  const meta = writeMetaByTable[table] ?? {};
  for (const [key, value] of Object.entries(data)) {
    const fieldMeta = meta[key];
    if (!fieldMeta || fieldMeta.kind === "scalar") continue;
    if (fieldMeta.kind === "stored" && isNestedWriteBag(value)) {
      throw new Error(`${context}: nested writes are one hop only (${table}.${key})`);
    }
    if ((fieldMeta.kind === "computed" || fieldMeta.kind === "edge") && value != null) {
      throw new Error(`${context}: nested writes are one hop only (${table}.${key})`);
    }
  }
}

export async function withWriteTransaction<T>(
  db: SurrealOpsLike,
  needsTxn: boolean,
  fn: (ops: SurrealOpsLike) => Promise<T>,
): Promise<T> {
  if (!needsTxn) return fn(db);
  const begin = (db as SurrealLike).beginTransaction;
  if (typeof begin !== "function") return fn(db);
  const txn = await begin.call(db);
  try {
    const result = await fn(txn);
    await txn.commit();
    return result;
  } catch (error) {
    await txn.cancel();
    throw error;
  }
}

export async function relateEdge(
  db: SurrealOpsLike,
  edgeTable: string,
  inTable: string,
  outTable: string,
  inId: string,
  outId: string,
  content: Record<string, unknown> = {},
): Promise<Record<string, unknown>> {
  const row = await db.query(`RELATE $in->${edgeTable}->$out CONTENT $content`, {
    in: normalizeThing(inTable, inId),
    out: normalizeThing(outTable, outId),
    content,
  });
  const created = await firstRow<Record<string, unknown>>(row);
  if (!created) throw new Error(`relate ${edgeTable} returned no row`);
  return created;
}

export async function queryScript(
  db: SurrealOpsLike,
  sql: string,
  vars?: Record<string, unknown>,
): Promise<unknown[]> {
  const result = await db.query(sql, vars);
  return Array.isArray(result) ? result : [result];
}

export function statementRows<T>(statement: unknown): T[] {
  if (Array.isArray(statement)) return statement as T[];
  if (statement == null) return [];
  return [statement as T];
}

export function readStoredLink(
  table: string,
  field: string,
  value: unknown,
  meta: Extract<FieldWriteMeta, { kind: "stored" }>,
): { value?: unknown; create?: Record<string, unknown> } {
  if (value == null || typeof value !== "object" || Array.isArray(value)) return { value };
  const bag = value as Record<string, unknown>;
  if ("disconnect" in bag) {
    if (!meta.optional) throw new Error(`update ${table}: cannot disconnect required link ${field}`);
    return { value: null };
  }
  if ("connect" in bag) {
    const id = (bag.connect as { id?: unknown } | undefined)?.id;
    if (typeof id !== "string") throw new Error(`update ${table}: ${field}.connect.id required`);
    return { value: normalizeThing(meta.targetTable, id) };
  }
  if ("create" in bag) {
    return { create: (bag.create ?? {}) as Record<string, unknown> };
  }
  return { value };
}

export async function insertStoredCreates(
  db: SurrealOpsLike,
  creates: { key: string; table: string; data: Record<string, unknown> }[],
): Promise<Record<string, string>> {
  const groups: { table: string; keys: string[]; rows: Record<string, unknown>[] }[] = [];
  for (const create of creates) {
    const last = groups[groups.length - 1];
    if (last && last.table === create.table) {
      last.keys.push(create.key);
      last.rows.push(create.data);
    } else {
      groups.push({ table: create.table, keys: [create.key], rows: [create.data] });
    }
  }
  const vars: Record<string, unknown> = {};
  const sql = groups.map((group, index) => {
    vars[`sc${index}`] = group.rows;
    return `INSERT INTO ${group.table} $sc${index} RETURN AFTER`;
  }).join(";\n");
  const statements = await queryScript(db, sql, vars);
  const ids: Record<string, string> = {};
  groups.forEach((group, index) => {
    const rows = statementRows<{ id?: unknown }>(statements[index]);
    group.keys.forEach((key, rowIndex) => {
      const id = rows[rowIndex]?.id;
      if (id == null) throw new Error(`create ${group.table} returned no row`);
      ids[key] = String(id);
    });
  });
  return ids;
}

export async function splitAndResolveWriteData(
  db: SurrealOpsLike,
  table: string,
  data: Record<string, unknown>,
): Promise<{ scalars: Record<string, unknown>; nested: Record<string, unknown> }> {
  const meta = writeMetaByTable[table] ?? {};
  const scalars: Record<string, unknown> = {};
  const nested: Record<string, unknown> = {};
  const storedCreates: { key: string; table: string; data: Record<string, unknown> }[] = [];
  for (const [key, value] of Object.entries(data)) {
    if (value === undefined) continue;
    const fieldMeta = meta[key];
    if (!fieldMeta || fieldMeta.kind === "scalar") {
      scalars[key] = value;
      continue;
    }
    if (fieldMeta.kind === "stored") {
      const resolved = readStoredLink(table, key, value, fieldMeta);
      if (resolved.create) {
        assertNoNestedWrites(resolved.create, fieldMeta.targetTable, `create ${table}.${key}`);
        storedCreates.push({ key, table: fieldMeta.targetTable, data: resolved.create });
      } else {
        scalars[key] = resolved.value;
      }
      continue;
    }
    if (fieldMeta.kind === "computed" || fieldMeta.kind === "edge") {
      if (!isNestedWriteBag(value)) {
        throw new Error(`${table}.${key}: expected { create|connect|disconnect }`);
      }
      nested[key] = value;
    }
  }
  if (storedCreates.length > 0) {
    const ids = await insertStoredCreates(db, storedCreates);
    for (const create of storedCreates) scalars[create.key] = ids[create.key];
  }
  return { scalars, nested };
}

export type NestedScriptOp =
  | { kind: "insert"; table: string; rows: Record<string, unknown>[] }
  | { kind: "merge"; ids: string[]; patch: Record<string, unknown> }
  | { kind: "unset"; ids: string[]; field: string }
  | { kind: "relate"; table: string; rows: Record<string, unknown>[] }
  | { kind: "deleteIds"; ids: string[] }
  | { kind: "deletePairs"; table: string; pairs: { from: string; to: string }[] }
  | {
      kind: "relateNewFar";
      edgeTable: string;
      farTable: string;
      farIsOut: boolean;
      items: { far: Record<string, unknown>; content: Record<string, unknown>; parent: string }[];
    };

export function bindNested(vars: Record<string, unknown>, value: unknown): string {
  const name = `nw${Object.keys(vars).length}`;
  vars[name] = value;
  return `$${name}`;
}

export function renderNestedOps(ops: NestedScriptOp[]): { sql: string; vars: Record<string, unknown> } {
  const vars: Record<string, unknown> = {};
  const parts: string[] = [];
  for (const op of ops) {
    if (op.kind === "insert") {
      parts.push(`INSERT INTO ${op.table} ${bindNested(vars, op.rows)} RETURN NONE`);
    } else if (op.kind === "merge") {
      parts.push(`UPDATE ${bindNested(vars, op.ids)} MERGE ${bindNested(vars, op.patch)}`);
    } else if (op.kind === "unset") {
      parts.push(`UPDATE ${bindNested(vars, op.ids)} UNSET ${op.field}`);
    } else if (op.kind === "relate") {
      parts.push(`INSERT RELATION INTO ${op.table} ${bindNested(vars, op.rows)} RETURN NONE`);
    } else if (op.kind === "deleteIds") {
      parts.push(`DELETE ${bindNested(vars, op.ids)}`);
    } else if (op.kind === "deletePairs") {
      parts.push(`FOR $pair IN ${bindNested(vars, op.pairs)} { DELETE ${op.table} WHERE in = $pair.from AND out = $pair.to }`);
    } else {
      const relate = op.farIsOut
        ? `$parent->${op.edgeTable}->$far`
        : `$far->${op.edgeTable}->$parent`;
      parts.push(`FOR $item IN ${bindNested(vars, op.items)} { LET $far = CREATE ONLY ${op.farTable} CONTENT $item.far; LET $parent = $item.parent; RELATE ${relate} CONTENT $item.content }`);
    }
  }
  return { sql: parts.join(";\n"), vars };
}

export async function runNestedOps(db: SurrealOpsLike, ops: NestedScriptOp[]): Promise<void> {
  if (ops.length === 0) return;
  const script = renderNestedOps(ops);
  await queryScript(db, script.sql, script.vars);
}

export function edgeContent(row: Record<string, unknown>, payloadFields: string[]): Record<string, unknown> {
  const content: Record<string, unknown> = {};
  for (const key of payloadFields) {
    if (row[key] !== undefined) content[key] = row[key];
  }
  return content;
}

export function relationRow(
  meta: Extract<FieldWriteMeta, { kind: "edge" }>,
  parentId: string,
  farId: string,
  content: Record<string, unknown>,
): Record<string, unknown> {
  const inId = meta.dir === "out" ? parentId : farId;
  const outId = meta.dir === "out" ? farId : parentId;
  return {
    ...content,
    in: normalizeThing(meta.inTable, inId),
    out: normalizeThing(meta.outTable, outId),
  };
}

export function edgeFarData(
  parentTable: string,
  parentId: string,
  farTable: string,
  createData: Record<string, unknown>,
): Record<string, unknown> {
  const data = { ...createData };
  const parentMeta = writeMetaByTable[farTable] ?? {};
  for (const [pkey, pmeta] of Object.entries(parentMeta)) {
    if (pmeta.kind === "stored" && pmeta.targetTable === parentTable && data[pkey] == null) {
      data[pkey] = normalizeThing(parentTable, parentId);
    }
  }
  return data;
}

export function collectComputed(
  ops: NestedScriptOp[],
  parentTable: string,
  parentId: string,
  field: string,
  bag: Record<string, unknown>,
  meta: Extract<FieldWriteMeta, { kind: "computed" }>,
): void {
  const parentThing = normalizeThing(parentTable, parentId);
  const creates = Array.isArray(bag.create) ? bag.create : [];
  const rows: Record<string, unknown>[] = [];
  for (const item of creates) {
    if (!item || typeof item !== "object") continue;
    const data = { ...(item as Record<string, unknown>), [meta.backLinkField]: parentThing };
    assertNoNestedWrites(data, meta.targetTable, `create ${parentTable}.${field}`);
    rows.push(data);
  }
  if (rows.length > 0) ops.push({ kind: "insert", table: meta.targetTable, rows });
  const connects = Array.isArray(bag.connect) ? bag.connect : [];
  const connectIds: string[] = [];
  for (const item of connects) {
    const id = item && typeof item === "object" ? (item as { id?: unknown }).id : undefined;
    if (typeof id !== "string") throw new Error(`${parentTable}.${field}.connect.id required`);
    connectIds.push(normalizeThing(meta.targetTable, id));
  }
  if (connectIds.length > 0) {
    ops.push({ kind: "merge", ids: connectIds, patch: { [meta.backLinkField]: parentThing } });
  }
  const disconnects = Array.isArray(bag.disconnect) ? bag.disconnect : [];
  const disconnectIds: string[] = [];
  for (const item of disconnects) {
    const id = item && typeof item === "object" ? (item as { id?: unknown }).id : undefined;
    if (typeof id !== "string") throw new Error(`${parentTable}.${field}.disconnect.id required`);
    if (!meta.backLinkOptional) {
      throw new Error(`${parentTable}.${field}: cannot disconnect required back-link ${meta.backLinkField}`);
    }
    disconnectIds.push(normalizeThing(meta.targetTable, id));
  }
  if (disconnectIds.length > 0) {
    ops.push({ kind: "unset", ids: disconnectIds, field: meta.backLinkField });
  }
}

export function collectEdge(
  ops: NestedScriptOp[],
  parentTable: string,
  parentId: string,
  field: string,
  bag: Record<string, unknown>,
  meta: Extract<FieldWriteMeta, { kind: "edge" }>,
): void {
  const farKey = meta.dir;
  const farTable = meta.dir === "out" ? meta.outTable : meta.inTable;
  const parentThing = normalizeThing(parentTable, parentId);
  const creates = Array.isArray(bag.create) ? bag.create : [];
  let relateRows: Record<string, unknown>[] = [];
  let farItems: { far: Record<string, unknown>; content: Record<string, unknown>; parent: string }[] = [];
  const flushRelate = () => {
    if (relateRows.length === 0) return;
    ops.push({ kind: "relate", table: meta.edgeTable, rows: relateRows });
    relateRows = [];
  };
  const flushFar = () => {
    if (farItems.length === 0) return;
    ops.push({ kind: "relateNewFar", edgeTable: meta.edgeTable, farTable, farIsOut: meta.dir === "out", items: farItems });
    farItems = [];
  };
  for (const item of creates) {
    if (!item || typeof item !== "object") continue;
    const row = item as Record<string, unknown>;
    const farVal = row[farKey];
    const content = edgeContent(row, meta.payloadFields);
    if (typeof farVal === "string") {
      flushFar();
      relateRows.push(relationRow(meta, parentId, farVal, content));
    } else if (farVal && typeof farVal === "object" && "create" in (farVal as object)) {
      const createData = { ...((farVal as { create?: Record<string, unknown> }).create ?? {}) };
      assertNoNestedWrites(createData, farTable, `create ${parentTable}.${field}.${farKey}`);
      flushRelate();
      farItems.push({ far: edgeFarData(parentTable, parentId, farTable, createData), content, parent: parentThing });
    } else {
      throw new Error(`${parentTable}.${field}.create.${farKey} required`);
    }
  }
  flushRelate();
  flushFar();
  const connects = Array.isArray(bag.connect) ? bag.connect : [];
  const connectRows: Record<string, unknown>[] = [];
  for (const item of connects) {
    if (!item || typeof item !== "object") continue;
    const row = item as Record<string, unknown>;
    if (typeof row.id === "string") {
      throw new Error(`${parentTable}.${field}.connect by edge id is not supported for connect (use create or out/in)`);
    }
    const farId = row[farKey];
    if (typeof farId !== "string") throw new Error(`${parentTable}.${field}.connect.${farKey} required`);
    connectRows.push(relationRow(meta, parentId, farId, edgeContent(row, meta.payloadFields)));
  }
  if (connectRows.length > 0) ops.push({ kind: "relate", table: meta.edgeTable, rows: connectRows });
  const disconnects = Array.isArray(bag.disconnect) ? bag.disconnect : [];
  let deleteIds: string[] = [];
  let deletePairs: { from: string; to: string }[] = [];
  const flushIds = () => {
    if (deleteIds.length === 0) return;
    ops.push({ kind: "deleteIds", ids: deleteIds });
    deleteIds = [];
  };
  const flushPairs = () => {
    if (deletePairs.length === 0) return;
    ops.push({ kind: "deletePairs", table: meta.edgeTable, pairs: deletePairs });
    deletePairs = [];
  };
  for (const item of disconnects) {
    if (!item || typeof item !== "object") continue;
    const row = item as Record<string, unknown>;
    if (typeof row.id === "string") {
      flushPairs();
      deleteIds.push(normalizeThing(meta.edgeTable, row.id));
      continue;
    }
    const farId = row[farKey];
    if (typeof farId !== "string") throw new Error(`${parentTable}.${field}.disconnect.${farKey} required`);
    flushIds();
    const ends = relationRow(meta, parentId, farId, {});
    deletePairs.push({ from: String(ends.in), to: String(ends.out) });
  }
  flushIds();
  flushPairs();
}

export async function applyNestedWrites(
  db: SurrealOpsLike,
  table: string,
  parentId: string,
  nested: Record<string, unknown>,
): Promise<void> {
  const meta = writeMetaByTable[table] ?? {};
  const ops: NestedScriptOp[] = [];
  for (const [field, value] of Object.entries(nested)) {
    const fieldMeta = meta[field];
    if (!fieldMeta || !isNestedWriteBag(value)) continue;
    if (fieldMeta.kind === "computed") {
      collectComputed(ops, table, parentId, field, value, fieldMeta);
    } else if (fieldMeta.kind === "edge") {
      collectEdge(ops, table, parentId, field, value, fieldMeta);
    }
  }
  await runNestedOps(db, ops);
}

export async function createWithNested<T extends { id?: unknown }>(
  db: SurrealOpsLike,
  table: string,
  data: Record<string, unknown>,
): Promise<T> {
  const hasNested = rowHasNestedWrite(table, data);
  return withWriteTransaction(db, hasNested, async (ops) => {
    const { scalars, nested } = await splitAndResolveWriteData(ops, table, data);
    const created = await createRecord<T>(ops, table, scalars);
    await applyNestedWrites(ops, table, String(created.id), nested);
    return created;
  });
}

export async function updateWithNested<T>(
  db: SurrealOpsLike,
  table: string,
  id: string,
  data: Record<string, unknown>,
  opts?: { select?: Record<string, unknown>; return?: MutationReturn },
): Promise<T | undefined | void | unknown> {
  const hasNested = rowHasNestedWrite(table, data);
  return withWriteTransaction(db, hasNested, async (ops) => {
    const { scalars, nested } = await splitAndResolveWriteData(ops, table, data);
    const updated = await updateOneRecord<T>(ops, table, id, scalars, opts);
    await applyNestedWrites(ops, table, id, nested);
    return updated;
  });
}

export async function selectRecord<T>(
  db: SurrealOpsLike,
  table: string,
  id: string,
): Promise<T | undefined> {
  const thing = normalizeThing(table, id);
  return firstRow<T>(await db.select(thing));
}

export async function selectRecordRelated<T>(
  db: SurrealOpsLike,
  table: string,
  id: string,
  fetch: string[],
): Promise<T | undefined> {
  const thing = normalizeThing(table, id);
  if (fetch.length === 0) return firstRow<T>(await db.select(thing));
  const row = await db.query(`SELECT * FROM type::record($thing) FETCH ${fetch.join(", ")}`, { thing });
  return firstRow<T>(row);
}

export async function createRecord<T>(
  db: SurrealOpsLike,
  table: string,
  data: Record<string, unknown>,
): Promise<T> {
  const created = await firstRow<T>(await db.create(table, data));
  if (!created) throw new Error(`create ${table} returned no row`);
  return created;
}

export async function updateRecord<T>(
  db: SurrealOpsLike,
  table: string,
  id: string,
  data: Record<string, unknown>,
): Promise<T | undefined> {
  const thing = normalizeThing(table, id);
  return firstRow<T>(await db.merge(thing, data));
}

export async function updateOneRecord<T>(
  db: SurrealOpsLike,
  table: string,
  id: string,
  data: Record<string, unknown>,
  opts?: { select?: Record<string, unknown>; return?: MutationReturn },
): Promise<T | undefined | void | unknown> {
  assertReturnSelectExclusive("update", table, opts?.select, opts?.return);
  const thing = normalizeThing(table, id);
  if (!opts?.select && opts?.return === undefined) {
    return firstRow<T>(await db.merge(thing, data));
  }
  const vars: Record<string, unknown> = { __data: data, thing };
  if (opts.select) {
    const projection = buildProjection(opts.select, table);
    const rows = await queryRows<T>(db, `UPDATE type::record($thing) MERGE $__data RETURN ${projection}`, vars);
    return rows[0];
  }
  if (opts.return === "NONE") {
    await queryRows(db, `UPDATE type::record($thing) MERGE $__data RETURN NONE`, vars);
    return;
  }
  if (opts.return === "DIFF") {
    const rows = await queryRows<unknown>(db, `UPDATE type::record($thing) MERGE $__data RETURN DIFF`, vars);
    return rows[0];
  }
  const rows = await queryRows<T>(db, `UPDATE type::record($thing) MERGE $__data RETURN ${opts.return}`, vars);
  return rows[0];
}

export async function deleteRecord(
  db: SurrealOpsLike,
  table: string,
  id: string,
): Promise<void> {
  await db.delete(normalizeThing(table, id));
}

export async function deleteOneRecord<T>(
  db: SurrealOpsLike,
  table: string,
  id: string,
  opts?: { select?: Record<string, unknown>; return?: MutationReturn },
): Promise<void | T | unknown> {
  assertReturnSelectExclusive("delete", table, opts?.select, opts?.return);
  if (opts?.return === "AFTER") {
    throw new Error(`delete ${table}: return AFTER is not supported (use BEFORE)`);
  }
  const thing = normalizeThing(table, id);
  if (!opts?.select && opts?.return === undefined) {
    await db.delete(thing);
    return;
  }
  const vars: Record<string, unknown> = { thing };
  if (opts.select) {
    const rows = await queryRows<Record<string, unknown>>(db, `DELETE type::record($thing) RETURN BEFORE`, vars);
    const row = rows[0];
    if (!row) throw new Error(`delete ${table}: no row returned for ${thing}`);
    return projectRow(row, opts.select) as T;
  }
  if (opts.return === "NONE") {
    await queryRows(db, `DELETE type::record($thing) RETURN NONE`, vars);
    return;
  }
  if (opts.return === "DIFF") {
    const rows = await queryRows<unknown>(db, `DELETE type::record($thing) RETURN DIFF`, vars);
    return rows[0];
  }
  const rows = await queryRows<T>(db, `DELETE type::record($thing) RETURN BEFORE`, vars);
  return rows[0];
}

export async function queryRows<T>(
  db: SurrealOpsLike,
  sql: string,
  vars?: Record<string, unknown>,
): Promise<T[]> {
  return asRows<T>(await db.query(sql, vars));
}

export function projectField(
  key: string,
  value: unknown,
  fieldMeta: FieldSelectMeta,
  allMeta: Record<string, Record<string, FieldSelectMeta>>,
): string | null {
  if (value === false || value == null) return null;
  if (fieldMeta.kind === "scalar") return key;
  if (value === true) {
    if (fieldMeta.kind === "edge") {
      const arrow = fieldMeta.dir === "out" ? `->${fieldMeta.edgeTable}` : `<-${fieldMeta.edgeTable}`;
      return `${arrow}.* AS ${key}`;
    }
    return `${key}.*`;
  }
  if (typeof value === "object" && value !== null && ("select" in value || "orderBy" in value || "take" in value || "skip" in value)) {
    const bag = value as { select?: Record<string, unknown>; orderBy?: Record<string, unknown> | Record<string, unknown>[]; take?: number; skip?: number };
    const nestedTable =
      fieldMeta.kind === "edge" ? fieldMeta.edgeTable : fieldMeta.targetTable;
    let nested = buildProjection(bag.select, nestedTable, allMeta);
    const countProj = collectOrderByCountProjections(bag.orderBy, nestedTable, allMeta);
    if (countProj.length > 0) nested = `${nested}, ${countProj.join(", ")}`;
    const orderClause = buildOrderBy(bag.orderBy, nestedTable, allMeta);
    const pageClause = nestedLimitClause(bag.take, bag.skip);
    if (orderClause || pageClause) {
      const tail = `${orderClause ? ` ${orderClause}` : ""}${pageClause}`;
      if (fieldMeta.kind === "edge") {
        const arrow = fieldMeta.dir === "out" ? "->" : "<-";
        return `${arrow}(SELECT ${nested} FROM ${fieldMeta.edgeTable}${tail}) AS ${key}`;
      }
      return `(SELECT ${nested} FROM $parent.${key}${tail}) AS ${key}`;
    }
    if (fieldMeta.kind === "edge") {
      const arrow = fieldMeta.dir === "out" ? `->${fieldMeta.edgeTable}` : `<-${fieldMeta.edgeTable}`;
      return `${arrow}.{ ${nested} } AS ${key}`;
    }
    return `${key}.{ ${nested} }`;
  }
  return null;
}

export function buildProjection(
  select: Record<string, unknown> | undefined,
  table: string,
  allMeta: Record<string, Record<string, FieldSelectMeta>> = selectMetaByTable,
): string {
  if (!select) return "*";
  const meta = allMeta[table] ?? {};
  const parts: string[] = [];
  for (const [key, value] of Object.entries(select)) {
    const fieldMeta = meta[key];
    if (!fieldMeta) continue;
    const part = projectField(key, value, fieldMeta, allMeta);
    if (part) parts.push(part);
  }
  return parts.length === 0 ? "*" : parts.join(", ");
}

export function uniqueWhereId(where: Record<string, unknown> | undefined): string | undefined {
  if (!where) return undefined;
  const keys = Object.keys(where).filter((key) => where[key] !== undefined);
  if (keys.length !== 1 || keys[0] !== "id") return undefined;
  const id = where.id;
  if (typeof id === "string") return id;
  if (id && typeof id === "object" && !Array.isArray(id) && "equals" in id) {
    const equals = (id as { equals?: unknown }).equals;
    return typeof equals === "string" ? equals : undefined;
  }
  return undefined;
}

export async function findUniqueRecord<T>(
  db: SurrealOpsLike,
  table: string,
  args: {
    where: Record<string, unknown>;
    select?: Record<string, unknown>;
    vars?: Record<string, unknown>;
  },
): Promise<T | undefined> {
  const idOnly = uniqueWhereId(args.where);
  if (idOnly !== undefined) {
    if (!args.select) return selectRecord<T>(db, table, idOnly);
    const thing = normalizeThing(table, idOnly);
    const projection = buildProjection(args.select, table);
    const row = await db.query(`SELECT ${projection} FROM type::record($thing)`, { thing });
    return firstRow<T>(row);
  }
  const projection = args.select ? buildProjection(args.select, table) : "*";
  const vars: Record<string, unknown> = { ...(args.vars ?? {}) };
  const whereClause = buildWhere(args.where, table, selectMetaByTable, { vars, n: 0 });
  if (!whereClause) return undefined;
  const rows = await queryRows<T>(db, `SELECT ${projection} FROM ${table} WHERE ${whereClause} LIMIT 1`, vars);
  return rows[0];
}

export async function findManyRecords<T>(
  db: SurrealOpsLike,
  table: string,
  args: {
    select?: Record<string, unknown>;
    where?: Record<string, unknown>;
    orderBy?: Record<string, unknown> | Record<string, unknown>[];
    take?: number;
    skip?: number;
    whereSql?: string;
    vars?: Record<string, unknown>;
  } = {},
): Promise<T[]> {
  let projection = args.select ? buildProjection(args.select, table) : "*";
  const countProj = collectOrderByCountProjections(args.orderBy, table);
  if (countProj.length > 0) {
    projection = projection === "*" ? `*, ${countProj.join(", ")}` : `${projection}, ${countProj.join(", ")}`;
  }
  const vars: Record<string, unknown> = { ...(args.vars ?? {}) };
  let sql: string;
  if (args.where) {
    const whereClause = buildWhere(args.where, table, selectMetaByTable, { vars, n: 0 });
    sql = whereClause
      ? `SELECT ${projection} FROM ${table} WHERE ${whereClause}`
      : `SELECT ${projection} FROM ${table}`;
  } else if (args.whereSql) {
    sql = args.whereSql.replace(/^\s*SELECT\s+\*/i, `SELECT ${projection}`);
  } else {
    sql = `SELECT ${projection} FROM ${table}`;
  }
  const orderClause = buildOrderBy(args.orderBy, table);
  if (orderClause) sql = `${sql} ${orderClause}`;
  sql = appendLimitStart(sql, args.take, args.skip, vars);
  const rows = await queryRows<T>(db, sql, vars);
  return stripOrderByCountFields(rows);
}

export function assertNonEmptyWhere(
  where: Record<string, unknown> | undefined,
  op: string,
  table: string,
): void {
  if (!where) throw new Error(`${op} ${table}: where must not be empty`);
  const keys = Object.keys(where).filter((key) => where[key] !== undefined);
  if (keys.length === 0) throw new Error(`${op} ${table}: where must not be empty`);
}

export function assertReturnSelectExclusive(
  op: string,
  table: string,
  select: Record<string, unknown> | undefined,
  ret: MutationReturn | undefined,
): void {
  if (select && ret !== undefined) {
    throw new Error(`${op} ${table}: select and return are mutually exclusive`);
  }
}

export async function countRecords(
  db: SurrealOpsLike,
  table: string,
  args: { where?: Record<string, unknown>; vars?: Record<string, unknown> } = {},
): Promise<number> {
  const vars: Record<string, unknown> = { ...(args.vars ?? {}) };
  let sql = `SELECT count() AS count FROM ${table}`;
  if (args.where) {
    const whereClause = buildWhere(args.where, table, selectMetaByTable, { vars, n: 0 });
    if (whereClause) sql += ` WHERE ${whereClause}`;
  }
  sql += " GROUP ALL";
  const rows = await queryRows<{ count?: number }>(db, sql, vars);
  return Number(rows[0]?.count ?? 0);
}

export async function countMatching(
  db: SurrealOpsLike,
  table: string,
  whereClause: string,
  vars: Record<string, unknown>,
): Promise<number> {
  const rows = await queryRows<{ count?: number }>(db, `SELECT count() AS count FROM ${table} WHERE ${whereClause} GROUP ALL`, vars);
  return Number(rows[0]?.count ?? 0);
}

export type GroupByAlias = { kind: string; field: string; alias: string };

export function buildGroupByOrderBy(
  orderBy: Record<string, unknown> | Record<string, unknown>[] | undefined,
  by: string[],
): string {
  if (!orderBy) return "";
  const items = Array.isArray(orderBy) ? orderBy : [orderBy];
  const bySet = new Set(by);
  const parts: string[] = [];
  for (const item of items) {
    if (!item || typeof item !== "object") continue;
    for (const [key, value] of Object.entries(item)) {
      if (key === "_count" && value && typeof value === "object") {
        const all = (value as { _all?: unknown })._all;
        if (all === "asc" || all === "desc") {
          parts.push(`__count_all ${String(all).toUpperCase()}`);
        }
        continue;
      }
      if ((value === "asc" || value === "desc") && bySet.has(key)) {
        parts.push(`${key} ${value.toUpperCase()}`);
      }
    }
  }
  return parts.length === 0 ? "" : `ORDER BY ${parts.join(", ")}`;
}

export function reshapeGroupByRow(
  row: Record<string, unknown>,
  by: string[],
  aliases: GroupByAlias[],
): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  for (const field of by) out[field] = row[field];
  for (const { kind, field, alias } of aliases) {
    if (!out[kind]) out[kind] = {};
    const bucket = out[kind] as Record<string, unknown>;
    if (kind === "_count") {
      bucket[field] = Number(row[alias] ?? 0);
    } else {
      bucket[field] = row[alias] ?? null;
    }
  }
  return out;
}

export function assertAggregateFilter(
  table: string,
  kind: string,
  field: string,
  filter: unknown,
): void {
  if (!filter || typeof filter !== "object" || Array.isArray(filter)) return;
  for (const op of Object.keys(filter as Record<string, unknown>)) {
    if (op === "equals" || op === "gt" || op === "gte" || op === "lt" || op === "lte") continue;
    throw new Error(`groupBy ${table}: having ${kind}.${field} does not support ${op}`);
  }
}

export function buildGroupHaving(
  table: string,
  having: Record<string, unknown> | undefined,
  by: string[],
  aliases: GroupByAlias[],
  ctx: WhereBuildCtx,
): string {
  if (!having) return "";
  const bySet = new Set(by);
  const parts: string[] = [];
  for (const [key, value] of Object.entries(having)) {
    if (value == null) continue;
    if (key === "_count" || key === "_sum" || key === "_avg" || key === "_min" || key === "_max") {
      if (!value || typeof value !== "object" || Array.isArray(value)) {
        throw new Error(`groupBy ${table}: having ${key} must be an object`);
      }
      for (const [field, filter] of Object.entries(value as Record<string, unknown>)) {
        if (filter == null) continue;
        const alias = aliases.find((item) => item.kind === key && item.field === field);
        if (!alias) throw new Error(`groupBy ${table}: having ${key}.${field} was not selected`);
        assertAggregateFilter(table, key, field, filter);
        parts.push(...scalarPredicate(alias.alias, filter, "number", ctx));
      }
      continue;
    }
    if (!bySet.has(key)) throw new Error(`groupBy ${table}: having ${key} was not selected`);
    const fieldMeta = selectMetaByTable[table]?.[key];
    const filterKind = fieldMeta && fieldMeta.kind === "scalar" ? fieldMeta.filter : "string";
    parts.push(...scalarPredicate(key, value, filterKind, ctx));
  }
  return parts.join(" AND ");
}

export async function groupByRecords(
  db: SurrealOpsLike,
  table: string,
  args: {
    by: string[];
    where?: Record<string, unknown>;
    _count?: true | Record<string, boolean | undefined>;
    _sum?: Record<string, boolean | undefined>;
    _avg?: Record<string, boolean | undefined>;
    _min?: Record<string, boolean | undefined>;
    _max?: Record<string, boolean | undefined>;
    having?: Record<string, unknown>;
    orderBy?: Record<string, unknown> | Record<string, unknown>[];
    take?: number;
    skip?: number;
    vars?: Record<string, unknown>;
  },
): Promise<Record<string, unknown>[]> {
  if (!args.by || args.by.length === 0) {
    throw new Error(`groupBy ${table}: by must not be empty`);
  }
  const countSpec = args._count === true ? { _all: true as const } : args._count;
  const hasCount = !!(countSpec && Object.values(countSpec).some((v) => v === true));
  const hasMath = [args._sum, args._avg, args._min, args._max].some(
    (spec) => !!spec && Object.values(spec).some((v) => v === true),
  );
  if (!hasCount && !hasMath) {
    throw new Error(`groupBy ${table}: at least one aggregate (_count/_sum/_avg/_min/_max) is required`);
  }
  const vars: Record<string, unknown> = { ...(args.vars ?? {}) };
  const whereCtx: WhereBuildCtx = { vars, n: 0 };
  const selectParts: string[] = [...args.by];
  const aliases: GroupByAlias[] = [];
  if (countSpec) {
    if (countSpec._all === true) {
      selectParts.push("count() AS __count_all");
      aliases.push({ kind: "_count", field: "_all", alias: "__count_all" });
    }
    for (const [field, on] of Object.entries(countSpec)) {
      if (field === "_all" || on !== true) continue;
      const alias = `__count_${field}`;
      selectParts.push(`count(${field}) AS ${alias}`);
      aliases.push({ kind: "_count", field, alias });
    }
  }
  const mathOps: Array<[string, string, Record<string, boolean | undefined> | undefined]> = [
    ["_sum", "math::sum", args._sum],
    ["_avg", "math::mean", args._avg],
    ["_min", "math::min", args._min],
    ["_max", "math::max", args._max],
  ];
  for (const [kind, fn, spec] of mathOps) {
    if (!spec) continue;
    for (const [field, on] of Object.entries(spec)) {
      if (on !== true) continue;
      const alias = `__${kind.slice(1)}_${field}`;
      selectParts.push(`${fn}(${field}) AS ${alias}`);
      aliases.push({ kind, field, alias });
    }
  }
  let sql = `SELECT ${selectParts.join(", ")} FROM ${table}`;
  if (args.where) {
    const whereClause = buildWhere(args.where, table, selectMetaByTable, whereCtx);
    if (whereClause) sql += ` WHERE ${whereClause}`;
  }
  sql += ` GROUP BY ${args.by.join(", ")}`;
  const havingClause = buildGroupHaving(table, args.having, args.by, aliases, whereCtx);
  if (havingClause) sql = `SELECT * FROM (${sql}) WHERE ${havingClause}`;
  const orderClause = buildGroupByOrderBy(args.orderBy, args.by);
  if (orderClause) sql += ` ${orderClause}`;
  sql = appendLimitStart(sql, args.take, args.skip, vars);
  const rows = await queryRows<Record<string, unknown>>(db, sql, vars);
  return rows.map((row) => reshapeGroupByRow(row, args.by, aliases));
}

export function projectRow(
  row: Record<string, unknown>,
  select: Record<string, unknown> | undefined,
): Record<string, unknown> {
  if (!select) return row;
  const out: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(select)) {
    if (value === false || value == null) continue;
    if (value === true) {
      out[key] = row[key];
      continue;
    }
    if (typeof value === "object" && value !== null && "select" in value) {
      const nestedSelect = (value as { select?: Record<string, unknown> }).select;
      const nested = row[key];
      if (Array.isArray(nested)) {
        out[key] = nested.map((item) =>
          item && typeof item === "object"
            ? projectRow(item as Record<string, unknown>, nestedSelect)
            : item,
        );
      } else if (nested && typeof nested === "object") {
        out[key] = projectRow(nested as Record<string, unknown>, nestedSelect);
      } else {
        out[key] = nested;
      }
    }
  }
  return out;
}

export function rowHasNestedWrite(table: string, data: Record<string, unknown>): boolean {
  const meta = writeMetaByTable[table] ?? {};
  return Object.entries(data).some(([key, value]) => {
    const fieldMeta = meta[key];
    if (!fieldMeta) return false;
    if (fieldMeta.kind === "computed" || fieldMeta.kind === "edge") return value != null;
    if (fieldMeta.kind === "stored") return isNestedWriteBag(value);
    return false;
  });
}

export async function insertManyRows<T>(
  db: SurrealOpsLike,
  table: string,
  rows: Record<string, unknown>[],
  relation: boolean,
  returning: string,
): Promise<T[]> {
  if (rows.length === 0) return [];
  const keyword = relation ? "INSERT RELATION INTO" : "INSERT INTO";
  return queryRows<T>(db, `${keyword} ${table} $rows RETURN ${returning}`, { rows });
}

export async function createManyVia<T extends { id?: unknown }>(
  db: SurrealOpsLike,
  table: string,
  args: {
    data: Record<string, unknown>[];
    select?: Record<string, unknown>;
    return?: MutationReturn;
  },
  createOne: (data: Record<string, unknown>) => Promise<T>,
  relation: boolean,
): Promise<{ count: number } | T[] | unknown[]> {
  assertReturnSelectExclusive("createMany", table, args.select, args.return);
  if (args.return === "BEFORE") {
    throw new Error(`createMany ${table}: return BEFORE is not supported (no prior row)`);
  }
  const hasNested = args.data.some((data) => rowHasNestedWrite(table, data));
  if (hasNested) {
    if (args.return === "DIFF") {
      const diffs: unknown[] = [];
      for (const data of args.data) {
        const part = await queryRows<unknown>(db, `CREATE ${table} CONTENT $__row RETURN DIFF`, { __row: data });
        diffs.push(...part);
      }
      return diffs;
    }
    if (args.select) {
      const rows: T[] = [];
      for (const data of args.data) {
        const created = await createOne(data);
        const id = created?.id != null ? String(created.id) : undefined;
        if (id === undefined) throw new Error(`createMany ${table}: created row missing id`);
        const projected = await findUniqueRecord<T>(db, table, { where: { id }, select: args.select });
        if (!projected) throw new Error(`createMany ${table}: could not reload ${id}`);
        rows.push(projected);
      }
      return rows;
    }
    if (args.return === "AFTER") {
      const rows: T[] = [];
      for (const data of args.data) {
        rows.push(await createOne(data));
      }
      return rows;
    }
    for (const data of args.data) {
      await createOne(data);
    }
    return { count: args.data.length };
  }
  if (args.return === "DIFF") {
    const parts = await insertManyRows<unknown[]>(db, table, args.data, relation, "DIFF");
    return parts.flat();
  }
  if (args.select) {
    const projection = buildProjection(args.select, table);
    return insertManyRows<T>(db, table, args.data, relation, projection);
  }
  if (args.return === "AFTER") {
    return insertManyRows<T>(db, table, args.data, relation, "AFTER");
  }
  await insertManyRows(db, table, args.data, relation, "NONE");
  return { count: args.data.length };
}

export async function updateManyRecords<T>(
  db: SurrealOpsLike,
  table: string,
  args: {
    where: Record<string, unknown>;
    data: Record<string, unknown>;
    select?: Record<string, unknown>;
    return?: MutationReturn;
  },
): Promise<{ count: number } | T[] | unknown[]> {
  assertNonEmptyWhere(args.where, "updateMany", table);
  assertReturnSelectExclusive("updateMany", table, args.select, args.return);
  const vars: Record<string, unknown> = { __data: args.data };
  const whereClause = buildWhere(args.where, table, selectMetaByTable, { vars, n: 0 });
  if (!whereClause) throw new Error(`updateMany ${table}: where must not be empty`);
  if (args.select) {
    const projection = buildProjection(args.select, table);
    return queryRows<T>(db, `UPDATE ${table} MERGE $__data WHERE ${whereClause} RETURN ${projection}`, vars);
  }
  if (args.return === "NONE") {
    const count = await countMatching(db, table, whereClause, vars);
    await queryRows(db, `UPDATE ${table} MERGE $__data WHERE ${whereClause} RETURN NONE`, vars);
    return { count };
  }
  if (args.return === "DIFF") {
    return queryRows<unknown>(db, `UPDATE ${table} MERGE $__data WHERE ${whereClause} RETURN DIFF`, vars);
  }
  if (args.return === "BEFORE" || args.return === "AFTER") {
    return queryRows<T>(db, `UPDATE ${table} MERGE $__data WHERE ${whereClause} RETURN ${args.return}`, vars);
  }
  const rows = await queryRows<T>(db, `UPDATE ${table} MERGE $__data WHERE ${whereClause} RETURN AFTER`, vars);
  return { count: rows.length };
}

export async function deleteManyRecords<T>(
  db: SurrealOpsLike,
  table: string,
  args: {
    where: Record<string, unknown>;
    select?: Record<string, unknown>;
    return?: MutationReturn;
  },
): Promise<{ count: number } | T[] | unknown[]> {
  assertNonEmptyWhere(args.where, "deleteMany", table);
  assertReturnSelectExclusive("deleteMany", table, args.select, args.return);
  if (args.return === "AFTER") {
    throw new Error(`deleteMany ${table}: return AFTER is not supported (use BEFORE)`);
  }
  const vars: Record<string, unknown> = {};
  const whereClause = buildWhere(args.where, table, selectMetaByTable, { vars, n: 0 });
  if (!whereClause) throw new Error(`deleteMany ${table}: where must not be empty`);
  if (args.select) {
    const rows = await queryRows<Record<string, unknown>>(db, `DELETE ${table} WHERE ${whereClause} RETURN BEFORE`, vars);
    return rows.map((row) => projectRow(row, args.select) as T);
  }
  if (args.return === "NONE") {
    const count = await countMatching(db, table, whereClause, vars);
    await queryRows(db, `DELETE ${table} WHERE ${whereClause} RETURN NONE`, vars);
    return { count };
  }
  if (args.return === "DIFF") {
    return queryRows<unknown>(db, `DELETE ${table} WHERE ${whereClause} RETURN DIFF`, vars);
  }
  if (args.return === "BEFORE") {
    return queryRows<T>(db, `DELETE ${table} WHERE ${whereClause} RETURN BEFORE`, vars);
  }
  const rows = await queryRows<Record<string, unknown>>(db, `DELETE ${table} WHERE ${whereClause} RETURN BEFORE`, vars);
  return { count: rows.length };
}

export async function upsertRecord<T extends { id?: unknown }>(
  db: SurrealOpsLike,
  table: string,
  args: {
    where: Record<string, unknown>;
    create: Record<string, unknown>;
    update: Record<string, unknown>;
    select?: Record<string, unknown>;
    return?: MutationReturn;
  },
  createOne: (data: Record<string, unknown>) => Promise<T> = (data) =>
    createRecord<T>(db, table, data),
): Promise<T | void | unknown> {
  assertNonEmptyWhere(args.where, "upsert", table);
  assertReturnSelectExclusive("upsert", table, args.select, args.return);
  const existing = await findUniqueRecord<{ id?: unknown }>(db, table, { where: args.where });
  if (existing?.id != null) {
    const id = String(existing.id);
    if (args.return === "BEFORE") {
      const before = await findUniqueRecord<T>(db, table, { where: { id } });
      await updateOneRecord(db, table, id, args.update, { return: "NONE" });
      return before;
    }
    if (args.return === "NONE") {
      await updateOneRecord(db, table, id, args.update, { return: "NONE" });
      return;
    }
    if (args.return === "DIFF") {
      return updateOneRecord(db, table, id, args.update, { return: "DIFF" });
    }
    if (args.select) {
      return updateOneRecord<T>(db, table, id, args.update, { select: args.select }) as Promise<T>;
    }
    const updated = await updateOneRecord<T>(db, table, id, args.update, { return: "AFTER" });
    if (updated === undefined) throw new Error(`upsert ${table}: update returned no row`);
    return updated as T;
  }
  if (args.return === "BEFORE") {
    await createOne(args.create);
    return undefined;
  }
  if (args.return === "NONE") {
    await createOne(args.create);
    return;
  }
  if (args.return === "DIFF") {
    const rows = await queryRows<unknown>(db, `CREATE ${table} CONTENT $__row RETURN DIFF`, { __row: args.create });
    return rows[0];
  }
  const created = await createOne(args.create);
  if (!args.select) return created;
  const id = created?.id != null ? String(created.id) : undefined;
  if (id === undefined) throw new Error(`upsert ${table}: created row missing id`);
  const projected = await findUniqueRecord<T>(db, table, { where: { id }, select: args.select });
  if (!projected) throw new Error(`upsert ${table}: could not reload after create`);
  return projected;
}

export function relationCountExpr(
  key: string,
  fieldMeta: FieldSelectMeta,
): string | null {
  if (fieldMeta.kind === "scalar" || !fieldMeta.list) return null;
  if (fieldMeta.kind === "edge") {
    const arrow = fieldMeta.dir === "out" ? `->${fieldMeta.edgeTable}` : `<-${fieldMeta.edgeTable}`;
    return `array::len(${arrow})`;
  }
  if (fieldMeta.kind === "stored" || fieldMeta.kind === "computed") {
    return `array::len(${key})`;
  }
  return null;
}

export function orderByCountAlias(key: string): string {
  return `__ob_${key}`;
}

export function collectOrderByCountProjections(
  orderBy: Record<string, unknown> | Record<string, unknown>[] | undefined,
  table: string,
  allMeta: Record<string, Record<string, FieldSelectMeta>> = selectMetaByTable,
): string[] {
  if (!orderBy) return [];
  const items = Array.isArray(orderBy) ? orderBy : [orderBy];
  const meta = allMeta[table] ?? {};
  const parts: string[] = [];
  const seen = new Set<string>();
  for (const item of items) {
    if (!item || typeof item !== "object") continue;
    for (const [key, value] of Object.entries(item)) {
      if (!value || typeof value !== "object" || Array.isArray(value) || !("_count" in value)) continue;
      const dir = (value as { _count?: unknown })._count;
      if (dir !== "asc" && dir !== "desc") continue;
      const fieldMeta = meta[key];
      if (!fieldMeta) continue;
      const expr = relationCountExpr(key, fieldMeta);
      if (!expr || seen.has(key)) continue;
      seen.add(key);
      parts.push(`${expr} AS ${orderByCountAlias(key)}`);
    }
  }
  return parts;
}

export function stripOrderByCountFields<T>(rows: T[]): T[] {
  return rows.map((row) => {
    if (!row || typeof row !== "object") return row;
    const out = { ...(row as Record<string, unknown>) };
    for (const key of Object.keys(out)) {
      if (key.startsWith("__ob_")) delete out[key];
    }
    return out as T;
  });
}

export function buildOrderBy(
  orderBy: Record<string, unknown> | Record<string, unknown>[] | undefined,
  table: string,
  allMeta: Record<string, Record<string, FieldSelectMeta>> = selectMetaByTable,
): string {
  if (!orderBy) return "";
  const items = Array.isArray(orderBy) ? orderBy : [orderBy];
  const meta = allMeta[table] ?? {};
  const parts: string[] = [];
  for (const item of items) {
    if (!item || typeof item !== "object") continue;
    for (const [key, value] of Object.entries(item)) {
      const fieldMeta = meta[key];
      if (!fieldMeta) continue;
      if (value === "asc" || value === "desc") {
        if (fieldMeta.kind !== "scalar") continue;
        parts.push(`${key} ${value.toUpperCase()}`);
        continue;
      }
      if (value && typeof value === "object" && !Array.isArray(value) && "_count" in value) {
        const dir = (value as { _count?: unknown })._count;
        if (dir !== "asc" && dir !== "desc") continue;
        if (!relationCountExpr(key, fieldMeta)) continue;
        parts.push(`${orderByCountAlias(key)} ${String(dir).toUpperCase()}`);
      }
    }
  }
  return parts.length === 0 ? "" : `ORDER BY ${parts.join(", ")}`;
}

export function nestedLimitClause(take: number | undefined, skip: number | undefined): string {
  let clause = "";
  if (take !== undefined) {
    if (!Number.isFinite(take) || take < 0) throw new Error("take must be a non-negative number");
    clause += ` LIMIT ${Math.floor(take)}`;
  }
  if (skip !== undefined) {
    if (!Number.isFinite(skip) || skip < 0) throw new Error("skip must be a non-negative number");
    clause += ` START ${Math.floor(skip)}`;
  }
  return clause;
}

export function appendLimitStart(
  sql: string,
  take: number | undefined,
  skip: number | undefined,
  vars: Record<string, unknown>,
): string {
  let out = sql;
  if (take !== undefined) {
    if (!Number.isFinite(take) || take < 0) throw new Error("take must be a non-negative number");
    vars.__take = Math.floor(take);
    out = out + " LIMIT $__take";
  }
  if (skip !== undefined) {
    if (!Number.isFinite(skip) || skip < 0) throw new Error("skip must be a non-negative number");
    vars.__skip = Math.floor(skip);
    out = out + " START $__skip";
  }
  return out;
}

export type WhereBuildCtx = { vars: Record<string, unknown>; n: number };

export function nextWhereVar(ctx: WhereBuildCtx, value: unknown): string {
  const key = `w${ctx.n++}`;
  ctx.vars[key] = value;
  return `$${key}`;
}

export function isScalarFilterObject(value: unknown): value is Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) return false;
  return ["equals", "in", "contains", "gt", "gte", "lt", "lte"].some((k) =>
    Object.prototype.hasOwnProperty.call(value, k),
  );
}

export function scalarPredicate(
  path: string,
  value: unknown,
  filter: string,
  ctx: WhereBuildCtx,
): string[] {
  if (value === undefined) return [];
  if (!isScalarFilterObject(value)) {
    return [`${path} = ${nextWhereVar(ctx, value)}`];
  }
  const parts: string[] = [];
  if (value.equals !== undefined) parts.push(`${path} = ${nextWhereVar(ctx, value.equals)}`);
  if (value.in !== undefined) parts.push(`${path} IN ${nextWhereVar(ctx, value.in)}`);
  if (value.contains !== undefined) {
    if (filter === "array") parts.push(`${path} CONTAINS ${nextWhereVar(ctx, value.contains)}`);
    else parts.push(`string::contains(${path}, ${nextWhereVar(ctx, value.contains)})`);
  }
  if (value.gt !== undefined) parts.push(`${path} > ${nextWhereVar(ctx, value.gt)}`);
  if (value.gte !== undefined) parts.push(`${path} >= ${nextWhereVar(ctx, value.gte)}`);
  if (value.lt !== undefined) parts.push(`${path} < ${nextWhereVar(ctx, value.lt)}`);
  if (value.lte !== undefined) parts.push(`${path} <= ${nextWhereVar(ctx, value.lte)}`);
  return parts;
}

export function relationMode(value: Record<string, unknown>, list: boolean): {
  mode: "some" | "every" | "none" | "is";
  nested: unknown;
} {
  if ("some" in value) return { mode: "some", nested: value.some };
  if ("every" in value) return { mode: "every", nested: value.every };
  if ("none" in value) return { mode: "none", nested: value.none };
  if ("is" in value) return { mode: "is", nested: value.is };
  return { mode: list ? "some" : "is", nested: value };
}

export function listRelationPredicate(
  collectionExpr: string,
  mode: "some" | "every" | "none" | "is",
  nestedClause: string,
): string | null {
  if (mode === "is") return null;
  if (mode === "some") {
    return nestedClause
      ? `array::len(${collectionExpr}[WHERE ${nestedClause}]) > 0`
      : `array::len(${collectionExpr}) > 0`;
  }
  if (mode === "every") {
    return nestedClause
      ? `array::len(${collectionExpr}[WHERE ${nestedClause}]) = array::len(${collectionExpr})`
      : "true";
  }
  return nestedClause
    ? `array::len(${collectionExpr}[WHERE ${nestedClause}]) = 0`
    : `array::len(${collectionExpr}) = 0`;
}

export function buildWhere(
  where: Record<string, unknown> | undefined,
  table: string,
  allMeta: Record<string, Record<string, FieldSelectMeta>> = selectMetaByTable,
  ctx: WhereBuildCtx = { vars: {}, n: 0 },
  pathPrefix = "",
): string {
  if (!where) return "";
  const meta = allMeta[table] ?? {};
  const parts: string[] = [];
  for (const [key, value] of Object.entries(where)) {
    if (value === undefined) continue;
    if (key === "AND") {
      const items = Array.isArray(value) ? value : [value];
      const inner = items
        .map((item) => buildWhere(item as Record<string, unknown>, table, allMeta, ctx, pathPrefix))
        .filter(Boolean);
      if (inner.length) parts.push(`(${inner.join(" AND ")})`);
      continue;
    }
    if (key === "OR") {
      const items = Array.isArray(value) ? value : [value];
      const inner = items
        .map((item) => buildWhere(item as Record<string, unknown>, table, allMeta, ctx, pathPrefix))
        .filter(Boolean);
      if (inner.length) parts.push(`(${inner.join(" OR ")})`);
      continue;
    }
    if (key === "NOT") {
      const items = Array.isArray(value) ? value : [value];
      const inner = items
        .map((item) => buildWhere(item as Record<string, unknown>, table, allMeta, ctx, pathPrefix))
        .filter(Boolean);
      if (inner.length === 1) parts.push(`!(${inner[0]})`);
      else if (inner.length > 1) parts.push(`!((${inner.join(" AND ")}))`);
      continue;
    }
    const fieldMeta = meta[key];
    if (!fieldMeta) continue;
    const path = pathPrefix ? `${pathPrefix}.${key}` : key;
    if (fieldMeta.kind === "scalar") {
      parts.push(...scalarPredicate(path, value, fieldMeta.filter, ctx));
      continue;
    }
    if (!value || typeof value !== "object" || Array.isArray(value)) continue;
    const { mode, nested } = relationMode(value as Record<string, unknown>, fieldMeta.list);
    if (fieldMeta.kind === "edge") {
      const arrow = fieldMeta.dir === "out" ? `->${fieldMeta.edgeTable}` : `<-${fieldMeta.edgeTable}`;
      const nestedClause = buildWhere(
        nested as Record<string, unknown> | undefined,
        fieldMeta.edgeTable,
        allMeta,
        ctx,
      );
      const pred = listRelationPredicate(arrow, mode, nestedClause);
      if (pred) parts.push(pred);
      continue;
    }
    if (fieldMeta.list) {
      const nestedClause = buildWhere(
        nested as Record<string, unknown> | undefined,
        fieldMeta.targetTable,
        allMeta,
        ctx,
      );
      const pred = listRelationPredicate(path, mode, nestedClause);
      if (pred) parts.push(pred);
      continue;
    }
    const nestedClause = buildWhere(
      nested as Record<string, unknown> | undefined,
      fieldMeta.targetTable,
      allMeta,
      ctx,
      path,
    );
    if (nestedClause) parts.push(nestedClause);
  }
  return parts.join(" AND ");
}
