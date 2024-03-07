<script lang="js">
  import ResizablePane from "../ResizablePane.svelte";
  import iro from '@jaames/iro';
  import { onMount } from "svelte";

  export var hex = "#ff0000"

  onMount(async () => {
    // @ts-ignore
    var colorPicker = new iro.ColorPicker("#picker", {
    // Set the size of the color picker
    width: 200,
    // Set the initial color to pure red
    color: "#f00"
  });
  // @ts-ignore
  colorPicker.on('color:change', function(color) {
     hex = colorPicker.color.hexString;
  });
  });
</script>

<ResizablePane side="right">
  <p>Color Picker</p>
  <div id="picker"></div>
  {#key hex}
  <p>Hex: {hex}</p>
  {/key}
  
</ResizablePane>

<style>
  p {
    padding-top:10px;
    text-align:center;
    font-size:25px
  }
  #picker {
    padding-left: 20px;
    padding-right: 20px;
    padding-top: 10px
  }
</style>