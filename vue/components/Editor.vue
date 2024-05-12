<script setup lang="ts">
  import init from '/dist/rust_scripter.js';

  init('/dist/rust_scripter_bg.wasm');
</script>

<template>
  <canvas id="the_canvas_id"></canvas>
</template>

<style scoped>

#the_canvas_id {
  width: 100% !important;
  height: 100% !important;
}

</style>
