import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts", "src/zugferd_2_3_2/index.ts"],
  format: ["cjs", "esm"], // Build for commonJS and ESmodules
  dts: true, // Generate declaration file (.d.ts)
  splitting: false,
  sourcemap: true,
  clean: true,
});
