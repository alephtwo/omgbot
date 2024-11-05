import esbuild from "esbuild";
import * as path from "path";

await esbuild.build({
  entryPoints: ["src/server.mts"],
  outdir: path.join(import.meta.dirname, "..", "dist"),
  platform: "node",
  bundle: true,
  minify: true,
  sourcemap: true,
  format: "esm",
  outExtension: { ".js": ".mjs" },
  logLevel: "info",
  packages: "external",
});
