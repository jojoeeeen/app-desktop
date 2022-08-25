<template>
  <div>
    <div>
      <table>
        <thead>
          <tr>
            <th>Mute words</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in internalItems" :key="item">
            <span>{{ item }}</span>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { MuteWord } from '@/types/mute-words'

const props = defineProps({
  items: { type: Array},
})

const internalItems = computed(() => {
  if (!props.items) {
    return []
  }
  // XXX: 本当は as ではなく検査したうえでちゃんと型を付けたほうがいい
  return props.items as MuteWord[];
})
</script>
