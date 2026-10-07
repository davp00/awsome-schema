import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client orderBy take/skip", () => {
  const env = readE2eEnv();
  const skip = env.skipped;

  let close: (() => Promise<void>) | undefined;
  let client: Awaited<ReturnType<typeof openGeneratedClient>>["client"];

  beforeAll(async () => {
    if (skip) return;
    const opened = await openGeneratedClient(env);
    client = opened.client;
    close = async () => {
      await opened.db.close();
    };
  });

  afterAll(async () => {
    await close?.();
  });

  it.skipIf(skip)("orders, pages, and filters findMany results", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const created: Array<{ id: unknown; age?: number | null }> = [];

    for (const [age, label] of [
      [30, "a"],
      [10, "b"],
      [20, "c"],
    ] as const) {
      const user = await client.user.create({
        email: `order-${label}-${suffix}@example.com`,
        age,
        metadata: { source: "order-e2e" },
        tags: ["order"],
      });
      created.push(user);
    }

    const ids = new Set(created.map((row) => recordIdString(row.id)));

    const asc = await client.user.findMany({
      where: { email: { contains: suffix } },
      orderBy: { age: "asc" },
    });
    const ascOurs = asc.filter((row) => ids.has(recordIdString(row.id)));
    expect(ascOurs.map((row) => row.age)).toEqual([10, 20, 30]);

    const multi = await client.user.findMany({
      where: { email: { contains: suffix } },
      orderBy: [{ age: "desc" }, { email: "asc" }],
    });
    const multiOurs = multi.filter((row) => ids.has(recordIdString(row.id)));
    expect(multiOurs.map((row) => row.age)).toEqual([30, 20, 10]);

    const page = await client.user.findMany({
      where: { email: { contains: suffix } },
      orderBy: { age: "asc" },
      take: 2,
      skip: 1,
    });
    const pageOurs = page.filter((row) => ids.has(recordIdString(row.id)));
    expect(pageOurs.map((row) => row.age)).toEqual([20, 30]);

    const filtered = await client.user.findMany({
      where: {
        email: { contains: suffix },
        age: { gte: 20 },
      },
      orderBy: { age: "asc" },
    });
    const filteredOurs = filtered.filter((row) => ids.has(recordIdString(row.id)));
    expect(filteredOurs.map((row) => row.age)).toEqual([20, 30]);

    for (const row of created) {
      await client.user.delete(recordIdString(row.id));
    }
  });
});
