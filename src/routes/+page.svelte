<script lang="ts">
  import { goto } from "$app/navigation";

  const startBuilding = () => {
    goto("/projects");
  };
</script>

<main>
  <h1 id="skru-title">Skru</h1>
  <p id="info-text">A paletted bitmap editor for the web.</p>
  <button id="start-building-button" on:click={startBuilding}>
    Start building -->
  </button>
</main>

<style>
  main {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding-bottom: 5rem;
    gap: 1rem;
  }
  #skru-title {
    font-size: 9rem;
    font-weight: 900;
    margin-bottom: -1.4rem;
  }
  #info-text {
    font-size: 2rem;
    margin-bottom: 0.3rem;
    font-weight: 600;
  }
  #start-building-button {
    font-family: inherit;
    padding: 1rem 2rem;
    font-size: 1.5rem;
    font-weight: 800;
    color: var(--white);
    border: 2px solid var(--accent);
    background-color: transparent;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: 0.3s ease all;
  }
  #start-building-button:hover {
    background-color: var(--accent);
  }
</style>
