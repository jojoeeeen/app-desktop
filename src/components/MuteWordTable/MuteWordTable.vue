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
          <tr v-for="item in items" :key="item">
            <span>{{ item }}</span>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  items: { type: Array },
})
</script>
