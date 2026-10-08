import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client orderBy relation _count", () => {
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

  it.skipIf(skip)("orders findMany by posts._count", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const users = [];
    const posts = [];

    for (const [label, postCount] of [
      ["a", 1],
      ["b", 3],
      ["c", 2],
    ] as const) {
      const user = await client.user.create({
        email: `obc-posts-${label}-${suffix}@example.com`,
        age: 20,
        metadata: { source: "order-count-e2e" },
        tags: ["obc"],
      });
      users.push(user);
      const userId = recordIdString(user.id);
      for (let i = 0; i < postCount; i++) {
        const post = await client.post.create({
          title: `obc-posts-${label}-${i}-${suffix}`,
          author: userId,
        });
        posts.push(post);
      }
    }

    const ids = new Set(users.map((row) => recordIdString(row.id)));
    const ordered = await client.user.findMany({
      where: { email: { contains: suffix } },
      orderBy: { posts: { _count: "desc" } },
    });
    const ours = ordered.filter((row) => ids.has(recordIdString(row.id)));
    expect(ours.map((row) => row.email)).toEqual([
      `obc-posts-b-${suffix}@example.com`,
      `obc-posts-c-${suffix}@example.com`,
      `obc-posts-a-${suffix}@example.com`,
    ]);

    for (const post of posts) {
      await client.post.delete(recordIdString(post.id));
    }
    for (const user of users) {
      await client.user.delete(recordIdString(user.id));
    }
  });

  it.skipIf(skip)("orders findMany by liked._count and scalar tie-break", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const users = [];
    const posts = [];
    const likes = [];

    for (const [label, likeCount] of [
      ["a", 2],
      ["b", 0],
      ["c", 1],
    ] as const) {
      const user = await client.user.create({
        email: `obc-liked-${label}-${suffix}@example.com`,
        age: 21,
        metadata: { source: "order-count-e2e" },
        tags: ["obc"],
      });
      users.push(user);
      const userId = recordIdString(user.id);
      for (let i = 0; i < likeCount; i++) {
        const post = await client.post.create({
          title: `obc-liked-${label}-${i}-${suffix}`,
          author: userId,
        });
        posts.push(post);
        const like = await client.likes.create({
          in: userId,
          out: recordIdString(post.id),
          score: i + 1,
        });
        likes.push(like);
      }
    }

    // Extra user with same like count as "c" (1) for email tie-break.
    const tie = await client.user.create({
      email: `obc-liked-d-${suffix}@example.com`,
      age: 22,
      metadata: { source: "order-count-e2e" },
      tags: ["obc"],
    });
    users.push(tie);
    const tiePost = await client.post.create({
      title: `obc-liked-d-0-${suffix}`,
      author: recordIdString(tie.id),
    });
    posts.push(tiePost);
    likes.push(
      await client.likes.create({
        in: recordIdString(tie.id),
        out: recordIdString(tiePost.id),
        score: 1,
      }),
    );

    const ids = new Set(users.map((row) => recordIdString(row.id)));
    const byLiked = await client.user.findMany({
      where: { email: { contains: suffix } },
      orderBy: { liked: { _count: "asc" } },
    });
    const likedOurs = byLiked.filter((row) => ids.has(recordIdString(row.id)));
    expect(likedOurs[0]?.email).toBe(`obc-liked-b-${suffix}@example.com`);
    expect(likedOurs[likedOurs.length - 1]?.email).toBe(
      `obc-liked-a-${suffix}@example.com`,
    );

    const combined = await client.user.findMany({
      where: { email: { contains: suffix } },
      orderBy: [{ posts: { _count: "desc" } }, { email: "asc" }],
    });
    const combinedOurs = combined.filter((row) =>
      ids.has(recordIdString(row.id)),
    );
    // a has 2 posts; c and d have 1 each — email asc puts c before d among ties.
    expect(combinedOurs.map((row) => row.email)).toEqual([
      `obc-liked-a-${suffix}@example.com`,
      `obc-liked-c-${suffix}@example.com`,
      `obc-liked-d-${suffix}@example.com`,
      `obc-liked-b-${suffix}@example.com`,
    ]);

    for (const like of likes) {
      await client.likes.delete(recordIdString(like.id));
    }
    for (const post of posts) {
      await client.post.delete(recordIdString(post.id));
    }
    for (const user of users) {
      await client.user.delete(recordIdString(user.id));
    }
  });
});
