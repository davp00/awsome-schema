import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client count and groupBy", () => {
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

  it.skipIf(skip)("counts with and without where", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const created = [];
    for (const age of [10, 20, 30]) {
      created.push(
        await client.user.create({
          email: `agg-count-${age}-${suffix}@example.com`,
          age,
          metadata: { source: "aggregate-e2e" },
          tags: ["agg"],
        }),
      );
    }

    const filtered = await client.user.count({
      where: { email: { contains: suffix } },
    });
    expect(filtered).toBe(3);

    const older = await client.user.count({
      where: { email: { contains: suffix }, age: { gte: 20 } },
    });
    expect(older).toBe(2);

    const total = await client.user.count();
    expect(total).toBeGreaterThanOrEqual(3);

    for (const row of created) {
      await client.user.delete(recordIdString(row.id));
    }
  });

  it.skipIf(skip)("groupBy age with numeric aggregates, orderBy, take", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const created = [];
    for (const age of [10, 10, 20, 30, 30, 30]) {
      created.push(
        await client.user.create({
          email: `agg-gb-${age}-${created.length}-${suffix}@example.com`,
          age,
          metadata: { source: "aggregate-e2e" },
          tags: ["groupby"],
        }),
      );
    }

    const groups = await client.user.groupBy({
      by: ["age"],
      where: { email: { contains: suffix } },
      _count: true,
      _sum: { age: true },
      _avg: { age: true },
      _min: { age: true },
      _max: { age: true },
      orderBy: { _count: { _all: "desc" } },
      take: 2,
    });

    expect(groups).toHaveLength(2);
    expect(groups[0]?.age).toBe(30);
    expect((groups[0]?._count as { _all?: number })?._all).toBe(3);
    expect((groups[0]?._sum as { age?: number })?.age).toBe(90);
    expect((groups[0]?._avg as { age?: number })?.age).toBe(30);
    expect((groups[0]?._min as { age?: number })?.age).toBe(30);
    expect((groups[0]?._max as { age?: number })?.age).toBe(30);

    expect(groups[1]?.age).toBe(10);
    expect((groups[1]?._count as { _all?: number })?._all).toBe(2);

    const counted = await client.user.groupBy({
      by: ["age"],
      where: { email: { contains: suffix } },
      _count: true,
      having: { _count: { _all: { gt: 1 } } },
      orderBy: { age: "asc" },
    });
    expect(counted.map((group) => group.age)).toEqual([10, 30]);

    const older = await client.user.groupBy({
      by: ["age"],
      where: { email: { contains: suffix } },
      _count: true,
      having: { age: { gte: 20 } },
      orderBy: { age: "asc" },
    });
    expect(older.map((group) => group.age)).toEqual([20, 30]);

    await expect(
      client.user.groupBy({ by: [], _count: true } as never),
    ).rejects.toThrow(/by must not be empty/);

    await expect(
      client.user.groupBy({ by: ["age"] } as never),
    ).rejects.toThrow(/at least one aggregate/);

    for (const row of created) {
      await client.user.delete(recordIdString(row.id));
    }
  });

  it.skipIf(skip)("edge count and groupBy on score", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `agg-edge-${suffix}@example.com`,
      age: 22,
      metadata: { source: "aggregate-e2e" },
      tags: ["edge"],
    });
    const userId = recordIdString(user.id);
    const posts = [];
    for (let i = 0; i < 3; i++) {
      posts.push(
        await client.post.create({
          title: `agg-edge-${i}-${suffix}`,
          author: userId,
        }),
      );
    }
    const likes = [];
    for (const [i, score] of [
      [0, 1],
      [1, 1],
      [2, 5],
    ] as const) {
      likes.push(
        await client.likes.create({
          in: userId,
          out: recordIdString(posts[i]!.id),
          score,
        }),
      );
    }

    const likeCount = await client.likes.count({
      where: { in: { id: userId } },
    });
    expect(likeCount).toBe(3);

    const byScore = await client.likes.groupBy({
      by: ["score"],
      where: { in: { id: userId } },
      _count: true,
      _sum: { score: true },
      orderBy: { score: "asc" },
    });
    expect(byScore).toHaveLength(2);
    expect(byScore[0]?.score).toBe(1);
    expect((byScore[0]?._count as { _all?: number })?._all).toBe(2);
    expect((byScore[0]?._sum as { score?: number })?.score).toBe(2);
    expect(byScore[1]?.score).toBe(5);
    expect((byScore[1]?._count as { _all?: number })?._all).toBe(1);

    const frequent = await client.likes.groupBy({
      by: ["score"],
      where: { in: { id: userId } },
      _count: true,
      having: { _count: { _all: { gt: 1 } } },
    });
    expect(frequent).toHaveLength(1);
    expect(frequent[0]?.score).toBe(1);

    for (const like of likes) {
      await client.likes.delete(recordIdString(like.id));
    }
    for (const post of posts) {
      await client.post.delete(recordIdString(post.id));
    }
    await client.user.delete(userId);
  });
});
