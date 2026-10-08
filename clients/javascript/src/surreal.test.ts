import { Surreal } from "surrealdb";
import { describe, expect, it } from "vitest";

import { asSurrealLike } from "./surreal.js";

describe("asSurrealLike", () => {
  it("accepts a surrealdb Surreal connection", () => {
    const session = asSurrealLike(new Surreal());
    expect(session.merge).toEqual(expect.any(Function));
    expect(session.query).toEqual(expect.any(Function));
    expect(session.beginTransaction).toEqual(expect.any(Function));
    expect(session.live).toEqual(expect.any(Function));
  });
});
