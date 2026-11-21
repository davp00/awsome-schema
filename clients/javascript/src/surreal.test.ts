import { Surreal } from "surrealdb";
import { describe, expect, it } from "vitest";

import { asSurrealLike, encodeRecordIdString } from "./surreal.js";

describe("asSurrealLike", () => {
  it("accepts a surrealdb Surreal connection", () => {
    const session = asSurrealLike(new Surreal());
    expect(session.merge).toEqual(expect.any(Function));
    expect(session.query).toEqual(expect.any(Function));
    expect(session.beginTransaction).toEqual(expect.any(Function));
    expect(session.live).toEqual(expect.any(Function));
  });

  it("returns the same session when the client is already adapted", () => {
    const session = asSurrealLike(new Surreal());
    expect(asSurrealLike(session as unknown as Surreal)).toBe(session);
  });
});

describe("encodeRecordIdString", () => {
  it("escapes hyphenated ids for Surreal 4 StringRecordId", () => {
    expect(String(encodeRecordIdString("post:missing-abc"))).toBe("post:⟨missing-abc⟩");
    expect(String(encodeRecordIdString("post:abc"))).toBe("post:abc");
    expect(String(encodeRecordIdString("post:⟨already⟩"))).toBe("post:⟨already⟩");
  });
});
