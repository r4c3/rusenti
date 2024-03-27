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
  let cursor = cursorState.PAN;

  onMount(async () => {
    await init();
    world = new World(500, 500);
    console.log(world, "world");
    world.render(); //black
  });

  /**
   * @param {MouseEvent} event
   */
  function handleMouseDown(event) {
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
      }
      default: {
        world.render();
      }
    }
  }

  /**
   * @param {MouseEvent} event
   */
  function handleMouseUp(event) {}

  /**
   * @param {WheelEvent} event
   */
  function handleWheel(event) {}

  function updateCanvasSize() {}
</script>

<canvas
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
