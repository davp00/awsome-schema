import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client nested writes", () => {
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

  it.skipIf(skip)("create user with nested posts create", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `nw-posts-${suffix}@example.com`,
      age: 31,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
      posts: {
        create: [{ title: `nw-a-${suffix}` }, { title: `nw-b-${suffix}` }],
      },
    });
    const userId = recordIdString(user.id);

    const posts = await client.post.findMany({
      where: { author: { id: userId } },
      orderBy: { title: "asc" },
    });
    expect(posts.map((p) => p.title)).toEqual([
      `nw-a-${suffix}`,
      `nw-b-${suffix}`,
    ]);

    for (const post of posts) {
      await client.post.delete(recordIdString(post.id));
    }
    await client.user.delete(userId);
  });

  it.skipIf(skip)("update posts connect and reject required disconnect", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `nw-conn-${suffix}@example.com`,
      age: 22,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const userId = recordIdString(user.id);
    const other = await client.user.create({
      email: `nw-other-${suffix}@example.com`,
      age: 23,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const otherId = recordIdString(other.id);
    const post = await client.post.create({
      title: `nw-conn-${suffix}`,
      author: otherId,
    });
    const postId = recordIdString(post.id);

    await client.user.update(userId, {
      posts: { connect: [{ id: postId }] },
    });
    const moved = await client.post.findUnique({ where: { id: postId } });
    expect(recordIdString(moved!.author)).toBe(userId);

    await expect(
      client.user.update(userId, {
        posts: { disconnect: [{ id: postId }] },
      }),
    ).rejects.toThrow(/cannot disconnect required back-link/);

    await client.post.delete(postId);
    await client.user.delete(userId);
    await client.user.delete(otherId);
  });

  it.skipIf(skip)("create user with nested liked edge create", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `nw-liked-${suffix}@example.com`,
      age: 40,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
      liked: {
        create: [
          {
            score: 8,
            out: { create: { title: `nw-liked-post-${suffix}` } },
          },
        ],
      },
    });
    const userId = recordIdString(user.id);

    const posts = await client.post.findMany({
      where: { title: `nw-liked-post-${suffix}` },
    });
    expect(posts).toHaveLength(1);
    const postId = recordIdString(posts[0]!.id);
    expect(recordIdString(posts[0]!.author)).toBe(userId);

    const selected = await client.user.findUnique({
      where: { id: userId },
      select: {
        id: true,
        liked: {
          select: {
            id: true,
            score: true,
            out: { select: { id: true, title: true } },
          },
        },
      },
    });
    const liked = asArray(
      selected?.liked as
        | {
            id: unknown;
            score: number;
            out: { id: unknown; title: string };
          }
        | Array<{
            id: unknown;
            score: number;
            out: { id: unknown; title: string };
          }>
        | undefined,
    );
    expect(liked).toHaveLength(1);
    expect(liked[0]!.score).toBe(8);
    expect(recordIdString(liked[0]!.out.id)).toBe(postId);

    await client.likes.delete(recordIdString(liked[0]!.id));
    await client.post.delete(postId);
    await client.user.delete(userId);
  });

  it.skipIf(skip)("update liked connect and disconnect by out id", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `nw-edge-${suffix}@example.com`,
      age: 28,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const userId = recordIdString(user.id);
    const post = await client.post.create({
      title: `nw-edge-post-${suffix}`,
      author: userId,
    });
    const postId = recordIdString(post.id);

    await client.user.update(userId, {
      liked: { connect: [{ out: postId, score: 3 }] },
    });
    let selected = await client.user.findUnique({
      where: { id: userId },
      select: {
        liked: { select: { id: true, out: { select: { id: true } } } },
      },
    });
    let liked = asArray(
      selected?.liked as { id: unknown; out: { id: unknown } }[],
    );
    expect(liked).toHaveLength(1);
    expect(recordIdString(liked[0]!.out.id)).toBe(postId);

    await client.user.update(userId, {
      liked: { disconnect: [{ out: postId }] },
    });
    selected = await client.user.findUnique({
      where: { id: userId },
      select: {
        liked: { select: { id: true } },
      },
    });
    liked = asArray(selected?.liked as { id: unknown }[]);
    expect(liked).toHaveLength(0);

    await client.post.delete(postId);
    await client.user.delete(userId);
  });

  it.skipIf(skip)("nested writes in $transaction roll back on throw", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `nw-tx-${suffix}@example.com`;

    await expect(
      client.$transaction(async (tx) => {
        await tx.user.create({
          email,
          age: 1,
          metadata: { source: "nested-write-e2e" },
          tags: ["nw"],
          posts: { create: [{ title: `nw-tx-${suffix}` }] },
        });
        throw new Error("force rollback");
      }),
    ).rejects.toThrow(/force rollback/);

    expect(await client.user.findUnique({ where: { email } })).toBeUndefined();
    const posts = await client.post.findMany({
      where: { title: `nw-tx-${suffix}` },
    });
    expect(posts).toHaveLength(0);
  });

  it.skipIf(skip)("create post with nested author create", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `nw-author-${suffix}@example.com`;
    const post = await client.post.create({
      title: `nw-author-post-${suffix}`,
      author: {
        create: {
          email,
          age: 5,
          metadata: { source: "nested-write-e2e" },
          tags: ["nw"],
        },
      },
    });
    const postId = recordIdString(post.id);
    const user = await client.user.findUnique({ where: { email } });
    expect(user).toBeDefined();
    expect(recordIdString(post.author)).toBe(recordIdString(user!.id));

    await client.post.delete(postId);
    await client.user.delete(recordIdString(user!.id));
  });

  it.skipIf(skip)("create user with posts and a liked connect together", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const author = await client.user.create({
      email: `nw-mix-author-${suffix}@example.com`,
      age: 9,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const authorId = recordIdString(author.id);
    const existing = await client.post.create({
      title: `nw-mix-existing-${suffix}`,
      author: authorId,
    });
    const existingId = recordIdString(existing.id);

    const user = await client.user.create({
      email: `nw-mix-${suffix}@example.com`,
      age: 11,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
      posts: {
        create: [{ title: `nw-mix-a-${suffix}` }, { title: `nw-mix-b-${suffix}` }],
      },
      liked: { connect: [{ out: existingId, score: 4 }] },
    });
    const userId = recordIdString(user.id);

    const posts = await client.post.findMany({
      where: { author: { id: userId } },
      orderBy: { title: "asc" },
    });
    expect(posts.map((p) => p.title)).toEqual([
      `nw-mix-a-${suffix}`,
      `nw-mix-b-${suffix}`,
    ]);

    const selected = await client.user.findUnique({
      where: { id: userId },
      select: { liked: { select: { id: true, score: true } } },
    });
    const liked = asArray(selected?.liked as { id: unknown; score: number }[]);
    expect(liked).toHaveLength(1);
    expect(liked[0]!.score).toBe(4);

    await client.likes.delete(recordIdString(liked[0]!.id));
    for (const post of posts) await client.post.delete(recordIdString(post.id));
    await client.post.delete(existingId);
    await client.user.delete(userId);
    await client.user.delete(authorId);
  });

  it.skipIf(skip)("disconnect liked by edge id", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `nw-edge-id-${suffix}@example.com`,
      age: 12,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const userId = recordIdString(user.id);
    const post = await client.post.create({
      title: `nw-edge-id-post-${suffix}`,
      author: userId,
    });
    const postId = recordIdString(post.id);
    await client.user.update(userId, {
      liked: { connect: [{ out: postId, score: 1 }] },
    });
    const selected = await client.user.findUnique({
      where: { id: userId },
      select: { liked: { select: { id: true } } },
    });
    const liked = asArray(selected?.liked as { id: unknown }[]);
    expect(liked).toHaveLength(1);

    await client.user.update(userId, {
      liked: { disconnect: [{ id: recordIdString(liked[0]!.id) }] },
    });
    const after = await client.user.findUnique({
      where: { id: userId },
      select: { liked: { select: { id: true } } },
    });
    expect(asArray(after?.liked as { id: unknown }[])).toHaveLength(0);

    await client.post.delete(postId);
    await client.user.delete(userId);
  });

  it.skipIf(skip)("rejects nested-on-nested second hop", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    await expect(
      client.post.create({
        title: `nw-deep-${suffix}`,
        author: {
          create: {
            email: `nw-deep-${suffix}@example.com`,
            age: 1,
            metadata: { source: "nested-write-e2e" },
            tags: ["nw"],
            posts: { create: [{ title: `nw-deep-child-${suffix}` }] },
          } as never,
        },
      }),
    ).rejects.toThrow(/one hop only/);
  });
});

function asArray<T>(value: T | T[] | null | undefined): T[] {
  if (value == null) return [];
  return Array.isArray(value) ? value : [value];
}
