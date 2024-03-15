<script>
  export let style = "hollow";
</script>

<button on:click class={style === "full" ? "full" : "hollow"}>
  <slot />
</button>

<style>
  button {
    border-radius: 2rem;
    cursor: pointer;
    width: fit-content;
    font-family: inherit;
    font-weight: 700;
  }

  .full {
    background: var(--white);
    color: var(--black);
    border: none;
  }

  .hollow {
    background: transparent;
    color: var(--white);
    border: 2px solid var(--accent);
    transition: 0.3s ease all;
  }
  .hollow:hover {
    background: var(--accent);
    color: var(--black);
  }
</style>
