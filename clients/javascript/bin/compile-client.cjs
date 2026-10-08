const { existsSync, readFileSync, writeFileSync } = require("node:fs");
const { stripTypeScriptTypes } = require("node:module");
const { dirname, join, resolve, sep } = require("node:path");

function bindings(block) {
  return block
    .split(",")
    .map((name) => name.trim())
    .filter(Boolean);
}

/** Turn stripped generated TypeScript into CommonJS `exports.name =` assignments. */
function esmToCjs(source) {
  let out = source.replace(
    /export\s*\{([\s\S]*?)\}\s*from\s*(['"])(.+?)\2\s*;?/g,
    (_, names, quote, specifier) => {
      const list = bindings(names);
      const lines = [`const __awesomeSchemaRuntime = require(${quote}${specifier}${quote});`];
      for (const name of list) {
        const parts = name.split(/\s+as\s+/);
        const imported = parts[0].trim();
        const local = (parts[1] ?? parts[0]).trim();
        lines.push(`exports.${local} = __awesomeSchemaRuntime.${imported};`);
      }
      return lines.join("\n");
    },
  );
  out = out.replace(
    /import\s*\{([\s\S]*?)\}\s*from\s*(['"])(.+?)\2\s*;?/g,
    (_, names, quote, specifier) => {
      const list = bindings(names);
      if (list.length === 0) return "";
      return `const { ${list.join(", ")} } = require(${quote}${specifier}${quote});`;
    },
  );
  out = out.replace(
    /export\s+async\s+function\s+([A-Za-z0-9_]+)/g,
    "exports.$1 = async function $1",
  );
  out = out.replace(/export\s+function\s+([A-Za-z0-9_]+)/g, "exports.$1 = function $1");
  out = out.replace(
    /export\s+const\s+([A-Za-z0-9_]+)\s*=/g,
    "const $1 = exports.$1 =",
  );
  return out;
}

function typescriptToCommonJs(source) {
  return esmToCjs(stripTypeScriptTypes(source, { mode: "strip" }));
}

function findGeneratedClient(start) {
  let dir = start;
  for (;;) {
    const candidate = join(dir, "node_modules", ".awesome-schema", "client", "index.ts");
    if (existsSync(candidate)) return candidate;
    const parent = dirname(dir);
    if (parent === dir) return null;
    dir = parent;
  }
}

function compileGeneratedClient(start) {
  const tsPath = findGeneratedClient(start);
  if (!tsPath) return null;
  return compileClientFile(tsPath);
}

/** Compile the default client when `tsPath` is `node_modules/.awesome-schema/client/index.ts`. */
function compileClientFile(tsPath) {
  const absolute = resolve(tsPath);
  const marker = `${sep}.awesome-schema${sep}client${sep}index.ts`;
  if (!absolute.endsWith(marker) || !existsSync(absolute)) return null;
  const cjsPath = join(dirname(absolute), "index.cjs");
  writeFileSync(cjsPath, typescriptToCommonJs(readFileSync(absolute, "utf8")));
  return cjsPath;
}

module.exports = {
  compileClientFile,
  compileGeneratedClient,
  findGeneratedClient,
  typescriptToCommonJs,
};
