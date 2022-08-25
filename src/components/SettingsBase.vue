<template>
  <mute-word-table :items="items" />

  <x-button @click="restoreSampleData">
    <span>ミュートワード: サンプルデータを流し込み</span>
  </x-button>
  <x-button @click="clearMuteWords">
    <span>ミュートワード: データをすべてクリア</span>
  </x-button>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { store } from '@/core/store';
import XButton from '@/components/XButton'
import { MuteWords } from '@/types/mute-words'

import MuteWordTable from './MuteWordTable';

const useStoreMock = process.env.VUE_APP_USE_STORE_MOCK;
const items = ref(['']);
items.value = await loadItems();

async function loadItems(): Promise<string[]> {
  await store.load();
  const v = await store.get<MuteWords>('mute-words');

  if (useStoreMock) {
    return ['this is mocked value', 'hello', 'world', '!'];
  }
  if(!v) {
    throw new Error('mute-words is null.')
  }
  return v.items;
}

const restoreSampleData = async (): Promise<void> => {
  await store.set('mute-words', { items: ['hello', 'world', '!']});
  await store.save();

  const v = await store.get<MuteWords>('mute-words');
  if(!v?.items) {
    throw new Error('no mute-words');
  }
  items.value = v.items;
}

const clearMuteWords = async (): Promise<void> => {
  await store.set('mute-words', { items: []});
  await store.save();

  const v = await store.get<MuteWords>('mute-words');
  if(!v?.items) {
    throw new Error('no mute-words');
  }
  items.value = v.items;
}
</script>
