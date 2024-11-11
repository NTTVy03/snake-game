import init, { World } from "snake_game";

init().then(_ => {
    const CELL_SIZE = 10;

    const world = World.new();
    const worldWidth = world.width();

    // canvas is an UI component that display the world grid
    const canvas = document.getElementById("snake-canvas");
    // context of canvas help us interact with the canvas
    const context = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

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
        const snake_head_idx = world.snake_head_idx();
        const row = Math.floor(snake_head_idx / worldWidth);
        const column = snake_head_idx % worldWidth;

        console.log(`Snake position: (${row}, ${column})`);

        // at the begin, snake length = 1 --> fill that cell
        context.beginPath();
        context.fillRect(
            column * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        );
        context.stroke();
    }

    function paint() {
        drawWorld();
        drawSnake();
    }

    function update() {
        // update the world grid after an interval time
        setTimeout(() => {
            // clear the current canvas
            context.clearRect(0, 0, canvas.width, canvas.height);
            
            // draw new world grid and snake
            world.update();
            paint();

            // this function optimized the rendering
            // we do not need to specify interval time between each re-rendering time
            requestAnimationFrame(update);
        }, 100)
    }

    paint();
    update();
})