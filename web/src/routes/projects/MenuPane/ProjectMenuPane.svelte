<script>
  import { goto } from "$app/navigation";

  import { NormalButton, MenuPane } from "$lib/components";
  import ProjectGroups from "./ProjectGroups.svelte";
  import CollapsibleProjectSublist from "./CollapsibleProjectSublist.svelte";

  // TODO: replace with user data
  const groups = ["Home", "CS2261"];
  const folders = ["Folder 1", "Folder 2", "Folder 3"];
  const favorites = ["Favorite 1", "Favorite 2", "Folder 9"];
  const recents = ["Recent 1", "Recent 2", "Recent 3"];

  const openNewProject = () => {
    goto("/canvas");
  };
</script>

<MenuPane side="left">
  <div id="menu-pane-header">
    <h1 id="skru-title">Skru</h1>
    <h1 id="version-subtitle">/ v0.0.1</h1>
  </div>

  <hr class="divider" />

  <div id="menu-pane-button-container">
    <NormalButton on:click={openNewProject} style="hollow">
      <p class="new-button-text">New Project</p>
    </NormalButton>
    <NormalButton style="hollow">
      <p class="new-button-text">New Folder</p>
    </NormalButton>
  </div>

  <hr class="divider" />

  <div id="menu-pane-groups">
    <ProjectGroups {groups} />
  </div>

  <hr class="divider" />

  <CollapsibleProjectSublist title="Folders" items={folders} />

  <hr class="divider" />

  <CollapsibleProjectSublist title="Favorites" items={favorites} />

  <hr class="divider" />

  <CollapsibleProjectSublist title="Recents" items={recents} />
</MenuPane>

<style>
  #menu-pane-header {
    display: flex;
  }
  #skru-title {
    color: var(--accent);
    margin-right: 0.75rem;
  }
  #version-subtitle {
    color: var(--disabled-gray);
    font-weight: 500;
  }
  #menu-pane-button-container {
    display: flex;
    justify-content: space-between;
  }
  .new-button-text {
    font-size: 1.3rem;
    color: var(--accent);
    font-weight: 400;
    padding: 0.6rem 1.4rem;
  }
  .divider {
    width: calc(100% + 4rem);
    margin: 2rem 0rem 2rem -2rem;
    background-color: var(--light-gray);
    border: none;
    height: 2px;
  }
</style>
