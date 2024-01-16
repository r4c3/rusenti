<div id="page-container">
  <slot/>
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@500;600;700;900&display=swap');
  
  /* global styles */
  :root {
    --white: #ECECEC;
    --gray: #24242D;
    --black: #141518;
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
