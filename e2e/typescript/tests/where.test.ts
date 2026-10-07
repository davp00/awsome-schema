import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client hybrid where", () => {
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

  it.skipIf(skip)(
    "filters with bare equality, nested relations, and Prisma-lite operators",
    async () => {
      const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
      const email = `where-${suffix}@example.com`;
      const title = `Where post ${suffix}`;

      const user = await client.user.create({
        email,
        age: 28,
        metadata: { source: "where-e2e", user_id: 3 },
        tags: ["filter"],
      });
      const userId = recordIdString(user.id);

      const post = await client.post.create({
        title,
        author: userId,
      });
      const postId = recordIdString(post.id);

      const like = await client.likes.create({
        in: userId,
        out: postId,
        score: 8,
      });
      const likeId = recordIdString(like.id);

      // Bare scalar equals
      const byEmail = await client.user.findMany({
        where: { email },
      });
      expect(byEmail.some((row) => recordIdString(row.id) === userId)).toBe(true);

      // Nested stored link (bare object ≡ is)
      const byAuthor = await client.post.findMany({
        where: { author: { email } },
      });
      expect(byAuthor.some((row) => recordIdString(row.id) === postId)).toBe(true);

      // List sugar ≡ some on computed posts
      const withPost = await client.user.findMany({
        where: { posts: { title } },
      });
      expect(withPost.some((row) => recordIdString(row.id) === userId)).toBe(true);

      // Explicit some + operator on edge
      const withLiked = await client.user.findMany({
        where: {
          liked: { some: { score: { gte: 5 } } },
        },
      });
      expect(withLiked.some((row) => recordIdString(row.id) === userId)).toBe(true);

      // Scalar operator
      const byAge = await client.user.findMany({
        where: {
          email: { contains: suffix },
          age: { gte: 18 },
        },
      });
      expect(byAge.some((row) => recordIdString(row.id) === userId)).toBe(true);

      await client.likes.delete(likeId);
      await client.post.delete(postId);
      await client.user.delete(userId);
    },
  );
});
