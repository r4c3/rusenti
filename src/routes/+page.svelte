<script lang="ts">
  import init, { greet } from "wasm";
  import { onMount } from "svelte";
  import getGoogleOAuthURL from "$lib";
  onMount(async () => {
     await init();
  });
</script>

<div>
  <button
    on:click={() => {
     // greet("hello");
    }}>Click Me</button
  >
  <a href={getGoogleOAuthURL()}>loginWGoogle</a>
</div>