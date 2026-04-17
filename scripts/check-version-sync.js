#!/usr/bin/env node
// Checks that all version strings across the repo match the canonical .version file.
const { readFileSync } = require("fs");
const { join } = require("path");
const { versionFiles } = require("./version-files");

const repoRoot = join(__dirname, "..");
const expected = readFileSync(join(repoRoot, ".version"), "utf8").trim();

let errors = 0;
for (const { path, label, regex } of versionFiles) {
    const absPath = join(repoRoot, path);
    const content = readFileSync(absPath, "utf8");
    const match = content.match(regex);
    if (!match) {
        console.error(`NO MATCH ${label} in ${path}`);
        errors++;
        continue;
    }
    const actual = match[2];
    if (actual !== expected) {
        console.error(`MISMATCH ${label}: got '${actual}', expected '${expected}'`);
        errors++;
    }
}

if (errors > 0) {
    console.error(
        `\nERROR: ${errors} version mismatch(es) found. All versions must match .version (${expected}).`,
    );
    process.exit(1);
}

console.log(`All versions match: ${expected}`);
