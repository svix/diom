#!/usr/bin/env node
// Propagates the version from .version to every file that tracks it.
const { readFileSync, writeFileSync } = require("fs");
const { join } = require("path");
const { spawnSync } = require("child_process");
const semver = require("semver");
const { versionFiles } = require("./version-files");

const repoRoot = join(__dirname, "..");
const version = readFileSync(join(repoRoot, ".version"), "utf8").trim();

if (!semver.valid(version)) {
    console.error(`ERROR: .version does not contain a valid semver: '${version}'`);
    process.exit(1);
}

for (const { path, label, regex } of versionFiles) {
    const absPath = join(repoRoot, path);
    const content = readFileSync(absPath, "utf8");
    const match = content.match(regex);
    if (!match) {
        console.error(`ERROR: no match for ${label} in ${path}`);
        process.exit(1);
    }
    const oldVersion = match[2];
    if (oldVersion !== version && !semver.gt(version, oldVersion)) {
        console.error(
            `ERROR: new version '${version}' is not greater than '${oldVersion}' in ${label} (${path})`,
        );
        process.exit(1);
    }
    const updated = content.replace(regex, (_, pre, _old, post) => `${pre}${version}${post}`);
    writeFileSync(absPath, updated);
}

console.log(`Updated all versions to: ${version}`);

const result = spawnSync("node", [join(__dirname, "check-version-sync.js")], { stdio: "inherit" });
process.exit(result.status ?? 1);
