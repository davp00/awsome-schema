import { afterAll, beforeAll, describe, expect, it } from "vitest";

import { openGeneratedClient, readE2eEnv } from "../src/setupClient.js";

describe("generated TypeScript client *Many return modes", () => {
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

  it.skipIf(skip)("createMany return AFTER and NONE", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;

    const after = await client.user.createMany({
      data: [
        {
          email: `ret-a-${suffix}@example.com`,
          age: 11,
          metadata: { source: "return-e2e" },
          tags: ["ret"],
        },
        {
          email: `ret-b-${suffix}@example.com`,
          age: 12,
          metadata: { source: "return-e2e" },
          tags: ["ret"],
        },
      ],
      return: "AFTER",
    });
    expect(after).toHaveLength(2);
    expect(after.map((row) => row.email).sort()).toEqual([
      `ret-a-${suffix}@example.com`,
      `ret-b-${suffix}@example.com`,
    ]);

    const none = await client.user.createMany({
      data: [
        {
          email: `ret-c-${suffix}@example.com`,
          age: 13,
          metadata: { source: "return-e2e" },
          tags: ["ret"],
        },
      ],
      return: "NONE",
    });
    expect(none).toEqual({ count: 1 });

    await expect(
      client.user.createMany({
        data: [
          {
            email: `ret-bad-${suffix}@example.com`,
            age: 1,
            metadata: { source: "return-e2e" },
            tags: ["ret"],
          },
        ],
        return: "BEFORE",
      } as never),
    ).rejects.toThrow(/BEFORE is not supported/);
  });

  it.skipIf(skip)("updateMany return AFTER DIFF NONE", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const marker = `upd-ret-${suffix}`;
    await client.user.createMany({
      data: [
        {
          email: `${marker}-a@example.com`,
          age: 20,
          metadata: { source: "return-e2e" },
          tags: ["upd-ret"],
        },
        {
          email: `${marker}-b@example.com`,
          age: 21,
          metadata: { source: "return-e2e" },
          tags: ["upd-ret"],
        },
      ],
    });

    const after = await client.user.updateMany({
      where: { email: { contains: marker } },
      data: { age: 30 },
      return: "AFTER",
    });
    expect(after).toHaveLength(2);
    expect(after.every((row) => row.age === 30)).toBe(true);

    const diff = await client.user.updateMany({
      where: { email: { contains: marker } },
      data: { tags: ["upd-ret", "patched"] },
      return: "DIFF",
    });
    expect(diff.length).toBeGreaterThan(0);

    const none = await client.user.updateMany({
      where: { email: { contains: marker } },
      data: { age: 31 },
      return: "NONE",
    });
    expect(none).toEqual({ count: 2 });

    const check = await client.user.findMany({
      where: { email: { contains: marker } },
    });
    expect(check.every((row) => row.age === 31)).toBe(true);

    await expect(
      client.user.updateMany({
        where: { email: { contains: marker } },
        data: { age: 32 },
        select: { id: true },
        return: "AFTER",
      } as never),
    ).rejects.toThrow(/mutually exclusive/);
  });

  it.skipIf(skip)("deleteMany return BEFORE and rejects AFTER", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const marker = `del-ret-${suffix}`;
    await client.user.createMany({
      data: [
        {
          email: `${marker}-a@example.com`,
          age: 1,
          metadata: { source: "return-e2e" },
          tags: ["del-ret"],
        },
        {
          email: `${marker}-b@example.com`,
          age: 2,
          metadata: { source: "return-e2e" },
          tags: ["del-ret"],
        },
      ],
    });

    await expect(
      client.user.deleteMany({
        where: { email: { contains: marker } },
        return: "AFTER",
      } as never),
    ).rejects.toThrow(/AFTER is not supported/);

    const before = await client.user.deleteMany({
      where: { email: { contains: marker } },
      return: "BEFORE",
    });
    expect(before).toHaveLength(2);
    expect(before.every((row) => String(row.email).includes(marker))).toBe(true);

    const remaining = await client.user.findMany({
      where: { email: { contains: marker } },
    });
    expect(remaining).toHaveLength(0);
  });
});
