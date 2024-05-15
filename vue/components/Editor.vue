<script setup lang="ts">
  //@ts-ignore
  import init from '/dist/rust/rust_scripter.js';

  //@ts-ignore
  init(`${import.meta.env.BASE_URL}/dist/rust/rust_scripter_bg.wasm`);
</script>

<template>
  <canvas id="the_canvas_id"></canvas>
</template>

<style scoped>

</style>
