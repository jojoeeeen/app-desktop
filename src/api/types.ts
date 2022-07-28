export interface VoiceQuery {
  accent_phrases: any; //後でちゃんと書く
  speedScale: number;
  pitchScale: number;
  intonationScale: number;
  volumeScale:number;
  prePhonemeLength: number;
  postPhonemeLength: number;
  outputSamplingRate: number;
  outputStereo: boolean;
  kana: string;
}
