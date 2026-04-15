// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { CacheConfigureNamespaceInSerializer, CacheConfigureNamespaceOutSerializer, CacheGetNamespaceInSerializer, CacheGetNamespaceOutSerializer } from "@diomhq/diom/internal";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerCacheNamespaceCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `configure <body>`,
    `Configure cache namespace`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "name": "some_namespace",
  "eviction_policy": "..."
}`,
          "",
          `Example response:
{
  "name": "some_namespace",
  "eviction_policy": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      
      const cacheConfigureNamespaceIn = CacheConfigureNamespaceInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.cache.namespace.configure(
        cacheConfigureNamespaceIn,
      );
      printWireJson(CacheConfigureNamespaceOutSerializer._toJsonObject(resp));
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
  "name": "some_namespace"
}`,
          "",
          `Example response:
{
  "name": "some_namespace",
  "eviction_policy": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      
      const cacheGetNamespaceIn = CacheGetNamespaceInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.cache.namespace.get(
        cacheGetNamespaceIn,
      );
      printWireJson(CacheGetNamespaceOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}