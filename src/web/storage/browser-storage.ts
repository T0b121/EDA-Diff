/*
Browser implementation for small EDA-Diff preferences.

This adapter intentionally uses localStorage only for compact, non-sensitive
settings. Project files, repositories, and large caches must use IndexedDB/OPFS.
*/

import type { AppStorage, KeyValueStore, UserPreference } from "./storage";

class LocalPreferenceStore implements KeyValueStore<UserPreference> {
  private readonly prefix = "eda-diff:preference:";

  async get(key: string): Promise<UserPreference | undefined> {
    const value = localStorage.getItem(this.prefix + key);
    return value === null ? undefined : JSON.parse(value) as UserPreference;
  }

  async set(key: string, value: UserPreference): Promise<void> {
    localStorage.setItem(this.prefix + key, JSON.stringify(value));
  }

  async remove(key: string): Promise<void> {
    localStorage.removeItem(this.prefix + key);
  }
}

export function createBrowserStorage(): AppStorage {
  return {
    preferences: new LocalPreferenceStore()
  };
}
