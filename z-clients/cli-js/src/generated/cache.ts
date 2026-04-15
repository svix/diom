// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import { parseByteString } from "../byte-string.ts";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { CacheSetInSerializer, CacheSetOutSerializer, CacheGetInSerializer, CacheGetOutSerializer, CacheDeleteInSerializer, CacheDeleteOutSerializer } from "@diomhq/diom";
import { registerCacheNamespaceCommands } from "./cacheNamespace.ts";


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
  "namespace": "some_namespace",
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
      const client = io.diom;
      
      const key = String(
        argv["key"],
      );
      
      
      const value = parseByteString(
        String(argv["value"]),
      );
      
      
      
      const cacheSetIn = CacheSetInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.cache.set(
        key,
        value,
        cacheSetIn,
      );
      printWireJson(CacheSetOutSerializer._toJsonObject(resp));
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
  "namespace": "some_namespace",
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
      const client = io.diom;
      
      const key = String(
        argv["key"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const cacheGetIn =
        bodyRaw === undefined
          ? {}
          : CacheGetInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.cache.get(
        key,
        cacheGetIn,
      );
      printWireJson(CacheGetOutSerializer._toJsonObject(resp));
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
  "namespace": "some_namespace"
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
      const client = io.diom;
      
      const key = String(
        argv["key"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const cacheDeleteIn =
        bodyRaw === undefined
          ? {}
          : CacheDeleteInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.cache.delete(
        key,
        cacheDeleteIn,
      );
      printWireJson(CacheDeleteOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}