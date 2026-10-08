import { describe, expect, it } from "vitest";

import {
  asRecordId,
  bindSchemaMeta,
  buildOrderBy,
  buildWhere,
  isNestedWriteBag,
  parseRecordId,
  recordId,
  type FieldSelectMeta,
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
});

describe("nested writes", () => {
  it("detects a one-hop write bag", () => {
    expect(isNestedWriteBag({ create: {} })).toBe(true);
    expect(isNestedWriteBag({ connect: { id: "post:1" } })).toBe(true);
    expect(isNestedWriteBag({ disconnect: true })).toBe(true);
    expect(isNestedWriteBag({ email: "a@b.c" })).toBe(false);
    expect(isNestedWriteBag(null)).toBe(false);
    expect(isNestedWriteBag(["create"])).toBe(false);
  });
});
