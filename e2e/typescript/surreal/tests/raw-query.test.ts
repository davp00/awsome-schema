import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

describe("generated TypeScript client raw query", () => {
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

  function userContent(email: string, age: number) {
    return {
      email,
      age,
      metadata: { source: "raw-e2e" },
      tags: ["raw"],
    };
  }

  async function deleteByEmail(email: string) {
    const found = await client.user.findMany({ where: { email } });
    for (const row of found) {
      await client.user.delete(recordIdString(row.id));
    }
  }

  it.skipIf(skip)("selects a bound row with $queryRaw", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `raw-select-${suffix}@example.com`;
    const user = await client.user.create(userContent(email, 21));
    const userId = recordIdString(user.id);

    const rows = await client.$queryRaw<{ email: string; age: number }>(
      "SELECT email, age FROM user WHERE email = $email",
      { email },
    );
    expect(rows).toEqual([{ email, age: 21 }]);

    await client.user.delete(userId);
  });

  it.skipIf(skip)("applies $executeRaw and reads the mutation back", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `raw-exec-${suffix}@example.com`;
    const user = await client.user.create(userContent(email, 21));
    const userId = recordIdString(user.id);

    await client.$executeRaw("UPDATE user SET age = $age WHERE email = $email", {
      age: 44,
      email,
    });

    const rows = await client.$queryRaw<{ age: number }>(
      "SELECT age FROM user WHERE email = $email",
      { email },
    );
    expect(rows).toEqual([{ age: 44 }]);

    await client.user.delete(userId);
  });

  it.skipIf(skip)("commits a raw write inside $transaction", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `raw-tx-ok-${suffix}@example.com`;

    await client.$transaction(async (tx) => {
      await tx.$executeRaw("CREATE user CONTENT $data", {
        data: userContent(email, 40),
      });
    });

    const rows = await client.$queryRaw<{ email: string }>(
      "SELECT email FROM user WHERE email = $email",
      { email },
    );
    expect(rows).toEqual([{ email }]);

    await deleteByEmail(email);
  });

  it.skipIf(skip)("rolls back a raw write when the callback throws", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `raw-tx-fail-${suffix}@example.com`;

    await expect(
      client.$transaction(async (tx) => {
        await tx.$executeRaw("CREATE user CONTENT $data", {
          data: userContent(email, 1),
        });
        throw new Error("force rollback");
      }),
    ).rejects.toThrow(/force rollback/);

    const rows = await client.$queryRaw<{ email: string }>(
      "SELECT email FROM user WHERE email = $email",
      { email },
    );
    expect(rows).toHaveLength(0);
  });
});
