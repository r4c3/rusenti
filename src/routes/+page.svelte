<script lang="ts">
  import init, { greet } from "wasm";
  import { onMount } from "svelte";

  onMount(async () => {
    await init();
  });
</script>

<div>
  <button
    on:click={() => {
      greet("hello");
    }}>Click Me</button
  >
</div>

