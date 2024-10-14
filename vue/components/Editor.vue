<script setup lang="ts">
  import { onMounted, ref } from 'vue';
  import init from '/dist/rust/rust_scripter.js';

  const isReady = ref(false);

  onMounted(async () => {
    const t = await init();
    isReady.value = true;
    console.log(t);
  })
</script>

<template>
  <canvas id="the_canvas_id"/>

  <div v-if="!isReady" id="loading">
    <p style="">
        Loading…
    </p>
    <q-linear-progress indeterminate style="width: 800px;"/>
  </div>
</template>

<style scoped>
#the_canvas_id {
  max-width: 100%;
  max-height: 100%;
  height: 100%;
  width: 100%;
}

#loading {
  position: absolute; 
  top: 0; 
  left: 0;
  background-color: lightgray;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  font-size:24px
}
</style>
