<script setup lang="ts">
import { VueComponent as ContentMarkdown } from '../../README.md'
</script>

<template>
  <div class="text">
    <ContentMarkdown />
  </div>
</template>

<style scoped>
.text {
  max-width: 750px;
  text-align: justify;
  margin: auto;
}
</style>
