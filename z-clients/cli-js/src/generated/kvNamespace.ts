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
export function registerKvNamespaceCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `create <body>`,
    `Create KV namespace`,
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
      
      
      const kvCreateNamespaceIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.kv.namespace.create(
        kvCreateNamespaceIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `get <body>`,
    `Get KV namespace`,
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
      
      
      const kvGetNamespaceIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.kv.namespace.get(
        kvGetNamespaceIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}