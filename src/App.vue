<template>
  <button v-if="!read" @click="startReadAloud">読み上げ開始</button>
  <button v-else @click="stopReadAloud">読み上げ停止</button>
  <button @click="execSampleFn">Rust関数実行</button>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import { tauri } from "@tauri-apps/api";

@Options({
  components: {},
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

      chats.forEach((chat) => this.readChatAloud(chat));

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

  readChatAloud(chat: string) {
    // tauri.invoke("read_chat_aloud");

    console.log(chat);
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
