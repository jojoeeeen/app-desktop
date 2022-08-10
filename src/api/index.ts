import { tauri } from "@tauri-apps/api";
import { VoiceQuery, WavBase64 } from "./types";

class ApiClient {
  async generate_query(speaker: number, text: string): Promise<VoiceQuery> {
    const ret: string = await tauri.invoke("generate_query", {speaker: speaker, text: text});
    return JSON.parse(ret);
  }

  async generate_voice(speaker: Number, query: VoiceQuery): Promise<WavBase64> {
    return await tauri.invoke("generate_voice", {speaker: speaker, query: query});
  }
}

export default new ApiClient();
