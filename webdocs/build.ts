import { $ } from "bun";
import fs from "fs";

await $`cd ../lotus-script && cargo doc --no-deps --target-dir ../webdocs/target`;

fs.rmSync("public", { recursive: true, force: true });

fs.mkdirSync("public");

const docDir = `target/doc`;
fs.cpSync(docDir, "public", { recursive: true });
