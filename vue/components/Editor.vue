<script setup lang="ts">
  //@ts-ignore
  import init from '/dist/rust/rust_scripter.js';

  //@ts-ignore
  init();
</script>

<template>
  <canvas id="the_canvas_id"></canvas>

  <!-- the loading spinner will be removed in main.rs -->
  <div class="centered" id="loading_text">
    <p style="font-size:16px">
        Loading…
    </p>
    <div class="lds-dual-ring"></div>
  </div>
</template>

<style scoped>
#the_canvas_id {
  max-width: 100%;
  max-height: 100%;
  height: 99%;
  width: 100%;
}
</style>
