<script>
  // TODO: use actual data
  const projects = [
    {
      name: "Landscape Design",
      src: "https://images.squarespace-cdn.com/content/v1/551a19f8e4b0e8322a93850a/1542954519870-8XJJ4CUXPBEAZHGN6OBT/Landscape.png",
      dimensions: "1024x512",
    },
    {
      name: "Another Design",
      src: "https://pics.craiyon.com/2023-11-05/530b165cacdc4a5cbd757f52c2f61d05.webp",
      dimensions: "512x512",
    },
    {
      name: "Untitled Project",
      src: "https://static.vecteezy.com/system/resources/previews/020/577/130/original/cute-whale-in-pixel-art-style-vector.jpg",
      dimensions: "128x128",
    },
  ];
</script>

<div id="bar-container">
  <div id="header">
    <div id="top-text">
      <h1>Projects</h1>
    </div>
  </div>
  <hr class="divider" />

  <div id="projects">
    {#each projects as project}
      <div class="project-container">
        <img class="project-thumbnail" src={project.src} />
        <h3 class="project-name">{project.name}</h3>
        <p class="project-dimensions">{project.dimensions}</p>
      </div>
    {/each}
  </div>
</div>

<style>
  #bar-container {
    display: flex;
    flex-direction: column;
    width: 100%;
  }
  #header {
    display: flex;
    justify-content: space-between;
  }
  #top-text {
    display: flex;
  }
  #projects {
    display: flex;
    align-items: center;
    width: 100%;
    justify-content: space-between;
  }
  .project-container {
    display: flex;
    flex-direction: column;
    max-width: 30%;
    justify-content: center;
  }
  .project-container img {
    width: 100%;
    height: auto;
    aspect-ratio: 3/2;
    object-fit: cover;
    border: 1px solid var(--light-gray);
    border-radius: 2rem;
  }
  .project-name {
    font-size: 1.2rem;
    font-weight: 400;
    margin-top: 0.6rem;
    margin-left: 0.3rem;
  }
  .project-dimensions {
    font-size: 1rem;
    font-weight: 400;
    color: var(--white-again);
    margin-top: 0.2rem;
    margin-left: 0.3rem;
  }
  .divider {
    width: calc(100%);
    background-color: var(--light-gray);
    border: none;
    margin: 0.7rem 0 1.3rem 0;
    height: 2px;
  }
</style>
