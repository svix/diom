import type { Diom } from "../../javascript/src/client.js";
import type { IoContext } from "./io.js";

/** Populated by `runCli` middleware before nested command handlers run. */
export function getCliDiom(io: IoContext): Diom {
  if (!io._diom) {
    throw new Error("internal: Diom client not initialized");
  }
  return io._diom;
}
