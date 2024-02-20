<script lang="ts">
  export let type = "full";
  export let fontSize = "3.4rem";

  const originalFontSize = 3.4; // Base font size for the original button
  const paddingVertical = 0.5 / originalFontSize; // Padding-top and padding-bottom (as a fraction of the font size)
  const paddingHorizontal = 2 / originalFontSize; // Padding-left and padding-right (as a fraction of the font size)
  const marginTopScale = 3.7 / originalFontSize; // Margin-top (as a fraction of the font size)

  const fontSizeValue = parseFloat(fontSize);
  const scaledPaddingVertical = paddingVertical * fontSizeValue + "rem";
  const scaledPaddingHorizontal = paddingHorizontal * fontSizeValue + "rem";
  const scaledMarginTop = marginTopScale * fontSizeValue + "rem";
</script>

<button
  on:click
  class={type === "full" ? "full" : "hollow"}
  style={`font-size: ${fontSize}; padding: ${scaledPaddingVertical} ${scaledPaddingHorizontal}; margin-top: ${scaledMarginTop};`}
>
  <slot />
</button>

<style>
  button {
    border-radius: 10rem;
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
    border: 2px solid var(--white);
  }
</style>
