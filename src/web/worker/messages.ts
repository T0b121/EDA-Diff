/*
Message contract between the browser UI and the EDA core Web Worker.

Keep worker requests and responses serializable. Domain-specific payloads can be
added here later without coupling the UI directly to Rust/WASM implementation.
*/

export type CoreRequest =
  | { id: number; type: "status" };

export type CoreResponse =
  | { id: number; type: "status"; status: string }
  | { id: number; type: "error"; message: string };
