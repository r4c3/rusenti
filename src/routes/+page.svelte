<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "$lib/components";

  const startBuilding = () => {
    goto("/canvas");
  };
</script>

<main>
  <div id="content">
    <h1>Skru</h1>
    <p>A paletted bitmap editor for the web.</p>
    <p class="bullet">• GPU-accelerated canvas rendering</p>
    <p class="bullet">• Server-side rendered static components</p>
    <p class="bullet">• Export to .png, .jpg, .c, and more</p>
    <Button on:click={startBuilding}>Start building --></Button>
  </div>
</main>

<style>
  main {
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
</style>
