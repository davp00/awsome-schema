import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client bulk ops + upsert", () => {
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

  it.skipIf(skip)("createMany returns count and persists rows", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const result = await client.user.createMany({
      data: [
        {
          email: `bulk-a-${suffix}@example.com`,
          age: 21,
          metadata: { source: "bulk-e2e" },
          tags: ["bulk"],
        },
        {
          email: `bulk-b-${suffix}@example.com`,
          age: 22,
          metadata: { source: "bulk-e2e" },
          tags: ["bulk"],
        },
        {
          email: `bulk-c-${suffix}@example.com`,
          age: 23,
          metadata: { source: "bulk-e2e" },
          tags: ["bulk"],
        },
      ],
    });
    expect(result).toEqual({ count: 3 });

    const projected = await client.user.createMany({
      data: [
        {
          email: `bulk-d-${suffix}@example.com`,
          age: 24,
          metadata: { source: "bulk-e2e" },
          tags: ["bulk"],
        },
      ],
      select: { email: true, age: true },
    });
    expect(projected).toEqual([{ email: `bulk-d-${suffix}@example.com`, age: 24 }]);

    const found = await client.user.findMany({
      where: { email: { contains: suffix } },
    });
    expect(found).toHaveLength(4);
  });

  it.skipIf(skip)("updateMany count and select RETURN projection", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    await client.user.createMany({
      data: [
        {
          email: `upd-a-${suffix}@example.com`,
          age: 40,
          metadata: { source: "bulk-e2e" },
          tags: ["upd"],
        },
        {
          email: `upd-b-${suffix}@example.com`,
          age: 41,
          metadata: { source: "bulk-e2e" },
          tags: ["upd"],
        },
      ],
    });

    const counted = await client.user.updateMany({
      where: { email: { contains: suffix } },
      data: { tags: ["upd", "touched"] },
    });
    expect(counted).toEqual({ count: 2 });

    const projected = await client.user.updateMany({
      where: { email: { contains: suffix } },
      data: { age: 50 },
      select: { id: true, email: true, age: true },
    });
    expect(projected).toHaveLength(2);
    for (const row of projected) {
      expect(row.age).toBe(50);
      expect(row.email).toContain(suffix);
      expect(row.id).toBeTruthy();
      expect("tags" in row).toBe(false);
    }
  });

  it.skipIf(skip)("upsert creates then updates by unique email", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `upsert-${suffix}@example.com`;

    const created = await client.user.upsert({
      where: { email },
      create: {
        email,
        age: 10,
        metadata: { source: "bulk-e2e" },
        tags: ["upsert"],
      },
      update: { age: 99 },
    });
    expect(created.email).toBe(email);
    expect(created.age).toBe(10);

    const updated = await client.user.upsert({
      where: { email },
      create: {
        email,
        age: 0,
        metadata: { source: "bulk-e2e" },
        tags: ["upsert"],
      },
      update: { age: 33 },
      select: { id: true, email: true, age: true },
    });
    expect(updated.email).toBe(email);
    expect(updated.age).toBe(33);
    expect("tags" in updated).toBe(false);
  });

  it.skipIf(skip)("deleteMany with select returns deleted ids", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    await client.user.createMany({
      data: [
        {
          email: `del-a-${suffix}@example.com`,
          age: 1,
          metadata: { source: "bulk-e2e" },
          tags: ["del"],
        },
        {
          email: `del-b-${suffix}@example.com`,
          age: 2,
          metadata: { source: "bulk-e2e" },
          tags: ["del"],
        },
      ],
    });

    const deleted = await client.user.deleteMany({
      where: { email: { contains: suffix } },
      select: { id: true, email: true },
    });
    expect(deleted).toHaveLength(2);
    for (const row of deleted) {
      expect(row.email).toContain(suffix);
      expect(row.id).toBeTruthy();
    }

    const remaining = await client.user.findMany({
      where: { email: { contains: suffix } },
    });
    expect(remaining).toHaveLength(0);
  });

  it.skipIf(skip)(
    "updateMany with where returns nested link and relation select",
    async () => {
      const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
      const email = `rel-upd-${suffix}@example.com`;

      const user = await client.user.create({
        email,
        age: 28,
        metadata: { source: "bulk-e2e" },
        tags: ["rel-upd"],
      });
      const userId = recordIdString(user.id);
      const post = await client.post.create({
        title: `rel-upd-${suffix}`,
        author: userId,
      });
      const postId = recordIdString(post.id);
      const like = await client.likes.create({
        in: userId,
        out: postId,
        score: 7,
      });
      const likeId = recordIdString(like.id);

      await expect(
        client.user.updateMany({
          where: {},
          data: { age: 29 },
        }),
      ).rejects.toThrow(/where must not be empty/);

      const updated = await client.user.updateMany({
        where: {
          email: { contains: suffix },
          posts: { some: { title: { contains: suffix } } },
        },
        data: { age: 29, tags: ["rel-upd", "nested"] },
        select: {
          id: true,
          email: true,
          age: true,
          posts: {
            select: {
              id: true,
              title: true,
              author: {
                select: {
                  email: true,
                },
              },
            },
          },
          liked: {
            select: {
              id: true,
              score: true,
              out: {
                select: {
                  id: true,
                  title: true,
                },
              },
            },
          },
        },
      });

      expect(updated).toHaveLength(1);
      const row = updated[0]!;
      expect(recordIdString(row.id)).toBe(userId);
      expect(row.email).toBe(email);
      expect(row.age).toBe(29);
      expect("tags" in row).toBe(false);

      const posts = asArray(row.posts);
      const nestedPost = posts.find((p) => recordIdString(p.id) === postId);
      expect(nestedPost).toBeDefined();
      expect(nestedPost!.title).toBe(`rel-upd-${suffix}`);
      expect(nestedPost!.author?.email).toBe(email);

      const likes = asArray(row.liked);
      const nestedLike = likes.find((l) => recordIdString(l.id) === likeId);
      expect(nestedLike).toBeDefined();
      expect(nestedLike!.score).toBe(7);
      expect(recordIdString(nestedLike!.out.id)).toBe(postId);
      expect(nestedLike!.out.title).toBe(`rel-upd-${suffix}`);

      await client.likes.delete(likeId);
      await client.post.delete(postId);
      await client.user.delete(userId);
    },
  );

  it.skipIf(skip)("edge createMany and deleteMany count path", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `edge-bulk-${suffix}@example.com`,
      age: 25,
      metadata: { source: "bulk-e2e" },
      tags: ["edge"],
    });
    const postA = await client.post.create({
      title: `edge-a-${suffix}`,
      author: recordIdString(user.id),
    });
    const postB = await client.post.create({
      title: `edge-b-${suffix}`,
      author: recordIdString(user.id),
    });

    const created = await client.likes.createMany({
      data: [
        {
          in: recordIdString(user.id),
          out: recordIdString(postA.id),
          score: 1,
        },
        {
          in: recordIdString(user.id),
          out: recordIdString(postB.id),
          score: 2,
        },
      ],
    });
    expect(created).toEqual({ count: 2 });

    const removed = await client.likes.deleteMany({
      where: { score: { lte: 2 }, in: { id: recordIdString(user.id) } },
    });
    expect(removed).toEqual({ count: 2 });
  });
});

function asArray<T>(value: T | T[] | null | undefined): T[] {
  if (value == null) return [];
  return Array.isArray(value) ? value : [value];
}
