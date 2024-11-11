import init, {World} from "snake_game";

init().then( _ => {
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

    drawWorld();
})