<script>
  import ProjectMenuPane from "./MenuPane/ProjectMenuPane.svelte";
  import NewWithSizeBar from "./ProjectChoiceView/NewWithSizeBar.svelte";
  import ProjectPreviews from "./ProjectChoiceView/ProjectPreviews.svelte";
</script>

<main>
  <ProjectMenuPane />
  <div id="project-choice-view">
    <NewWithSizeBar />
    <ProjectPreviews />
  </div>
</main>

<style>
  main {
    width: 100%;
    height: 100%;
    display: flex;
  }
  #project-choice-view {
    display: flex;
    justify-content: space-evenly;
    flex-direction: column;
    width: 100%;
    padding: 4rem 12rem;
  }
</style>
