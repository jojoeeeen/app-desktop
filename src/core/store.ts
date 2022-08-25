import { Store } from 'tauri-plugin-store-api';

// ここ本当は window を any にしないで window 型をどこかで上書きしたほうが良い
const isTauri = !!(window as any).__TAURI__

// もし tauri を使わずに serve したとき用のモック
class StoreMock {
  async set(key: string, value: unknown): Promise<void> {
    return;
  }
  async get<T>(key: string): Promise<T | null> {
    return null;
  }
  async load(): Promise<void> {
    return;
  }
  async save(): Promise<void> {
    return;
  }
}

export const store = isTauri ? new Store('.settings') : new StoreMock();
