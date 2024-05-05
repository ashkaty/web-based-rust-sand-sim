import init, { Grid, Vector2, Element } from './pkg/web_based_rust_sandsim.js';

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

    

    canvas.addEventListener('click', (event) => {
        const rect = canvas.getBoundingClientRect();
        const x = Math.floor((event.clientX - rect.left) / cellSize);
        const y = Math.floor((event.clientY - rect.top) / cellSize);
        grid.set(new Vector2(x, y), Element.sand());
    });

    function update() {


        grid.update();

    

        ctx.fillStyle = 'black';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        for (let y = 0; y < gridHeight; y++) {
            for (let x = 0; x < gridWidth; x++) {
                const element = grid.get(new Vector2(x, y));
                // console.log(`${element.to_string()}`)
                const color = element.get_color();
                ctx.fillStyle = `rgb(${color.r}, ${color.g}, ${color.b})`;
                ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
            }
        }

        requestAnimationFrame(update);
    }

    update();
}

document.addEventListener("DOMContentLoaded", run);