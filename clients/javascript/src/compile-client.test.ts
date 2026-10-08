import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createRequire } from "node:module";
import { afterEach, describe, expect, it } from "vitest";

import { compileGeneratedClient, typescriptToCommonJs } from "../bin/compile-client.cjs";

const require = createRequire(import.meta.url);
const roots: string[] = [];

afterEach(() => {
  for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe("generated client compile", () => {
  it("turns generated TypeScript into named CommonJS exports", () => {
    const source = `
import { bindSchemaMeta, type RecordId } from "awesome-schema/runtime";
export { recordId, type RecordId } from "awesome-schema/runtime";
export type User = { id: string };
export const Tables = { user: "user" } as const;
export async function createUser(db: unknown, data: User): Promise<User> {
  return bindSchemaMeta(db, data as Record<string, unknown>);
}
export function createClient(db: unknown) {
  return { user: createUser };
}
`;
    const cjs = typescriptToCommonJs(source);
    expect(cjs).toContain('require("awesome-schema/runtime")');
    expect(cjs).toContain("exports.recordId = __awesomeSchemaRuntime.recordId;");
    expect(cjs).toContain("const Tables = exports.Tables =");
    expect(cjs).toContain("exports.createUser = async function createUser");
    expect(cjs).toContain("exports.createClient = function createClient");
    expect(cjs).not.toContain("export ");
    expect(cjs).not.toContain("import ");
  });

  it("writes index.cjs next to the generated client", () => {
    const root = mkdtempSync(join(tmpdir(), "awesome-schema-client-"));
    roots.push(root);
    const clientDir = join(root, "node_modules", ".awesome-schema", "client");
    mkdirSync(clientDir, { recursive: true });
    writeFileSync(
      join(clientDir, "index.ts"),
      'export function createClient() { return "ok"; }\n',
    );
    const written = compileGeneratedClient(root);
    expect(written).toBe(join(clientDir, "index.cjs"));
    expect(readFileSync(written, "utf8")).toContain("exports.createClient = function createClient");
    expect(require(written).createClient()).toBe("ok");
  });
});
