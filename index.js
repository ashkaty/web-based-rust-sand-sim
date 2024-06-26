import init, { Grid } from './pkg/web_based_rust_sandsim.js';

async function run() {
    await init();

    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');
    const gridWidth = 226;
    const gridHeight = 126;
    const cellSize = 5;


    const grid = new Grid(gridWidth, gridHeight);

    canvas.width = gridWidth * cellSize;
    canvas.height = gridHeight * cellSize;

    let mouse_down = false;
    let mouse_pos_x = 0;
    let mouse_pos_y = 0;

    canvas.addEventListener('mousedown', (event) => {
        mouse_down = true;
        const rect = canvas.getBoundingClientRect();
        mouse_pos_x = Math.floor((event.clientX - rect.left) / cellSize);
        mouse_pos_y = Math.floor((event.clientY - rect.top) / cellSize);
        grid.set_mouse(mouse_pos_x, mouse_pos_y);
    });
    canvas.addEventListener('mousemove', (event) => {
        const rect = canvas.getBoundingClientRect();
        mouse_pos_x = Math.floor((event.clientX - rect.left) / cellSize);
        mouse_pos_y = Math.floor((event.clientY - rect.top) / cellSize);
    });
    canvas.addEventListener('mouseup', () => {
        mouse_down = false;
    });
    canvas.addEventListener('keydown', function(event) {
        // console.log(event.key)
        grid.handle_input(event.key);
    });


    function update() {
        if (mouse_down) {
            grid.draw_mouse(mouse_pos_x, mouse_pos_y);
        }
        grid.render(ctx, 5);
        requestAnimationFrame(update);
    }

    update();
}

// document.addEventListener("DOMContentLoaded", run);
run();