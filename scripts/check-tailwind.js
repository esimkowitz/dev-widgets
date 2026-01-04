#!/usr/bin/env node
/**
 * Ensures assets/tailwind.css is up to date with source files.
 * If stale, rebuilds and stages the updated CSS.
 */

import { execSync } from "child_process";
import { createHash } from "crypto";
import { readFileSync, existsSync } from "fs";

const CSS_FILE = "assets/tailwind.css";

function getFileHash(filepath) {
  if (!existsSync(filepath)) return null;
  const content = readFileSync(filepath);
  return createHash("sha1").update(content).digest("hex");
}

// Get hash before rebuild
const beforeHash = getFileHash(CSS_FILE);

// Rebuild
execSync("npm run tailwind:build --silent", { stdio: "inherit" });

// Get hash after rebuild
const afterHash = getFileHash(CSS_FILE);

if (beforeHash !== afterHash) {
  console.log(`Tailwind CSS was out of date. Rebuilt and staging ${CSS_FILE}`);
  execSync(`git add ${CSS_FILE}`, { stdio: "inherit" });
}
