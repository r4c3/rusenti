<script lang="ts">
  export let side: "left" | "right";
</script>

<div class="resizable-pane">
  <div
    class="resize-handle"
    style={side === "left" ? "right: 0;" : "left: 0;"}
  ></div>
  <slot />
</div>

<style>
  .resizable-pane {
    position: relative;
    background-color: var(--dark-gray); /* Light grey background */
    width: 30rem; /* Default width, adjust as needed */
    height: 100%; /* Full viewport height */
    padding: 2rem;
    box-shadow: 0 0 8px rgba(0, 0, 0, 0.1); /* Soft shadow for some depth */
  }

  .resize-handle {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px; /* Handle width */
    cursor: ew-resize;
    background-color: var(--light-gray); /* Handle color */
  }
</style>
