// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { KvConfigureNamespaceInSerializer, KvConfigureNamespaceOutSerializer, KvGetNamespaceInSerializer, KvGetNamespaceOutSerializer } from "@diomhq/diom";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerKvNamespaceCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `configure <body>`,
    `Configure KV namespace`,
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
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      
      const kvConfigureNamespaceIn = KvConfigureNamespaceInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.kv.namespace.configure(
        kvConfigureNamespaceIn,
      );
      printWireJson(KvConfigureNamespaceOutSerializer._toJsonObject(resp));
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
  "name": "some_namespace"
}`,
          "",
          `Example response:
{
  "name": "some_namespace",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      
      const kvGetNamespaceIn = KvGetNamespaceInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.kv.namespace.get(
        kvGetNamespaceIn,
      );
      printWireJson(KvGetNamespaceOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}