// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { printWireJson } from "../print-json.ts";
import { PingOutSerializer } from "@diomhq/diom/internal";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerHealthCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  y.command(
    "ping",
    `Verify the server is up and running.`,
    (cmdY) => {
      cmdY.epilog(
        `Example response:
{
  "ok": "..."
}`,
      );
      return cmdY;
    },
    async (_argv) => {
      const client = io.diom;
      const resp = await client.health.ping();
      printWireJson(PingOutSerializer._toJsonObject(resp));
    },
  );
  
  
  y.command(
    "error",
    `Intentionally return an error`,
    (cmdY) => {
      return cmdY;
    },
    async (_argv) => {
      const client = io.diom;
      await client.health.error();
    },
  );
  
  

  return y.demandCommand(1).strict();
}