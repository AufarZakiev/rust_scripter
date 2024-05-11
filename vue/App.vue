<script setup lang="ts">
import Editor from './components/Editor.vue'

import init from '/dist/rust_scripter.js';
init('/dist/rust_scripter_bg.wasm');
    
</script>

<template>
  <Editor/>
</template>

<style scoped>

</style>
