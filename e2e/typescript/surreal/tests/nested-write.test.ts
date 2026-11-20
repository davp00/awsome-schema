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

  it.skipIf(skip)("nested set and delete in $transaction roll back on throw", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `nw-tx-set-${suffix}@example.com`,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const userId = recordIdString(user.id);
    const keep = await client.post.create({ title: `nw-tx-keep-${suffix}`, author: userId });
    const drop = await client.post.create({ title: `nw-tx-drop-${suffix}`, author: userId });
    const keepId = recordIdString(keep.id);
    const dropId = recordIdString(drop.id);
    await client.user.update(userId, {
      liked: { connect: [{ out: keepId, score: 1 }, { out: dropId, score: 2 }] },
    });

    await expect(
      client.$transaction(async (tx) => {
        await tx.user.update(userId, {
          liked: { set: [{ out: keepId, score: 9 }] },
        });
        await tx.user.update(userId, {
          liked: { delete: [{ out: keepId }] },
        });
        throw new Error("force nested rollback");
      }),
    ).rejects.toThrow(/force nested rollback/);

    const selected = await client.user.findUnique({
      where: { id: userId },
      select: { liked: { select: { score: true, out: { select: { id: true } } } } },
    });
    const liked = asArray(
      selected?.liked as { score: number; out: { id: unknown } }[],
    );
    expect(liked).toHaveLength(2);
    expect(liked.map((row) => row.score).sort()).toEqual([1, 2]);

    await client.likes.deleteMany({ where: { in: { id: userId } } });
    await client.post.delete(keepId);
    await client.post.delete(dropId);
    await client.user.delete(userId);
  });

  it.skipIf(skip)("optional stored delete clears fk then deletes target", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const author = await client.user.create({
      email: `nw-opt-author-${suffix}@example.com`,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const editor = await client.user.create({
      email: `nw-opt-editor-${suffix}@example.com`,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const authorId = recordIdString(author.id);
    const editorId = recordIdString(editor.id);
    const post = await client.post.create({
      title: `nw-opt-${suffix}`,
      author: authorId,
      editor: editorId,
    });
    const postId = recordIdString(post.id);

    await client.post.update(postId, { editor: { delete: true } });

    const after = await client.post.findUnique({
      where: { id: postId },
      select: { id: true, editor: true },
    });
    expect(after?.editor ?? null).toBeNull();
    expect(await client.user.findUnique({ where: { id: editorId } })).toBeUndefined();

    await client.post.delete(postId);
    await client.user.delete(authorId);
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

  it.skipIf(skip)("posts connectOrCreate, update, upsert, and delete", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const owner = await client.user.create({
      email: `nw-op-owner-${suffix}@example.com`,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const ownerId = recordIdString(owner.id);
    const other = await client.user.create({
      email: `nw-op-other-${suffix}@example.com`,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const otherId = recordIdString(other.id);
    const post = await client.post.create({
      title: `nw-op-keep-${suffix}`,
      author: ownerId,
    });
    const postId = recordIdString(post.id);

    await client.user.update(otherId, {
      posts: {
        connectOrCreate: [{ where: { id: postId }, create: { title: `nw-op-unused-${suffix}` } }],
      },
    });
    const moved = await client.post.findUnique({ where: { id: postId } });
    expect(recordIdString(moved!.author)).toBe(otherId);

    await client.user.update(otherId, {
      posts: { update: [{ where: { id: postId }, data: { title: `nw-op-edited-${suffix}` } }] },
    });
    const edited = await client.post.findUnique({ where: { id: postId } });
    expect(edited!.title).toBe(`nw-op-edited-${suffix}`);

    await client.user.update(otherId, {
      posts: {
        upsert: [{
          where: { id: `missing-${suffix}` },
          update: { title: "unused" },
          create: { title: `nw-op-upserted-${suffix}` },
        }],
      },
    });
    const posts = await client.post.findMany({
      where: { author: { id: otherId } },
      orderBy: { title: "asc" },
    });
    expect(posts.map((row) => row.title)).toEqual([
      `nw-op-edited-${suffix}`,
      `nw-op-upserted-${suffix}`,
    ]);

    await client.user.update(otherId, {
      posts: { delete: [{ id: postId }] },
    });
    expect(await client.post.findUnique({ where: { id: postId } })).toBeUndefined();

    for (const row of posts) {
      if (recordIdString(row.id) !== postId) await client.post.delete(recordIdString(row.id));
    }
    await client.user.delete(otherId);
    await client.user.delete(ownerId);
  });

  it.skipIf(skip)("liked set replaces edges and upsert updates score", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `nw-set-${suffix}@example.com`,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const userId = recordIdString(user.id);
    const keep = await client.post.create({ title: `nw-set-keep-${suffix}`, author: userId });
    const drop = await client.post.create({ title: `nw-set-drop-${suffix}`, author: userId });
    const keepId = recordIdString(keep.id);
    const dropId = recordIdString(drop.id);
    await client.user.update(userId, {
      liked: {
        connect: [
          { out: keepId, score: 1 },
          { out: dropId, score: 2 },
        ],
      },
    });

    await client.user.update(userId, {
      liked: { set: [{ out: keepId, score: 5 }] },
    });
    let selected = await client.user.findUnique({
      where: { id: userId },
      select: { liked: { select: { id: true, score: true } } },
    });
    let liked = asArray(selected?.liked as { id: unknown; score: number }[]);
    expect(liked).toHaveLength(1);
    expect(liked[0]!.score).toBe(5);
    expect(await client.likes.count({ where: { out: { id: dropId } } })).toBe(0);

    await client.user.update(userId, {
      liked: {
        upsert: [{
          where: { out: keepId },
          update: { score: 9 },
          create: { out: keepId, score: 9 },
        }],
      },
    });
    selected = await client.user.findUnique({
      where: { id: userId },
      select: { liked: { select: { id: true, score: true } } },
    });
    liked = asArray(selected?.liked as { id: unknown; score: number }[]);
    expect(liked).toHaveLength(1);
    expect(liked[0]!.score).toBe(9);

    await client.likes.delete(recordIdString(liked[0]!.id));
    await client.post.delete(keepId);
    await client.post.delete(dropId);
    await client.user.delete(userId);
  });

  it.skipIf(skip)("author connectOrCreate links an existing user or creates one", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const existing = await client.user.create({
      email: `nw-coc-${suffix}@example.com`,
      metadata: { source: "nested-write-e2e" },
      tags: ["nw"],
    });
    const existingId = recordIdString(existing.id);
    const linked = await client.post.create({
      title: `nw-coc-link-${suffix}`,
      author: {
        connectOrCreate: {
          where: { email: `nw-coc-${suffix}@example.com` },
          create: {
            email: `nw-coc-${suffix}@example.com`,
            metadata: { source: "nested-write-e2e" },
            tags: ["nw"],
          },
        },
      },
    });
    expect(recordIdString(linked.author)).toBe(existingId);

    const created = await client.post.create({
      title: `nw-coc-new-${suffix}`,
      author: {
        connectOrCreate: {
          where: { email: `nw-coc-new-${suffix}@example.com` },
          create: {
            email: `nw-coc-new-${suffix}@example.com`,
            metadata: { source: "nested-write-e2e" },
            tags: ["nw"],
          },
        },
      },
    });
    const createdAuthor = recordIdString(created.author);
    expect(createdAuthor).not.toBe(existingId);
    const author = await client.user.findUnique({ where: { id: createdAuthor } });
    expect(author!.email).toBe(`nw-coc-new-${suffix}@example.com`);

    await client.post.delete(recordIdString(linked.id));
    await client.post.delete(recordIdString(created.id));
    await client.user.delete(existingId);
    await client.user.delete(createdAuthor);
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
