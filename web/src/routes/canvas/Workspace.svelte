<script lang="ts">
  import LeftPane from "./LeftPane/LeftPane.svelte";
  import Canvas from "./WASMHydrator.svelte";
  import RightPane from "./RightPane/RightPane.svelte";
  var value: any;
</script>

<div id="workspace-container">
  <LeftPane />
  <Canvas color={value} />
  <RightPane bind:hex={value} />
</div>

<style>
  #workspace-container {
    width: 100%;
    display: flex;
  }
</style>
