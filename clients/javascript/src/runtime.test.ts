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
