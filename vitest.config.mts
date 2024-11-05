import { defineConfig } from "vitest/config";
import * as path from "path";

export default defineConfig({
  test: {
    include: ["src/**/*.test.mts"],
    env: {
      OMGBOT_SOUND_DIR: path.join(import.meta.dirname, "sounds"),
    },
  },
});
