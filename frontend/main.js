import initWasm, {game_start, game_tick as rust_gt} from "../pkg/Reverse_Tower_Defense.js";

function render() {
  throw new Error("still making");
}

window.render = render;

await initWasm();
game_tick();

function game_tick() {
  rust_gt();
  requestAnimationFrame(game_tick);
}
