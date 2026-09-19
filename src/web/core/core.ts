/*
Browser bridge to the generated Rust/WebAssembly EDA core.

Only this module knows the wasm-bindgen output path. Other browser modules should
call this wrapper instead of importing generated WebAssembly bindings directly.
*/

import initCore, {
  core_status,
  diff_kicad_pcb_json,
  diff_kicad_schematic_json,
  parse_kicad_pcb_json,
  parse_kicad_schematic_json
} from "../generated/core/eda_diff_core.js";

let initialization: Promise<void> | undefined;

export function initializeCore(): Promise<void> {
  initialization ??= initCore().then(() => undefined);
  return initialization;
}

export async function getCoreStatus(): Promise<string> {
  await initializeCore();
  return core_status();
}

export async function parseKiCadPcb(source: string, path: string): Promise<string> {
  await initializeCore();
  return parse_kicad_pcb_json(source, path);
}

export async function parseKiCadSchematic(
  source: string,
  path: string
): Promise<string> {
  await initializeCore();
  return parse_kicad_schematic_json(source, path);
}

export async function diffKiCadPcb(
  beforeSource: string,
  afterSource: string,
  beforePath: string,
  afterPath: string
): Promise<string> {
  await initializeCore();
  return diff_kicad_pcb_json(
    beforeSource,
    afterSource,
    beforePath,
    afterPath
  );
}

export async function diffKiCadSchematic(
  beforeSource: string,
  afterSource: string,
  beforePath: string,
  afterPath: string
): Promise<string> {
  await initializeCore();
  return diff_kicad_schematic_json(
    beforeSource,
    afterSource,
    beforePath,
    afterPath
  );
}
