import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client nested relation orderBy", () => {
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

  it.skipIf(skip)("orders nested posts by title and projects selected fields", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `nord-posts-${suffix}@example.com`,
      age: 30,
      metadata: { source: "nested-order-e2e" },
      tags: ["nord"],
      posts: {
        create: [
          { title: `nord-c-${suffix}` },
          { title: `nord-a-${suffix}` },
          { title: `nord-b-${suffix}` },
        ],
      },
    });
    const userId = recordIdString(user.id);

    const row = await client.user.findUnique({
      where: { id: userId },
      select: {
        email: true,
        posts: {
          select: { id: true, title: true },
          orderBy: { title: "asc" },
        },
      },
    });
    expect(row?.email).toContain(suffix);
    const posts = asArray(row?.posts);
    expect(posts.map((p) => p.title)).toEqual([
      `nord-a-${suffix}`,
      `nord-b-${suffix}`,
      `nord-c-${suffix}`,
    ]);
    expect(posts.every((p) => !("author" in p))).toBe(true);

    const listed = await client.user.findMany({
      where: { email: { contains: suffix } },
      orderBy: { email: "asc" },
      select: {
        id: true,
        posts: {
          select: { title: true },
          orderBy: { title: "desc" },
        },
      },
    });
    expect(listed).toHaveLength(1);
    expect(asArray(listed[0]?.posts).map((p) => p.title)).toEqual([
      `nord-c-${suffix}`,
      `nord-b-${suffix}`,
      `nord-a-${suffix}`,
    ]);

    for (const post of posts) {
      await client.post.delete(recordIdString(post.id));
    }
    await client.user.delete(userId);
  });

  it.skipIf(skip)("orders nested liked edges by score", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `nord-liked-${suffix}@example.com`,
      age: 25,
      metadata: { source: "nested-order-e2e" },
      tags: ["nord"],
    });
    const userId = recordIdString(user.id);
    const posts = [];
    for (const [label, score] of [
      ["x", 2],
      ["y", 9],
      ["z", 5],
    ] as const) {
      const post = await client.post.create({
        title: `nord-liked-${label}-${suffix}`,
        author: userId,
      });
      posts.push(post);
      await client.likes.create({
        in: userId,
        out: recordIdString(post.id),
        score,
      });
    }

    const row = await client.user.findUnique({
      where: { id: userId },
      select: {
        liked: {
          select: {
            id: true,
            score: true,
            out: { select: { id: true, title: true } },
          },
          orderBy: { score: "desc" },
        },
      },
    });
    const liked = asArray(
      row?.liked as
        | { id: unknown; score: number; out: { title: string } }
        | Array<{ id: unknown; score: number; out: { title: string } }>
        | undefined,
    );
    expect(liked.map((l) => l.score)).toEqual([9, 5, 2]);
    expect(liked[0]?.out.title).toContain("nord-liked-y-");

    for (const like of liked) {
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
