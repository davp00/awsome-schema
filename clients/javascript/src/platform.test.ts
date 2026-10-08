import { describe, expect, it } from "vitest";

import { assetName, releaseUrl, rustTarget } from "../bin/lib.js";

describe("platform binary", () => {
  it("maps the current machine to one rust target", () => {
    expect(rustTarget("darwin", "arm64")).toBe("aarch64-apple-darwin");
    expect(rustTarget("darwin", "x64")).toBe("x86_64-apple-darwin");
    expect(rustTarget("linux", "x64")).toBe("x86_64-unknown-linux-gnu");
    expect(rustTarget("win32", "x64")).toBe("x86_64-pc-windows-msvc");
    expect(rustTarget("linux", "arm64")).toBeNull();
  });

  it("names a single release asset for that target", () => {
    expect(assetName("aarch64-apple-darwin")).toBe("awesome-schema-aarch64-apple-darwin");
    expect(assetName("x86_64-pc-windows-msvc")).toBe("awesome-schema-x86_64-pc-windows-msvc.exe");
    expect(releaseUrl("0.0.0-alpha.0", "x86_64-unknown-linux-gnu")).toBe(
      "https://github.com/davp00/awesome-schema/releases/download/v0.0.0-alpha.0/awesome-schema-x86_64-unknown-linux-gnu",
    );
  });
});
