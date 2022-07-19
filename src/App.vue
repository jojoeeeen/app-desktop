<template>
  <button @click="startReadAloud">読み上げ開始/終了</button>
  <button @click="execSampleFn">Rust関数実行</button>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import { tauri } from "@tauri-apps/api";

@Options({
  components: {},
})
export default class App extends Vue {
  startReadAloud() {
    console.log("yomiage");
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
