<script setup lang="ts">
import { ref } from 'vue'

import Editor from './components/Editor.vue'
import Info from './components/Info.vue';

const tab = ref('info');
    
</script>

<template>
  <div style="height: 100%; display: flex; flex-direction: column;">
    <q-tabs
      v-model="tab"
      class="text-teal"
    >
      <q-tab name="info" label="what is it" />
      <q-tab name="demo" label="demo" />
    </q-tabs>

    <q-tab-panels
      v-model="tab"
      animated
      transition-prev="jump-up"
      transition-next="jump-up"
      keep-alive
      style="flex-grow: 1;"
    >
      <q-tab-panel name="info">
        <Info/>
      </q-tab-panel>

      <q-tab-panel name="demo" class="q-pa-none" style="overflow: hidden;">
        <Editor/> 
      </q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<style scoped>

</style>
