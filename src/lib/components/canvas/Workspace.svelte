<script lang="ts">
  import LeftPane from "./LeftPane/LeftPane.svelte";
  import Canvas from "./Canvas/WASMHydrator.svelte";
  import RightPane from "./RightPane/RightPane.svelte";
</script>

<div id="workspace-container">
  <LeftPane />
  <Canvas />
  <RightPane />
</div>

<style>
  #workspace-container {
    width: 100%;
    display: flex;
  }
</style>
