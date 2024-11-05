import { describe, it, expect } from "vitest";
import { globSync } from "glob";
import * as path from "path";
import { getAllCategories } from "./soundUtils.mjs";

describe("Sound Utils", () => {
  it("Get all categories returns everything", () => {
    const dir = path.join(import.meta.dirname, "..", "..", "sounds", "*");
    const expected = new Set(globSync(dir).map((e) => path.basename(e)));

    const sounds = getAllCategories();

    expect(sounds).to.have.all.keys(...expected);
  });
});
