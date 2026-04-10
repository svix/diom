// @ts-nocheck
// this file is @generated
import type { Argv } from "yargs";
import type { IoContext } from "../io.js";
import { getCliDiom } from "../diom-holder.js";
import { parseByteString } from "../byte-string.js";
import { parseJsonArg } from "../json-arg.js";
import { printJsonOutput } from "../print-json.js";

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
    async (argv) => {
      const client = getCliDiom(io);
      const resp = await client.health.ping();
      printJsonOutput(resp);
    },
  );
  
  
  y.command(
    "error",
    `Intentionally return an error`,
    (cmdY) => {
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      await client.health.error();
    },
  );
  
  

  return y.demandCommand(1).strict();
}