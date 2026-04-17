// Shared list of files that track the project version.
// Each regex must have exactly three capture groups:
//   1. prefix (kept as-is)
//   2. the version string (replaced by bump-version.js, read by check-version-sync.js)
//   3. suffix (kept as-is)
const versionFiles = [
    // Cargo workspace — scoped to the [workspace.package] block so only that version is touched.
    {
        path: "Cargo.toml",
        label: "Cargo.toml [workspace.package]",
        regex: /(\[workspace\.package\][\s\S]*?\nversion\s*=\s*")([^"]*)(")/,
    },
    // JavaScript — matches the first "version" field (the package's own).
    {
        path: "z-clients/javascript/package.json",
        label: "z-clients/javascript/package.json",
        regex: /("version"\s*:\s*")([^"]*)(")/,
    },
    {
        path: "z-clients/javascript/src/request.ts",
        label: "z-clients/javascript/src/request.ts LIB_VERSION",
        regex: /(export const LIB_VERSION\s*=\s*")([^"]*)(")/,
    },
    // Python
    {
        path: "z-clients/python/pyproject.toml",
        label: "z-clients/python/pyproject.toml",
        regex: /(^version\s*=\s*")([^"]*)(")/m,
    },
    {
        path: "z-clients/python/diom/client_base.py",
        label: "z-clients/python/diom/client_base.py user-agent",
        regex: /("user-agent":\s*"svix-libs\/)([^/]*)(\/python")/,
    },
    // Java — pom.xml regex matches the first <version> tag (the project's own).
    {
        path: "z-clients/java/pom.xml",
        label: "z-clients/java/pom.xml",
        regex: /(<version>)([^<]*)(<\/version>)/,
    },
    {
        path: "z-clients/java/src/main/java/com/svix/diom/Version.java",
        label: "z-clients/java/src/main/java/com/svix/diom/Version.java VERSION",
        regex: /(public static final String VERSION\s*=\s*")([^"]*)(")/,
    },
    {
        path: "z-clients/java/README.md",
        label: "z-clients/java/README.md <version>",
        regex: /(<version>)([^<]*)(<\/version>)/,
    },
    {
        path: "z-clients/java/README.md",
        label: "z-clients/java/README.md gradle",
        regex: /(com\.svix:diom:)([^"]*)(")/,
    },
    // Go
    {
        path: "z-clients/go/client.go",
        label: "z-clients/go/client.go user-agent",
        regex: /("diom-sdks\/)([^/]*)(\/go")/,
    },
];

module.exports = { versionFiles };
