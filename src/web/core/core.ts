/*
Browser bridge to the generated Rust/WebAssembly EDA core.

Only this module knows the wasm-bindgen output path. Other browser modules should
call this wrapper instead of importing generated WebAssembly bindings directly.
*/

import initCore, { core_status } from "../generated/core/eda_diff_core.js";

let initialization: Promise<void> | undefined;

export function initializeCore(): Promise<void> {
  initialization ??= initCore().then(() => undefined);
  return initialization;
}

export async function getCoreStatus(): Promise<string> {
  await initializeCore();
  return core_status();
}
