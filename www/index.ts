import init, { World, Direction } from "snake_game";
import { random } from "./utils/rand";

init().then(wasm => {
    const CELL_SIZE = 10;
    const WORLD_WIDTH = 8;
    const WORLD_SIZE = WORLD_WIDTH * WORLD_WIDTH;
    const snakeSpawnIdx = random(WORLD_SIZE);
    const SNAKE_LENGTH = 3;

    const world = World.new(WORLD_WIDTH, snakeSpawnIdx, SNAKE_LENGTH);
    const worldWidth = world.width();

    // canvas is an UI component that display the world grid
    const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
    // context of canvas help us interact with the canvas
    const context = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

    document.addEventListener("keydown", (e) => {
        switch (e.code) {
            case "ArrowLeft":
                world.change_snake_direction(Direction.Left);
                break;
            case "ArrowRight":
                world.change_snake_direction(Direction.Right);
                break;
            case "ArrowDown":
                world.change_snake_direction(Direction.Down);
                break;
            case "ArrowUp":
                world.change_snake_direction(Direction.Up);
        }
    })

    function drawWorld() {
        context.beginPath();

        // draw the vertival line
        for (let x = 0; x <= worldWidth; x++) {
            context.moveTo(x * CELL_SIZE, 0);
            context.lineTo(x * CELL_SIZE, worldWidth * CELL_SIZE);
        }

        // draw the horizontal line
        for (let y = 0; y <= worldWidth; y++) {
            context.moveTo(0, y * CELL_SIZE);
            context.lineTo(worldWidth * CELL_SIZE, y * CELL_SIZE);
        }

        context.stroke();
    }

    function drawSnake() {
        // get list of snake cells
        const snakeCells = new Uint32Array (
            wasm.memory.buffer,
            world.snake_cells(),
            world.snake_length()
        )
        
        snakeCells.forEach((cellIdx, i) => {
            // set other color for snake head
            context.fillStyle = (i == 0 ? "#7878db" : "#000000");

            const column = cellIdx % worldWidth;
            const row = Math.trunc(cellIdx / worldWidth);

            context.beginPath();
            context.fillRect(
                column * CELL_SIZE,
                row * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE
            );
            context.stroke();
        })
    }

    function drawReward() {
        // get list of snake cells
        const rewardCells = new Uint32Array (
            wasm.memory.buffer,
            world.reward_cell(),
            1
        )
        
        rewardCells.forEach((cellIdx, i) => {
            context.fillStyle = "#FF0000";

            const column = cellIdx % worldWidth;
            const row = Math.trunc(cellIdx / worldWidth);

            context.beginPath();
            context.fillRect(
                column * CELL_SIZE,
                row * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE
            );
            context.stroke();
        })
    }

    function paint() {
        drawWorld();
        drawSnake();
        drawReward();
    }

    function update() {
        // fps: frames per second
        // increase fps for faster snake movement
        const fps = 5;

        // update the world grid after an interval time
        setTimeout(() => {
            // clear the current canvas
            context.clearRect(0, 0, canvas.width, canvas.height);

            // draw new world grid and snake
            world.step();
            paint();

            // this function optimized the rendering
            // we do not need to specify interval time between each re-rendering time
            requestAnimationFrame(update);
        }, 1000 / fps)
    }

    paint();
    update();
})