import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client nested select", () => {
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
    "projects nested stored link, computed link, and edge with field picks",
    async () => {
      const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
      const email = `nested-${suffix}@example.com`;

      // Seed: create graph used by later select steps
      const user = await client.user.create({
        email,
        age: 42,
        metadata: { source: "select-e2e", user_id: 7 },
        tags: ["nested"],
      });
      const userId = recordIdString(user.id);

      const post = await client.post.create({
        title: `Nested post ${suffix}`,
        author: userId,
      });
      const postId = recordIdString(post.id);

      const like = await client.likes.create({
        in: userId,
        out: postId,
        score: 11,
      });
      const likeId = recordIdString(like.id);

      // Stored @link: Post.author.{ … }
      const postProjected = await client.post.findUnique({
        where: { id: postId },
        select: {
          id: true,
          title: true,
          author: {
            select: {
              id: true,
              email: true,
              age: true,
            },
          },
        },
      });
      expect(postProjected).toBeDefined();
      expect(recordIdString(postProjected!.id)).toBe(postId);
      expect(postProjected!.title).toBe(`Nested post ${suffix}`);
      expect(postProjected!.author).toBeDefined();
      expect(recordIdString(postProjected!.author.id)).toBe(userId);
      expect(postProjected!.author.email).toBe(email);
      expect(postProjected!.author.age).toBe(42);
      // Scalar-only nested pick: metadata/tags must not appear
      expect(
        postProjected!.author &&
          typeof postProjected!.author === "object" &&
          "metadata" in postProjected!.author,
      ).toBe(false);

      // Computed @link + @relation edge, with deeper nest on edge.out and posts.author
      const userProjected = await client.user.findUnique({
        where: { id: userId },
        select: {
          id: true,
          email: true,
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
      expect(userProjected).toBeDefined();
      expect(userProjected!.email).toBe(email);

      const posts = asArray(userProjected!.posts);
      expect(posts.length).toBeGreaterThanOrEqual(1);
      const nestedPost = posts.find((row) => recordIdString(row.id) === postId);
      expect(nestedPost).toBeDefined();
      expect(nestedPost!.title).toBe(`Nested post ${suffix}`);
      expect(nestedPost!.author?.email).toBe(email);

      const likes = asArray(userProjected!.liked);
      expect(likes.length).toBeGreaterThanOrEqual(1);
      const nestedLike = likes.find((row) => recordIdString(row.id) === likeId);
      expect(nestedLike).toBeDefined();
      expect(nestedLike!.score).toBe(11);
      expect(recordIdString(nestedLike!.out.id)).toBe(postId);
      expect(nestedLike!.out.title).toBe(`Nested post ${suffix}`);

      // Relation `true` expands edge scalars (no nested out object required)
      const userEdgeTrue = await client.user.findUnique({
        where: { id: userId },
        select: {
          id: true,
          liked: true,
        },
      });
      const likesTrue = asArray(userEdgeTrue!.liked);
      expect(likesTrue.some((row) => recordIdString(row.id) === likeId)).toBe(true);

      // Cleanup (reverse of create)
      await client.likes.delete(likeId);
      await client.post.delete(postId);
      await client.user.delete(userId);
    },
  );
});

function asArray<T>(value: T | T[] | null | undefined): T[] {
  if (value == null) return [];
  return Array.isArray(value) ? value : [value];
}
