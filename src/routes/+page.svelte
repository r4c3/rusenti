<script lang="ts">
    export let data: string;
</script>

<main>
    {@html data.data}
</main>


<style>
  main {
    display: flex;
    flex-direction: column;
    height: 100vh;
    align-items: center;
    justify-content: center;
  }
</style>
