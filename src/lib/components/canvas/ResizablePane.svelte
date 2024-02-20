<script lang="ts">
  let isResizing = false;
  let startX: number;
  let startWidth: number;

  function onMouseDown(event: MouseEvent): void {
    isResizing = true;
    startX = event.clientX;
    startWidth = element.offsetWidth;
    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("mouseup", onMouseUp);
  }

  function onMouseMove(event: MouseEvent): void {
    if (!isResizing) return;
    const dx = event.clientX - startX;
    const newWidth = startWidth - dx;
    element.style.width = `${newWidth}px`;
  }

  function onMouseUp(event: MouseEvent): void {
    if (isResizing) {
      window.removeEventListener("mousemove", onMouseMove);
      window.removeEventListener("mouseup", onMouseUp);
      isResizing = false;
    }
  }

  let element: HTMLElement;
</script>

<div bind:this={element} class="resizable">
  <div class="handle" on:mousedown={onMouseDown} />
  <slot />
</div>

<style>
  .resizable {
    position: relative;
    display: flex;
    flex-direction: row-reverse;
    overflow: hidden;
    width: 200px; /* Initial width */
    min-width: 100px; /* Minimum width */
    height: 100%;
    background-color: var(--dark-gray);
  }

  .handle {
    cursor: ew-resize;
    width: 10px;
    height: 100%;
  }
</style>
