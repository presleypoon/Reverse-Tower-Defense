import initWasm, {game_start} from "../pkg/Reverse_Tower_Defense.js";

export function render() {
  throw new Error("still making");
}

export function log(string) {
  console.log(string);
}

export function error(string) {
  console.error(string);
}

await initWasm();
game_start();
