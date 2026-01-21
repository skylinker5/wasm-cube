import init, { start } from "../wasm/pkg/wasm_cube";

async function main() {
  await init();
  start("canvas");
}

main();
