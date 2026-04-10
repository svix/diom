import { defineConfig } from "vitest/config";

export default defineConfig({
  ssr: {
    noExternal: ["yargs"],
  },
  test: {
    environment: "node",
    include: ["src/**/*.test.ts"],
  },
});
