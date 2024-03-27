<script lang="ts">
  import { onMount } from "svelte";
  import init, { World, LayerManager } from "wasm";

  export let color: { r: number; g: number; b: number };

  let canvasElement: HTMLCanvasElement;
  let world: World;
  let layerManager: LayerManager;
  let startPos: { x: number; y: number } = { x: 0, y: 0 };
  let panning = false;
  let drawing = false;

  onMount(async () => {
    await init();
    world = new World("canvas");
    layerManager = new LayerManager(8192, 8192);
    world.set_layer_manager(layerManager);
    updateCanvasSize();
    window.addEventListener("resize", updateCanvasSize);
  });

  function handleMouseDown(event: MouseEvent) {}

  function handleMouseMove(event: MouseEvent) {}

  function handleMouseUp() {}

  function handleWheel(event: WheelEvent) {}

  function updateCanvasSize() {}
</script>

<canvas
  bind:this={canvasElement}
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
