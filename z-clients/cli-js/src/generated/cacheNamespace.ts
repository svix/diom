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
export function registerCacheNamespaceCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `create <body>`,
    `Create cache namespace`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "name": "...",
  "eviction_policy": "..."
}`,
          "",
          `Example response:
{
  "name": "...",
  "eviction_policy": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const cacheCreateNamespaceIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.cache.namespace.create(
        cacheCreateNamespaceIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `get <body>`,
    `Get cache namespace`,
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
  "eviction_policy": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const cacheGetNamespaceIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.cache.namespace.get(
        cacheGetNamespaceIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}