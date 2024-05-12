<script setup lang="ts">
import { ref } from 'vue'

import Editor from './components/Editor.vue'

const tab = ref('editor');
const splitterModel = ref(20);
    
</script>

<template>
  <q-splitter
      v-model="splitterModel"
      style="height: 100%"
    >

      <template v-slot:before>
        <q-tabs
          v-model="tab"
          vertical
          class="text-teal"
        >
          <q-tab name="account" icon="mail" label="account" />
          <q-tab name="editor" icon="alarm" label="editor" />
        </q-tabs>
      </template>

      <template v-slot:after>
        <q-tab-panels
          v-model="tab"
          animated
          swipeable
          vertical
          transition-prev="jump-up"
          transition-next="jump-up"
        >
          <q-tab-panel name="account">
            <div class="text-h4 q-mb-md">My account</div>
          </q-tab-panel>

          <q-tab-panel name="editor" class="q-pa-none" style="height: 100vh;">
            <Editor/>
          </q-tab-panel>
        </q-tab-panels>
      </template>

    </q-splitter>
</template>

<style scoped>

</style>
