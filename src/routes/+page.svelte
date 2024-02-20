<script lang="ts">
  import { goto } from "$app/navigation";

  const startBuilding = () => {
    goto("/canvas");
  };
</script>

<div id="screen">
  <div id="content">
    <h1>Skru</h1>
    <p>A paletted bitmap editor for the web.</p>
    <p class="bullet">• GPU-accelerated canvas rendering</p>
    <p class="bullet">• Server-side rendered static components</p>
    <p class="bullet">• Export to .png, .jpg, .c, and more</p>
    <button on:click={startBuilding}> Start building --></button>
  </div>
</div>

<style>
  #screen {
    width: 100%;
    height: 100%;
    padding: 9% 8%;
  }
  #content {
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: fit-content;
    height: fit-content;
  }
  h1 {
    font-size: 15rem;
    font-weight: 900;
  }
  p {
    font-size: 3.4rem;
    margin-bottom: 0.3rem;
    font-weight: 600;
  }
  .bullet {
    margin-left: 1rem;
  }
  button {
    font-size: 3.4rem;
    background: var(--white);
    border: none;
    border-radius: 10rem;
    color: var(--black);
    cursor: pointer;
    width: fit-content;
    padding: 0.5rem 2rem;
    font-family: inherit;
    font-weight: 700;
    margin-top: 3.7rem;
  }
</style>
