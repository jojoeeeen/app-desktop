interface Mora {
  text: string;
  consonant: string;
  consonant_length: number;
  vowel: string;
  vowel_length: number;
  pitch: number;
}

interface AccentPhrase {
  moras: Mora[];
  accent: number;
  pause_mora: Mora[]; 
  is_interrogative: Boolean;
}

export interface VoiceQuery {
  accent_phrases: AccentPhrase[];
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

export type WavBase64 = string;