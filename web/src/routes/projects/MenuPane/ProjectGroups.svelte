<script>
  export let groups = [];

  let selectedGroup = groups[0];

  function selectGroup(group) {
    selectedGroup = group;
  }
</script>

<div id="group-item-button-container">
  {#each groups as group}
    <div
      class="group-item {group === selectedGroup ? 'selected' : ''}"
      on:click={() => selectGroup(group)}
    >
      <h3>{group}</h3>
    </div>
  {/each}
</div>

<style>
  #group-item-button-container {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    cursor: pointer;
    gap: 0.8rem;
  }
  .group-item {
    width: 100%;
    display: flex;
    padding: 0.4em 1em;
    border-radius: 2rem;
    align-items: center;
    gap: 0.4em;
    transition: 0.3s ease all;
  }
  .group-item:hover {
    background-color: var(--light-gray); /* Hover effect */
  }
  .group-item.selected {
    background-color: gray; /* Selected item background */
  }
  h3 {
    font-weight: 500;
    font-size: 1.25em;
  }
</style>
