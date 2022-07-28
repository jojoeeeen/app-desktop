import { tauri } from "@tauri-apps/api";
import { VoiceQuery } from "./types";

class ApiClient {
  async generate_query(speaker: number, text: string): Promise<VoiceQuery> {
    const ret: string = await tauri.invoke("generate_query", {speaker: speaker, text: text});
    return JSON.parse(ret);
  }
}

export default new ApiClient();
