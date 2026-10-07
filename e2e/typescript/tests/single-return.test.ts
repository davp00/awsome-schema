import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client single update/delete/upsert select+return", () => {
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

  it.skipIf(skip)("update scalar select and return AFTER", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `single-upd-${suffix}@example.com`,
      age: 40,
      metadata: { source: "single-e2e" },
      tags: ["single"],
    });
    const id = recordIdString(user.id);

    const projected = await client.user.update(
      id,
      { age: 41 },
      { select: { id: true, email: true, age: true } },
    );
    expect(projected?.age).toBe(41);
    expect(projected?.email).toContain(suffix);
    expect(projected && "tags" in projected).toBe(false);

    const after = await client.user.update(id, { age: 42 }, { return: "AFTER" });
    expect(after?.age).toBe(42);
    expect(after?.tags).toEqual(["single"]);

    await client.user.delete(id);
  });

  it.skipIf(skip)("update with complex nested relation select", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `single-rel-${suffix}@example.com`;
    const user = await client.user.create({
      email,
      age: 28,
      metadata: { source: "single-e2e" },
      tags: ["rel"],
    });
    const userId = recordIdString(user.id);
    const post = await client.post.create({
      title: `single-rel-${suffix}`,
      author: userId,
    });
    const postId = recordIdString(post.id);
    const like = await client.likes.create({
      in: userId,
      out: postId,
      score: 7,
    });
    const likeId = recordIdString(like.id);

    const updated = await client.user.update(
      userId,
      { age: 29, tags: ["rel", "nested"] },
      {
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
      },
    );

    expect(updated).toBeDefined();
    expect(recordIdString(updated!.id)).toBe(userId);
    expect(updated!.email).toBe(email);
    expect(updated!.age).toBe(29);
    expect("tags" in updated!).toBe(false);

    const posts = asArray(updated!.posts);
    const nestedPost = posts.find((p) => recordIdString(p.id) === postId);
    expect(nestedPost).toBeDefined();
    expect(nestedPost!.title).toBe(`single-rel-${suffix}`);
    expect(nestedPost!.author?.email).toBe(email);

    const likes = asArray(updated!.liked);
    const nestedLike = likes.find((l) => recordIdString(l.id) === likeId);
    expect(nestedLike).toBeDefined();
    expect(nestedLike!.score).toBe(7);
    expect(recordIdString(nestedLike!.out.id)).toBe(postId);
    expect(nestedLike!.out.title).toBe(`single-rel-${suffix}`);

    await client.likes.delete(likeId);
    await client.post.delete(postId);
    await client.user.delete(userId);
  });

  it.skipIf(skip)("delete with select and rejects AFTER", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const user = await client.user.create({
      email: `single-del-${suffix}@example.com`,
      age: 1,
      metadata: { source: "single-e2e" },
      tags: ["del"],
    });
    const id = recordIdString(user.id);

    await expect(
      client.user.delete(id, { return: "AFTER" } as never),
    ).rejects.toThrow(/AFTER is not supported/);

    const deleted = await client.user.delete(id, {
      select: { id: true, email: true },
    });
    expect(recordIdString(deleted.id)).toBe(id);
    expect(deleted.email).toContain(suffix);

    expect(await client.user.findUnique({ where: { id } })).toBeUndefined();
  });

  it.skipIf(skip)("upsert complex select and return NONE", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `single-upsert-${suffix}@example.com`;

    const created = await client.user.upsert({
      where: { email },
      create: {
        email,
        age: 10,
        metadata: { source: "single-e2e" },
        tags: ["upsert"],
      },
      update: { age: 99 },
      select: {
        id: true,
        email: true,
        posts: {
          select: {
            id: true,
            title: true,
          },
        },
      },
    });
    expect(created.email).toBe(email);
    expect("age" in created).toBe(false);
    expect(asArray(created.posts)).toEqual([]);

    const none = await client.user.upsert({
      where: { email },
      create: {
        email,
        age: 0,
        metadata: { source: "single-e2e" },
        tags: ["upsert"],
      },
      update: { age: 33 },
      return: "NONE",
    });
    expect(none).toBeUndefined();

    const read = await client.user.findUnique({ where: { email } });
    expect(read?.age).toBe(33);

    await expect(
      client.user.update(recordIdString(read!.id), { age: 34 }, {
        select: { id: true },
        return: "AFTER",
      } as never),
    ).rejects.toThrow(/mutually exclusive/);

    await client.user.delete(recordIdString(read!.id));
  });
});

function asArray<T>(value: T | T[] | null | undefined): T[] {
  if (value == null) return [];
  return Array.isArray(value) ? value : [value];
}
