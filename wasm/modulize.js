import { readFileSync, writeFileSync, unlinkSync } from "fs";

const dirName = "./wasm/pkg/"; // change this to match your Rust library's name

const content = readFileSync(dirName + "package.json");

const packageJSON = JSON.parse(String(content));
packageJSON["type"] = "module";

writeFileSync(dirName + "package.json", JSON.stringify(packageJSON));
