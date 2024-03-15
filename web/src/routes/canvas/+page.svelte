<script lang="ts">
  import { CanvasHeader, CanvasWorkspace } from "$lib/components";
</script>

<main>
  <CanvasHeader />
  <CanvasWorkspace />
</main>

<style>
  main {
    display: grid;
    /* the workspace assumes the screen height - header height*/
    grid-template-rows: auto 1fr;
    height: 100%;
  }
</style>
