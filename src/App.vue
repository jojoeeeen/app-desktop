<template>
  <x-button v-if="!read" @click="startReadAloud">読み上げ開始</x-button>
  <x-button v-else @click="stopReadAloud">読み上げ停止</x-button>
  <x-button @click="execSampleFn">Rust関数実行</x-button>
  <x-button @click="() => readChatAloud('生成テスト')">
    VoiceQuery生成テスト
  </x-button>

  <suspense>
    <template #default>
      <settings-base />
    </template>
    <template #fallback>
      <p>Loading...</p>
    </template>
  </suspense>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { tauri } from "@tauri-apps/api";
import client from "./api";

import type { VoiceQuery, WavBase64 } from "./api/types";

import XButton from '@/components/XButton';
import SettingsBase from '@/components/SettingsBase.vue';

const read = ref(false);

const startReadAloud = async ()  => {
  const _sleep = (ms: number) =>
    new Promise((resolve) => setTimeout(resolve, ms));

  read.value = true;
  while (read.value) {
    // TODO: パフォーマンスが悪ければ別ループに切り出して chats をキューする
    const chats = getChats();

      for (let i = 0; i < chats.length; i++) {
        await readChatAloud(chats[i]);
      }

    await _sleep(30000);
  }
}

const stopReadAloud = () => {
  read.value = false;
}

const getChats = ():string[] => {
  // tauri.invoke("get_chats");

  return ["草", "ワロタ", "ここすこ"];
}

const readChatAloud = async (chat: string) => {
    const speaker = 1;
    const voiceQuery: VoiceQuery = await client.generate_query(speaker, chat);
    const voice: WavBase64 = await client.generate_voice(speaker, voiceQuery);

    const play = () => {
      return new Promise((resolve) => {
        const audio = new Audio("data:audio/wav;base64," + voice);
        audio.onended = resolve;
        audio.play();
      });
    };

    await play();
}

const execSampleFn = async () => {
  let ret = await tauri.invoke("sample_fn");
  console.log("sample_fn result:", ret);
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
