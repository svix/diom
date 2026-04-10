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
export function registerIdempotencyNamespaceCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `create <body>`,
    `Create idempotency namespace`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "name": "..."
}`,
          "",
          `Example response:
{
  "name": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const idempotencyCreateNamespaceIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.idempotency.namespace.create(
        idempotencyCreateNamespaceIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `get <body>`,
    `Get idempotency namespace`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "name": "..."
}`,
          "",
          `Example response:
{
  "name": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const idempotencyGetNamespaceIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.idempotency.namespace.get(
        idempotencyGetNamespaceIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}