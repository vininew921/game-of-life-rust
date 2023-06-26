<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  const CELL_SIZE = 10; // px
  const GRID_COLOR = "#CCCCCC";
  const DEAD_COLOR = "#FFFFFF";
  const ALIVE_COLOR = "#000000";
  const WIDTH = 64;
  const HEIGHT = 64;

  let global_cells = Array<number>();

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  window.onload = () => {
    canvas.width = (CELL_SIZE + 1) * WIDTH + 1;
    canvas.height = (CELL_SIZE + 1) * HEIGHT + 1;

    ctx = canvas.getContext("2d");

    renderLoop();
  };

  const tickUniverse = () => {
    invoke("tick_universe").then((res: Array<number>) => (global_cells = res));
  };

  const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    //Vertical lines
    for (let i = 0; i <= WIDTH; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
    }

    //Horizontal lines
    for (let j = 0; j <= HEIGHT; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
  };

  const getIndex = (row: number, column: number): number => {
    return row * WIDTH + column;
  };

  const drawCells = () => {
    ctx.beginPath();

    for (let row = 0; row < HEIGHT; row++) {
      for (let col = 0; col < WIDTH; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = global_cells[idx] === 0 ? DEAD_COLOR : ALIVE_COLOR;

        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    ctx.stroke();
  };

  const fps = 20;

  const renderLoop = () => {
    tickUniverse();
    drawGrid();
    drawCells();

    setTimeout(() => {
      renderLoop();
    }, 1000 / fps);
  };
</script>

<div id="game">
  <canvas id="game-canvas" bind:this={canvas} />
</div>

<style>
  #game {
    width: 100%;
    height: 100%;
  }
</style>
