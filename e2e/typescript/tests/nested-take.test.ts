import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client nested relation take/skip", () => {
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

  it.skipIf(skip)("pages nested posts with skip and take", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `ntake-posts-${suffix}@example.com`,
      age: 30,
      metadata: { source: "nested-take-e2e" },
      tags: ["ntake"],
      posts: {
        create: [
          { title: `ntake-c-${suffix}` },
          { title: `ntake-a-${suffix}` },
          { title: `ntake-b-${suffix}` },
        ],
      },
    });
    const userId = recordIdString(user.id);

    const row = await client.user.findUnique({
      where: { id: userId },
      select: {
        posts: {
          select: { id: true, title: true },
          orderBy: { title: "asc" },
          skip: 1,
          take: 2,
        },
      },
    });
    const posts = asArray(row?.posts);
    expect(posts.map((post) => post.title)).toEqual([`ntake-b-${suffix}`, `ntake-c-${suffix}`]);

    for (const post of posts) {
      await client.post.delete(recordIdString(post.id));
    }
    const skipped = await client.post.findMany({
      where: { title: { contains: suffix } },
    });
    for (const post of skipped) {
      await client.post.delete(recordIdString(post.id));
    }
    await client.user.delete(userId);
  });

  it.skipIf(skip)("takes the highest nested liked edge", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `ntake-liked-${suffix}@example.com`,
      age: 25,
      metadata: { source: "nested-take-e2e" },
      tags: ["ntake"],
    });
    const userId = recordIdString(user.id);
    const posts = [];
    const likes = [];
    for (const [label, score] of [
      ["x", 2],
      ["y", 9],
      ["z", 5],
    ] as const) {
      const post = await client.post.create({
        title: `ntake-liked-${label}-${suffix}`,
        author: userId,
      });
      posts.push(post);
      likes.push(
        await client.likes.create({
          in: userId,
          out: recordIdString(post.id),
          score,
        }),
      );
    }

    const row = await client.user.findUnique({
      where: { id: userId },
      select: {
        liked: {
          select: { id: true, score: true },
          orderBy: { score: "desc" },
          take: 1,
        },
      },
    });
    const liked = asArray(row?.liked as { id: unknown; score: number } | Array<{ id: unknown; score: number }> | undefined);
    expect(liked.map((like) => like.score)).toEqual([9]);

    for (const like of likes) {
      await client.likes.delete(recordIdString(like.id));
    }
    for (const post of posts) {
      await client.post.delete(recordIdString(post.id));
    }
    await client.user.delete(userId);
  });
});

function asArray<T>(value: T | T[] | null | undefined): T[] {
  if (value == null) return [];
  return Array.isArray(value) ? value : [value];
}
