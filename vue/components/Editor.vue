<script setup lang="ts">
  import { computed, onMounted, ref } from 'vue';
  import init from '/dist/rust/rust_scripter.js';

  const isReady = ref(false);
  const rustAdder = ref(null as null | Function)

  onMounted(async () => {
    const rustModule = await init();
    isReady.value = true;
    rustAdder.value = rustModule.add;
  })

  const a = ref(2);
  const b = ref(2);
  const res = computed(() => {
    if (!rustAdder.value) return 'Failed';

    return rustAdder.value(a.value, b.value)
  })
</script>

<template>
  <div class="calc">
    <span> Try Rusty adder: </span>
    <q-input v-model="a" inputmode="numeric" dense input-style="text-align: center"/>
    <span> + </span>
    <q-input v-model="b" inputmode="numeric" dense input-style="text-align: center" />
    <span> = </span>
    <q-input v-model="res" readonly dense filled input-style="text-align: center"/>
    <q-icon name="info">
      <q-tooltip>
        This value was retrieved from Rust code
      </q-tooltip>
    </q-icon>
  </div>

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

.calc {
  display: flex;
  justify-content: center;
  gap: 10px;
  padding-bottom: 5px;
}

.calc span, i {
  margin-top: auto;
  margin-bottom: auto;
}

.calc .q-field {
  width: 40px;
}
</style>
