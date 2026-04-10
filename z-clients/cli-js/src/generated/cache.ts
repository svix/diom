// @ts-nocheck
// this file is @generated
import { registerCacheNamespaceCommands } from "./cacheNamespace.js";
import type { Argv } from "yargs";
import type { IoContext } from "../io.js";
import { getCliDiom } from "../diom-holder.js";
import { parseByteString } from "../byte-string.js";
import { parseJsonArg } from "../json-arg.js";
import { printJsonOutput } from "../print-json.js";

/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerCacheCommands(
  y: Argv,
  io: IoContext,
): Argv {
  y.command(
    "namespace",
    "",
    (y2) => {
      registerCacheNamespaceCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  

  
  
  y.command(
    `set <key> <value> <body>`,
    `Cache Set`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "...",
  "ttl_ms": "..."
}`,
          "",
          `Example response:
{
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      const key = String(
        argv["key"],
      );
      
      
      const value = parseByteString(
        String(argv["value"]),
      );
      
      
      
      const cacheSetIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.cache.set(
        key,
        value,
        cacheSetIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `get <key> [body]`,
    `Cache Get`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "...",
  "consistency": "..."
}`,
          "",
          `Example response:
{
  "expiry": "...",
  "value": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      const key = String(
        argv["key"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const cacheGetIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.cache.get(
        key,
        cacheGetIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `delete <key> [body]`,
    `Cache Delete`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "..."
}`,
          "",
          `Example response:
{
  "success": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      const key = String(
        argv["key"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const cacheDeleteIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.cache.delete(
        key,
        cacheDeleteIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}