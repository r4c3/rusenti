<script>
  let sizes = [
    { name: "Landscape", dimensions: "1024x512" },
    { name: "Large", dimensions: "512x512" },
    { name: "Medium", dimensions: "128x128" },
    { name: "Small", dimensions: "32x32" },
  ];
</script>

<div id="container">
  <div id="custom-size-option">
    <h2 class="size-name">Custom</h2>
    <p class="size-dimensions">mxn</p>
  </div>

  {#each sizes as size}
    <div class="size-option">
      <h2 class="size-name">{size.name}</h2>
      <p class="size-dimensions">{size.dimensions}</p>
    </div>
  {/each}
</div>

<style>
  #container {
    background-color: var(--dark-gray);
    border: 1px solid var(--light-gray);
    width: 100%;
    border-radius: 2rem;
    display: flex;
    gap: 1rem;
  }
  #custom-size-option {
    background-color: var(--black);
    border: 1px solid var(--light-gray);
    border-radius: 2rem;
    margin: 2rem;
    aspect-ratio: 1;
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
  .size-option {
    flex: 1;
    margin: 2rem;
    aspect-ratio: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
</style>
