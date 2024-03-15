<script>
  import { slide } from "svelte/transition";

  export let title = "";
  export let items = [];

  let isCollapsed = false;

  function toggleCollapse() {
    isCollapsed = !isCollapsed;
  }
</script>

<div class="collapsible-project-pane">
  <div class="project-pane-title" on:click={toggleCollapse}>
    <h3>{title}</h3>
    <span>{isCollapsed ? "+" : "-"}</span>
  </div>

  {#if !isCollapsed}
    <ul transition:slide={{ duration: 230 }}>
      {#each items as item}
        <li>{item}</li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .collapsible-project-pane {
    border-radius: 4px;
    overflow: hidden;
  }

  .project-pane-title {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    cursor: pointer;
  }

  h3 {
    margin: 0;
    font-weight: 500;
    font-size: 1.25em;
  }

  ul {
    list-style-type: none;
    padding: 0;
    margin: 0;
  }

  li {
    color: var(--off-white);
    padding: 10px;
  }
</style>
