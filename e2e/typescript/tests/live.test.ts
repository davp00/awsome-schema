import { afterAll, beforeAll, describe, expect, it } from "vitest";

import {
  openGeneratedClient,
  readE2eEnv,
  recordIdString,
} from "../src/setupClient.js";

type LiveAction = "CREATE" | "UPDATE" | "DELETE";

type LiveHandle<T> = {
  subscribe(listener: (action: LiveAction, result: T) => void): void;
  kill(): Promise<void>;
};

describe("generated TypeScript client LIVE SELECT", () => {
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
      metadata: { source: "live-e2e" },
      tags: ["live"],
    };
  }

  function nextMatching<T extends { email?: string }>(
    handle: LiveHandle<T>,
    email: string,
    timeoutMs = 8000,
  ): Promise<{ action: LiveAction; result: T }> {
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => reject(new Error(`live event timed out for ${email}`)), timeoutMs);
      handle.subscribe((action, result) => {
        if (result.email !== email) return;
        clearTimeout(timer);
        resolve({ action, result });
      });
    });
  }

  async function deleteByEmail(email: string) {
    const found = await client.user.findMany({ where: { email } });
    for (const row of found) {
      await client.user.delete(recordIdString(row.id));
    }
  }

  it.skipIf(skip)("user.live receives CREATE for a new row", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `live-create-${suffix}@example.com`;
    const live = await client.user.live();
    const pending = nextMatching(live, email);

    try {
      await client.user.create(userContent(email, 21));
      const event = await pending;
      expect(event.action).toBe("CREATE");
      expect(event.result.email).toBe(email);
    } finally {
      await live.kill();
      await deleteByEmail(email);
    }
  });

  it.skipIf(skip)("kill stops later live events", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `live-kill-${suffix}@example.com`;
    const live = await client.user.live();
    let seen = false;
    live.subscribe((_action, result) => {
      if (result.email === email) seen = true;
    });

    try {
      await live.kill();
      await client.user.create(userContent(email, 9));
      await new Promise((resolve) => setTimeout(resolve, 400));
      expect(seen).toBe(false);
    } finally {
      await deleteByEmail(email);
    }
  });

  it.skipIf(skip)("$live with a bound WHERE receives only the matching row", async () => {
    const suffix = `${Date.now()}-${Math.random().toString(16).slice(2, 8)}`;
    const email = `live-match-${suffix}@example.com`;
    const other = `live-other-${suffix}@example.com`;
    const live = await client.$live<{ email: string }>(
      "LIVE SELECT * FROM user WHERE email = $email",
      { email },
    );
    const pending = nextMatching(live, email);

    try {
      await client.user.create(userContent(other, 1));
      await client.user.create(userContent(email, 2));
      const event = await pending;
      expect(event.action).toBe("CREATE");
      expect(event.result.email).toBe(email);
    } finally {
      await live.kill();
      await deleteByEmail(email);
      await deleteByEmail(other);
    }
  });
});
