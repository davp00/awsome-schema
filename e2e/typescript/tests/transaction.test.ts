import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client $transaction", () => {
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

  it.skipIf(skip)("commits user and post created inside $transaction", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `tx-ok-${suffix}@example.com`;

    const { userId, postId } = await client.$transaction(async (tx) => {
      const user = await tx.user.create({
        email,
        age: 40,
        metadata: { source: "tx-e2e" },
        tags: ["tx"],
      });
      const post = await tx.post.create({
        title: `tx-ok-${suffix}`,
        author: recordIdString(user.id),
      });
      return {
        userId: recordIdString(user.id),
        postId: recordIdString(post.id),
      };
    });

    const user = await client.user.findUnique({ where: { id: userId } });
    const post = await client.post.findUnique({ where: { id: postId } });
    expect(user?.email).toBe(email);
    expect(post?.title).toBe(`tx-ok-${suffix}`);

    await client.post.delete(postId);
    await client.user.delete(userId);
  });

  it.skipIf(skip)("rolls back when the callback throws", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `tx-fail-${suffix}@example.com`;

    await expect(
      client.$transaction(async (tx) => {
        await tx.user.create({
          email,
          age: 1,
          metadata: { source: "tx-e2e" },
          tags: ["tx-fail"],
        });
        throw new Error("force rollback");
      }),
    ).rejects.toThrow(/force rollback/);

    const found = await client.user.findMany({
      where: { email },
    });
    expect(found).toHaveLength(0);
  });

  it.skipIf(skip)("transaction client omits $transaction (no nesting)", async () => {
    await client.$transaction(async (tx) => {
      expect("$transaction" in tx).toBe(false);
    });
  });
});
