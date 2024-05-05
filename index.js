import init, { Grid, Vector2, Element } from './pkg/web_based_rust_sandsim.js';

async function run() {
    await init();

    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');
    const GRID_WIDTH = 226;
    const GRID_HEIGHT = 126;
    const CELL_SIZE = 5;
    const TARGET_FPS = 60; // Desired frame rate
    const FRAME_INTERVAL = 1000 / TARGET_FPS; // Time between frames in milliseconds

    const grid = new Grid(GRID_WIDTH, GRID_HEIGHT);

    canvas.width = GRID_WIDTH * CELL_SIZE;
    canvas.height = GRID_HEIGHT * CELL_SIZE;

    let lastUpdateTime = performance.now();

    canvas.addEventListener('click', (event) => {
        const rect = canvas.getBoundingClientRect();
        const x = Math.floor((event.clientX - rect.left) / CELL_SIZE);
        const y = Math.floor((event.clientY - rect.top) / CELL_SIZE);
        grid.set(new Vector2(x, y), Element.sand());
    });

    function update() {
        const currentTime = performance.now();
        const elapsed = currentTime - lastUpdateTime;

        if (elapsed >= FRAME_INTERVAL) {
            grid.update();
            lastUpdateTime = currentTime;
        }

        ctx.fillStyle = 'black';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        for (let y = 0; y < GRID_HEIGHT; y++) {
            for (let x = 0; x < GRID_WIDTH; x++) {
                const element = grid.get(new Vector2(x, y));
                const color = element.get_color();
                ctx.fillStyle = `rgb(${color.r}, ${color.g}, ${color.b})`;
                ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
            }
        }

        requestAnimationFrame(update);
    }

    update();
}

document.addEventListener("DOMContentLoaded", run);