<script>
  import { onMount } from "svelte";
  import init, { Viewfinder } from "wasm";

  /** @type {HTMLCanvasElement} */
  let canvasElement;

  /** @type {Viewfinder} */
  let viewfinder;

  /** @type {{x: number, y: number}} */
  let startPos = { x: 0, y: 0 };

  /** @type {boolean} */
  let panning = false;

  /** @type {boolean}*/
  let drawing = false;
  /**
   * Initializes the WASM module and the viewfinder.
   */
  onMount(async () => {
    await init();
    viewfinder = new Viewfinder("canvas");
    updateCanvasSize();
    window.addEventListener("resize", updateCanvasSize);
  });

  /**
   * Handles the mouse down event to start panning.
   * @param {MouseEvent} event - The mouse down event.
   */
  function handleMouseDown(event) {
    startPos.x = event.clientX;
    startPos.y = event.clientY;
    panning = false;
    drawing = true;
  }

  /**
   * Handles the mouse move event to pan the view.
   * @param {MouseEvent} event - The mouse move event.
   */
  function handleMouseMove(event) {
    if (!drawing) return;
    viewfinder.color(event.clientX, event.clientY);
    if (!panning) return;
    const dx = (event.clientX - startPos.x);
    const dy = (event.clientY - startPos.y);
    viewfinder.pan(dx, dy);
    startPos.x = event.clientX;
    startPos.y = event.clientY;
  }

  /**
   * Handles the mouse up event to stop panning.
   */
  function handleMouseUp() {
    panning = false;
    drawing = false;
  }

  /**
   * Handles the wheel event to zoom in or out.
   * @param {WheelEvent} event - The wheel event.
   */
  function handleWheel(event) {
    event.preventDefault(); //no scroll
    const zoomFactor = event.deltaY > 0 ? 0.9 : 1.1;
    viewfinder.zoom(zoomFactor);
  }

  /**
   * Updates the canvas size to match its display size.
   */
  function updateCanvasSize() {
    const rect = canvasElement.getBoundingClientRect();
    canvasElement.width = rect.width;
    canvasElement.height = rect.height;
    viewfinder.render();
  }
</script>

<canvas
  bind:this={canvasElement}
  on:mousedown={handleMouseDown}
  on:mousemove={handleMouseMove}
  on:mouseup={handleMouseUp}
  on:mouseleave={handleMouseUp}
  on:wheel={handleWheel}
  id="canvas"
>
</canvas>

<style>
  canvas {
    width: 100%;
    height: 100%;
    background-color: var(--black);
  }
</style>
