<script>
  import SizeOptions from "./SizeOptions.svelte";

  // TODO: use actual data
  let username = "Alex";
</script>

<div id="bar-container">
  <div id="header">
    <div id="top-text">
      <h1>Welcome to Skru,</h1>
      <h1>&nbsp;{username}.</h1>
    </div>
    <button id="popular-templates">Popular Templates</button>
  </div>
  <hr class="divider" />
  <SizeOptions />
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
  #popular-templates {
    border: none;
    outline: none;
    background-color: transparent;
    color: var(--disabled-text);
    font-family: inherit;
    font-size: 1.2rem;
    font-weight: 600;
    margin-right: 0.4rem;
  }
  .divider {
    width: calc(100%);
    background-color: var(--light-gray);
    border: none;
    margin: 0.7rem 0 1.3rem 0;
    height: 2px;
  }
</style>
