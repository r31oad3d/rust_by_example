import {Universe, Cell} from "wasm-game-of-life";
import * as memory from "wasm-game-of-life/wasm_game_of_life_bg"
const CELL_SIZE : number = 5;
const GRID_COLOR : string = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas:any = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext('2d');
let animationId = null
const isPaused = () => {
    return animationId === null;
}
