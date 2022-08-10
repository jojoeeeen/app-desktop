<template>
  <x-button v-if="!read" @click="startReadAloud">読み上げ開始</x-button>
  <x-button v-else @click="stopReadAloud">読み上げ停止</x-button>
  <x-button @click="execSampleFn">Rust関数実行</x-button>
  <x-button @click="() => readChatAloud('生成テスト')">
    VoiceQuery生成テスト
  </x-button>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import { tauri } from "@tauri-apps/api";
import client from "./api";

import type { VoiceQuery, WavBase64 } from "./api/types";

import XButton from "./components/XButton";

@Options({
  components: {
    XButton,
  },
})
export default class App extends Vue {
  read = false;
  async startReadAloud() {
    const _sleep = (ms: number) =>
      new Promise((resolve) => setTimeout(resolve, ms));

    this.read = true;
    while (this.read) {
      // TODO: パフォーマンスが悪ければ別ループに切り出して chats をキューする
      const chats = this.getChats();

      for (let i = 0; i < chats.length; i++) {
        await this.readChatAloud(chats[i]);
      }

      await _sleep(30000);
    }
  }

  stopReadAloud() {
    this.read = false;
  }

  getChats(): string[] {
    // tauri.invoke("get_chats");

    return ["草", "ワロタ", "ここすこ"];
  }

  async readChatAloud(chat: string) {
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

  async execSampleFn() {
    let ret = await tauri.invoke("sample_fn");
    console.log("sample_fn result:", ret);
  }
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
