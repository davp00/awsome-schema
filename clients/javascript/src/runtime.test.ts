import { describe, expect, it } from "vitest";

import {
  asRecordId,
  appendLimitStart,
  bindSchemaMeta,
  buildGroupHaving,
  buildOrderBy,
  buildProjection,
  buildWhere,
  collectComputed,
  collectEdge,
  findManyRecords,
  isNestedWriteBag,
  parseRecordId,
  recordId,
  splitAndResolveWriteData,
  updateWithNested,
  type FieldSelectMeta,
  type FieldWriteMeta,
  type SurrealOpsLike,
} from "./runtime.js";

const selectMeta: Record<string, Record<string, FieldSelectMeta>> = {
  user: {
    email: { kind: "scalar", filter: "string" },
    posts: { kind: "computed", targetTable: "post", list: true },
  },
};

describe("record ids", () => {
  it("brands a table and key", () => {
    expect(recordId("user", "1")).toBe("user:1");
    expect(parseRecordId("user:1")).toEqual({ table: "user", key: "1" });
    expect(parseRecordId("nope")).toBeNull();
    expect(asRecordId("user", "1")).toBe("user:1");
    expect(asRecordId("user", "user:9")).toBe("user:9");
  });
});

describe("query fragments", () => {
  it("builds a string contains predicate and a relation count order", () => {
    bindSchemaMeta(selectMeta, {});
    const where = buildWhere({ email: { contains: "ada" } }, "user", selectMeta);
    expect(where).toContain("string::contains(email,");
    const order = buildOrderBy({ posts: { _count: "desc" } }, "user", selectMeta);
    expect(order).toBe("ORDER BY __ob_posts DESC");
  });

  it("compares record id and stored link identity via type::record", () => {
    const meta: Record<string, Record<string, FieldSelectMeta>> = {
      user: {
        id: { kind: "scalar", filter: "id" },
        email: { kind: "scalar", filter: "string" },
      },
      post: {
        id: { kind: "scalar", filter: "id" },
        author: { kind: "stored", targetTable: "user", list: false },
      },
      likes: {
        id: { kind: "scalar", filter: "id" },
        out: { kind: "stored", targetTable: "post", list: false },
        score: { kind: "scalar", filter: "number" },
      },
    };
    const idCtx = { vars: {}, n: 0 };
    expect(buildWhere({ id: "user:1" }, "user", meta, idCtx)).toBe("id = type::record($w0)");
    expect(idCtx.vars.w0).toBe("user:1");

    const linkCtx = { vars: {}, n: 0 };
    expect(buildWhere({ author: { id: "user:9" } }, "post", meta, linkCtx)).toBe(
      "author = type::record($w0)",
    );
    expect(linkCtx.vars.w0).toBe("user:9");

    const edgeCtx = { vars: {}, n: 0 };
    expect(buildWhere({ out: { id: "post:2" } }, "likes", meta, edgeCtx)).toBe(
      "out = type::record($w0)",
    );

    const scalarCtx = { vars: {}, n: 0 };
    expect(buildWhere({ email: "a@b.c" }, "user", meta, scalarCtx)).toBe("email = $w0");
    expect(scalarCtx.vars.w0).toBe("a@b.c");
  });

  it("emits ORDER BY id when take/skip has no orderBy", async () => {
    bindSchemaMeta(
      {
        user: {
          id: { kind: "scalar", filter: "id" },
          email: { kind: "scalar", filter: "string" },
        },
      },
      {},
    );
    const queries: string[] = [];
    const db = {
      query: async (sql: string) => {
        queries.push(sql);
        return [];
      },
      select: async () => [],
      create: async () => [],
      merge: async () => [],
      delete: async () => [],
    } satisfies SurrealOpsLike;
    await findManyRecords(db, "user", { take: 2, skip: 1 });
    expect(queries[0]).toContain("ORDER BY id");
    expect(queries[0]).toContain("LIMIT $__take");
    expect(queries[0]).toContain("START $__skip");

    const nested = buildProjection(
      { posts: { select: { id: true }, take: 1 } },
      "user",
      {
        user: {
          posts: { kind: "computed", targetTable: "post", list: true },
        },
        post: { id: { kind: "scalar", filter: "id" } },
      },
    );
    expect(nested).toContain("ORDER BY id");
    expect(nested).toContain("LIMIT 1");

    const vars: Record<string, unknown> = {};
    expect(appendLimitStart("SELECT * FROM user", 3, undefined, vars)).toContain("LIMIT $__take");
  });

  it("compiles nested object filters, whole-value equals, and array contains", () => {
    const meta: Record<string, Record<string, FieldSelectMeta>> = {
      user: {
        metadata: {
          kind: "object",
          fields: {
            source: { kind: "scalar", filter: "string" },
            user_id: { kind: "scalar", filter: "number" },
          },
        },
        profile: { kind: "scalar", filter: "json" },
        tags: { kind: "scalar", filter: "array" },
      },
    };
    const nested = { vars: {}, n: 0 };
    expect(buildWhere({ metadata: { source: "where-e2e" } }, "user", meta, nested)).toBe(
      "metadata.source = $w0",
    );
    expect(nested.vars.w0).toBe("where-e2e");

    const contains = { vars: {}, n: 0 };
    expect(
      buildWhere({ metadata: { source: { contains: "web" } } }, "user", meta, contains),
    ).toBe("string::contains(metadata.source, $w0)");

    const whole = { vars: {}, n: 0 };
    const value = { source: "where-e2e", user_id: 3 };
    expect(buildWhere({ metadata: { equals: value } }, "user", meta, whole)).toBe("metadata = $w0");
    expect(whole.vars.w0).toEqual(value);

    const tags = { vars: {}, n: 0 };
    expect(buildWhere({ tags: { contains: "filter" } }, "user", meta, tags)).toBe(
      "tags CONTAINS $w0",
    );
    expect(tags.vars.w0).toBe("filter");

    const bare = { vars: {}, n: 0 };
    expect(buildWhere({ profile: { foo: 1 } }, "user", meta, bare)).toBe("profile = $w0");

    const ignored = { vars: {}, n: 0 };
    expect(buildWhere({ metadata: { extra: true, source: "a" } }, "user", meta, ignored)).toBe(
      "metadata.source = $w0",
    );
  });

  it("compiles nested object filters in groupBy having", () => {
    const meta: Record<string, Record<string, FieldSelectMeta>> = {
      user: {
        metadata: {
          kind: "object",
          fields: {
            source: { kind: "scalar", filter: "string" },
          },
        },
      },
    };
    bindSchemaMeta(meta, {});
    const ctx = { vars: {}, n: 0 };
    expect(
      buildGroupHaving("user", { metadata: { source: "web" } }, ["metadata"], [], ctx),
    ).toBe("metadata.source = $w0");
    expect(ctx.vars.w0).toBe("web");
  });
});

