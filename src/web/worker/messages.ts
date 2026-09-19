/*
Message contract between the browser UI and the EDA core Web Worker.

Keep requests and responses serializable. File bytes are transferred to the
worker so parsing does not copy large EDA files on the UI thread.
*/

export type CoreRequest =
  | { id: number; type: "status" }
  | { id: number; type: "parse-kicad-pcb"; path: string; bytes: ArrayBuffer }
  | { id: number; type: "parse-kicad-schematic"; path: string; bytes: ArrayBuffer };

export type CoreResponse =
  | { id: number; type: "status"; status: string }
  | { id: number; type: "pcb"; json: string }
  | { id: number; type: "schematic"; json: string }
  | { id: number; type: "error"; message: string };
