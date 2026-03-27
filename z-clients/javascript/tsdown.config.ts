import { defineConfig } from "tsdown";

export default defineConfig({
  tsconfig: "tsconfig.json",
  outDir: "dist",
  format: {
    esm: {
      target: ["es2022"],
    },
  },
});
