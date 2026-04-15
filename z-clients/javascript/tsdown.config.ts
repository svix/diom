import { defineConfig } from "tsdown";

export default defineConfig({
  tsconfig: "tsconfig.json",
  entry: ["src/index.ts", "src/internal.ts"],
  outDir: "dist",
  format: {
    esm: {
      target: ["es2022"],
    }
  },
  sourcemap: true,
  outputOptions: {
    chunkFileNames: "[hash].mjs", // the default ([name]-[hash].mjs) produces stupid names
  },
});
