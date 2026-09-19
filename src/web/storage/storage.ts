/*
Persistence contracts for browser-side EDA-Diff state.

Small preferences may use localStorage while project metadata and large working
data can later use IndexedDB or OPFS behind these abstractions.
*/

export interface KeyValueStore<T> {
  get(key: string): Promise<T | undefined>;
  set(key: string, value: T): Promise<void>;
  remove(key: string): Promise<void>;
}

export type UserPreference = string | number | boolean;

export interface AppStorage {
  preferences: KeyValueStore<UserPreference>;
}