describe("nested writes", () => {
  it("detects a one-hop write bag", () => {
    expect(isNestedWriteBag({ create: {} })).toBe(true);
    expect(isNestedWriteBag({ connect: { id: "post:1" } })).toBe(true);
    expect(isNestedWriteBag({ disconnect: true })).toBe(true);
    expect(isNestedWriteBag({ set: [] })).toBe(true);
    expect(isNestedWriteBag({ update: [] })).toBe(true);
    expect(isNestedWriteBag({ delete: [] })).toBe(true);
    expect(isNestedWriteBag({ upsert: [] })).toBe(true);
    expect(isNestedWriteBag({ connectOrCreate: [] })).toBe(true);
    expect(isNestedWriteBag({ email: "a@b.c" })).toBe(false);
    expect(isNestedWriteBag(null)).toBe(false);
    expect(isNestedWriteBag(["create"])).toBe(false);
  });

  const posts: Extract<FieldWriteMeta, { kind: "computed" }> = {
    kind: "computed",
    targetTable: "post",
    list: true,
    backLinkField: "author",
    backLinkOptional: false,
    uniqueFields: [],
  };
  const liked: Extract<FieldWriteMeta, { kind: "edge" }> = {
    kind: "edge",
    edgeTable: "likes",
    dir: "out",
    list: true,
    inTable: "user",
    outTable: "post",
    payloadFields: ["score"],
  };

  function fakeDb(rowsFor: (sql: string, vars?: Record<string, unknown>) => unknown[]) {
    const queries: { sql: string; vars?: Record<string, unknown> }[] = [];
    const merges: { thing: string; data: Record<string, unknown> }[] = [];
    const db = {
      query: async (sql: string, vars?: Record<string, unknown>) => {
        queries.push({ sql, vars });
        return rowsFor(sql, vars);
      },
      select: async () => [],
      create: async () => [],
      merge: async (thing: string, data?: Record<string, unknown>) => {
        merges.push({ thing, data: data ?? {} });
        return [{ id: thing, ...(data ?? {}) }];
      },
      delete: async () => [],
    } satisfies SurrealOpsLike;
    return { db, queries, merges };
  }

  function script(queries: { sql: string }[]): string {
    return queries.map((query) => query.sql).join("\n");
  }

  it("replaces optional computed membership and rejects a required unlink", async () => {
    const optional = { ...posts, backLinkOptional: true };
    const replaced = fakeDb((sql) =>
      sql.includes("WHERE author = type::record") ? [{ id: "post:old" }, { id: "post:keep" }] : [],
    );
    await collectComputed(replaced.db, [], "user", "user:1", "posts", {
      set: [{ id: "keep" }, { id: "post:new" }],
    }, optional);
    expect(script(replaced.queries)).toContain("author = type::record($v)");
    expect(script(replaced.queries)).toContain("UNSET author");
    expect(script(replaced.queries)).toContain("MERGE");
    expect(replaced.queries.some((query) =>
      JSON.stringify(query.vars).includes("post:old")
    )).toBe(true);

    const required = fakeDb(() => [{ id: "post:old" }]);
    await expect(collectComputed(required.db, [], "user", "user:1", "posts", {
      set: [{ id: "post:new" }],
    }, posts)).rejects.toThrow(/cannot disconnect required back-link/);
  });

  it("updates, deletes, upserts, and connectOrCreates computed rows", async () => {
    const db = fakeDb((sql, vars) => {
      if (!sql.includes("id = type::record($v)")) return [];
      return vars?.v === "post:missing" ? [] : [{ id: "post:1" }];
    });
    await collectComputed(db.db, [], "user", "user:1", "posts", {
      update: [{ where: { id: "post:1" }, data: { title: "edited" } }],
      delete: [{ id: "post:2" }],
      upsert: [{ where: { id: "missing" }, update: { title: "n" }, create: { title: "made" } }],
      connectOrCreate: [{ where: { id: "post:1" }, create: { title: "unused" } }],
    }, posts);
    const sql = script(db.queries);
    expect(sql).toContain("id = type::record($v)");
    expect(sql).toContain("DELETE");
    expect(sql).toContain("MERGE");
    expect(sql).toContain("INSERT INTO post");
    expect(sql).not.toContain("WHERE title");
  });

  it("sets and upserts edge rows with type::record lookups", async () => {
    const db = fakeDb((sql) => {
      if (sql.includes("WHERE in = type::record($v)")) return [{ id: "likes:old" }];
      if (sql.includes("out = type::record($far)")) return [{ id: "likes:edge" }];
      return [];
    });
    await collectEdge(db.db, [], "user", "user:1", "liked", {
      set: [{ out: "post:a", score: 1 }],
      upsert: [{ where: { out: "post:a" }, update: { score: 4 }, create: { out: "post:a", score: 4 } }],
    }, liked);
    const sql = script(db.queries);
    expect(sql).toContain("in = type::record($v)");
    expect(sql).toContain("out = type::record($far)");
    expect(sql).toContain("DELETE");
    expect(sql).toContain("INSERT RELATION INTO likes");
    expect(sql).toContain("MERGE");
  });

  it("resolves stored connectOrCreate and rejects required delete", async () => {
    const writeMeta: Record<string, Record<string, FieldWriteMeta>> = {
      post: {
        author: {
          kind: "stored",
          targetTable: "user",
          list: false,
          optional: false,
          uniqueFields: ["email"],
        },
      },
    };
    bindSchemaMeta({}, writeMeta);
    const found = fakeDb((sql) => (sql.includes("WHERE email = $v") ? [{ id: "user:9" }] : []));
    const resolved = await splitAndResolveWriteData(found.db, "post", {
      title: "t",
      author: {
        connectOrCreate: {
          where: { email: "ada@example.com" },
          create: { email: "ada@example.com" },
        },
      },
    });
    expect(resolved.scalars.author).toBe("user:9");
    expect(resolved.scalars.title).toBe("t");
    expect(script(found.queries)).toContain("email = $v");
    expect(script(found.queries)).not.toContain("email = type::record");

    const missing = fakeDb((sql) => (sql.includes("INSERT") ? [{ id: "user:new" }] : []));
    const created = await splitAndResolveWriteData(missing.db, "post", {
      author: {
        connectOrCreate: {
          where: { email: "new@example.com" },
          create: { email: "new@example.com" },
        },
      },
    });
    expect(created.scalars.author).toBe("user:new");

    await expect(splitAndResolveWriteData(fakeDb(() => []).db, "post", {
      author: { delete: true },
    }, "post:1")).rejects.toThrow(/cannot delete required link/);
  });

  it("clears an optional stored link before deleting the related row", async () => {
    const writeMeta: Record<string, Record<string, FieldWriteMeta>> = {
      post: {
        author: {
          kind: "stored",
          targetTable: "user",
          list: false,
          optional: true,
          uniqueFields: ["email"],
        },
        title: { kind: "scalar" },
      },
    };
    bindSchemaMeta({}, writeMeta);
    const steps: string[] = [];
    const { db, merges } = fakeDb((sql, vars) => {
      if (sql.includes("SELECT author FROM type::record($thing)")) {
        steps.push(`select:${String(vars?.thing)}`);
        return [{ author: "user:old" }];
      }
      if (sql.includes("UNSET author")) {
        steps.push(`unset:${String(vars?.thing)}`);
        return [];
      }
      if (sql.includes("DELETE type::record($thing)")) {
        steps.push(`delete:${String(vars?.thing)}`);
        return [];
      }
      if (sql.includes("SELECT * FROM") || sql.includes("select")) {
        return [{ id: "post:1", author: null }];
      }
      return [{ id: "post:1" }];
    });
    const originalMerge = db.merge.bind(db);
    db.merge = async (thing: string, data?: Record<string, unknown>) => {
      steps.push(`merge:${thing}`);
      return originalMerge(thing, data);
    };
    const originalSelect = db.select.bind(db);
    db.select = async (thing: string) => {
      steps.push(`selectRow:${thing}`);
      return originalSelect(thing);
    };
    await updateWithNested(db, "post", "post:1", { author: { delete: true } });
    expect(steps).toEqual(["select:post:1", "unset:post:1", "selectRow:post:1", "delete:user:old"]);
    expect(merges).toHaveLength(0);
  });
});
