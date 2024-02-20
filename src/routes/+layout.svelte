<div id="page-container">
  <slot />
</div>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Inter:wght@500;600;700;900&display=swap");

  /* container styles */
  #page-container {
    height: 100vh;
    width: 100vw;
  }

  /* global styles */
  :root {
    --white: #ffffff;
    --gray: #1d1d1d;
    --black: #131313;
    --font-family: "Inter";
    font-size: 100%; /* 1rem = 16px */
  }

  /* css reset */
  :global(body) {
    font-family: var(--font-family);
    background-color: var(--black);
    color: var(--white);
  }
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }
  :global(a) {
    text-decoration: none;
    color: inherit;
  }
</style>
