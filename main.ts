import init, { start } from "./pkg/wasm_webgpu_test";

await init();
start(document.getElementById("GameView"));
