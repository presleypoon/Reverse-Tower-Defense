import initWasm, {game_start} from "../pkg/Reverse_Tower_Defense.js";

function render() {
  throw new Error("still making");
}

function log(string) {
  console.log(string);
}

function error(string) {
  console.error(string);
}

await initWasm();
game_start();
