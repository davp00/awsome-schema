import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client CRUD", () => {
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
    "create → update → delete User, Post, and Likes in sequence",
    async () => {
      const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
      const email = `alice-${suffix}@example.com`;
      const emailUpdated = `alice-updated-${suffix}@example.com`;

      // 1. Create User
      const user = await client.user.create({
        email,
        age: 30,
        metadata: { source: "e2e", user_id: 1 },
        tags: ["alpha"],
      });
      expect(user.email).toBe(email);
      expect(user.age).toBe(30);
      const userId = recordIdString(user.id);

      // 2. Create Post linked to User
      const post = await client.post.create({
        title: "Hello",
        author: userId,
      });
      expect(post.title).toBe("Hello");
      expect(recordIdString(post.author)).toBe(userId);
      const postId = recordIdString(post.id);

      // 3. Create Likes edge
      const like = await client.likes.create({
        in: userId,
        out: postId,
        score: 5,
      });
      expect(like.score).toBe(5);
      const likeId = recordIdString(like.id);

      // 4. Update User
      const updatedUser = await client.user.update(userId, {
        email: emailUpdated,
        age: 31,
      });
      expect(updatedUser?.email).toBe(emailUpdated);
      expect(updatedUser?.age).toBe(31);
      const readUser = await client.user.findUnique({ where: { id: userId } });
      expect(readUser?.email).toBe(emailUpdated);

      // 5. Update Post
      const updatedPost = await client.post.update(postId, { title: "Hello again" });
      expect(updatedPost?.title).toBe("Hello again");
      const readPost = await client.post.findUnique({ where: { id: postId } });
      expect(readPost?.title).toBe("Hello again");

      // 6. Update Likes
      const updatedLike = await client.likes.update(likeId, { score: 9 });
      expect(updatedLike?.score).toBe(9);
      const readLike = await client.likes.findUnique({ where: { id: likeId } });
      expect(readLike?.score).toBe(9);

      // 7. Delete Likes
      await client.likes.delete(likeId);
      expect(await client.likes.findUnique({ where: { id: likeId } })).toBeUndefined();

      // 8. Delete Post
      await client.post.delete(postId);
      expect(await client.post.findUnique({ where: { id: postId } })).toBeUndefined();

      // 9. Delete User
      await client.user.delete(userId);
      expect(await client.user.findUnique({ where: { id: userId } })).toBeUndefined();
    },
  );
});
