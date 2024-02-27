<script>
  import { onMount } from "svelte";
  import init from "wasm";

  onMount(async () => {
    await init();
  });
</script>

<canvas id="canvas" />

<style>
  canvas {
    width: 100%;
    height: 100%;
    background-color: var(--black);
  }
</style>
