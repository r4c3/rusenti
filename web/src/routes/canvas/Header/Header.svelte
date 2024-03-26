<script lang="ts">
  import ProjectBar from "./ProjectBar.svelte";
  import CenterBar from "./CenterBar.svelte";
  import RightButtons from "./RightButtons.svelte";
</script>

<header>
  <ProjectBar />
  <CenterBar />
  <RightButtons />
</header>

<style>
  header {
    width: 100%;
    background-color: var(--light-gray);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 2rem 3rem;
  }
</style>
