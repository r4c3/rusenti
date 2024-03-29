<script>
  import { onMount } from "svelte";
  import init, { World } from "wasm";

  /** @type {World} */
  let world;

  /** @type {{x: number, y: number}} */
  let startPos = { x: 0, y: 0 };

  /** @type {{PAN: string, DRAW: string}} */
  let cursorState = {
    PAN: "PAN",
    DRAW: "DRAW",
  };

  /** @type {string} */
  let cursor = "";
  /** @type {HTMLCanvasElement}*/
  let canvas;
  onMount(async () => {
    await init();
    world = new World(200, 200);
    console.log(world, "world");
    updateCanvasSize();
    world.render(); //black
  });

  /**
   * @param {MouseEvent} event
   */
  function handleMouseDown(event) {
    cursor = cursorState.PAN;
    switch (cursor) {
      case cursorState.PAN:
        startPos = { x: event.clientX, y: event.clientY };
        break;
      case cursorState.DRAW:
        break;
    }
  }

  /**
   * @param {MouseEvent} event
   */
  function handleMouseMove(event) {
    switch (cursor) {
      case cursorState.PAN: {
        const dx = event.clientX - startPos.x;
        const dy = event.clientY - startPos.y;

        if (world == null) {
          return;
        }

        world.pan(dx, dy);
        startPos.x = event.clientX;
        startPos.y = event.clientY;
        world.render();
      }
    }
  }

  /**
   * @param {MouseEvent} event
   */
  function handleMouseUp(event) {
    switch (cursor) {
      case cursorState.PAN: {
        cursor = cursorState.DRAW;
      }
    }
  }

  /**
   * @param {WheelEvent} event
   */
  function handleWheel(event) {
    event.preventDefault(); //no scroll
    const zoomFactor = event.deltaY > 0 ? 0.9 : 1.1;
    world.zoom(zoomFactor);
    world.render();
  }

  function updateCanvasSize() {
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width;
    canvas.height = rect.height;
  }
</script>

<canvas
bind:this={canvas}
  on:mousedown={handleMouseDown}
  on:mousemove={handleMouseMove}
  on:mouseup={handleMouseUp}
  on:mouseleave={handleMouseUp}
  on:wheel={handleWheel}
  id="canvas"
></canvas>

<style>
  canvas {
    width: 100%;
    height: 100%;
    background-color: var(--black);
  }
</style>
