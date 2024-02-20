<script lang="ts">
  import { onMount } from "svelte";

  export let resizeSide: "left" | "right" = "right";

  let startX: number, startWidth: number;

  function initResize(event: MouseEvent): void {
    startX = event.clientX;
    startWidth = (event.target as HTMLElement).parentElement!.offsetWidth;
    document.addEventListener("mousemove", startResizing);
    document.addEventListener("mouseup", stopResizing);
  }

  function startResizing(event: MouseEvent): void {
    let newWidth: number;
    if (resizeSide === "right") {
      newWidth = startWidth + event.clientX - startX;
    } else {
      // 'left'
      newWidth = startWidth - (event.clientX - startX);
    }
    (event.target as HTMLElement).parentElement!.style.width = `${newWidth}px`;
  }

  function stopResizing(): void {
    document.removeEventListener("mousemove", startResizing);
    document.removeEventListener("mouseup", stopResizing);
  }

  onMount(() => {
    //todo
  });
</script>

<div class="pane" on:mousedown={initResize}>
  <slot />
</div>

<style>
  .pane {
    height: 100%;
    background-color: var(--dark-gray);
    position: relative;
    overflow: auto;
    resize: horizontal;
    width: 200px;
    min-width: 100px;
    max-width: 100%;
  }

  .pane:after {
    content: " ";
    display: block;
    position: absolute;
    top: 0;
    /* Position handle on the left or right based on the resizeSide prop */
    left: var(--resizer-position-left, auto);
    right: var(--resizer-position-right, auto);
    width: 10px;
    height: 100%;
    cursor: ew-resize;
    z-index: 1;
  }

  :global() .pane {
    --resizer-position-right: {
      resizeside=== 'right' ? '-5px' : "auto";
    };
    --resizer-position-left: {
      resizeside=== 'left' ? '-5px' : "auto";
    };
  }
</style>
