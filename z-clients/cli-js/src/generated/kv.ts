// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import { parseByteString } from "../byte-string.ts";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { KvSetInSerializer, KvSetOutSerializer, KvGetInSerializer, KvGetOutSerializer, KvDeleteInSerializer, KvDeleteOutSerializer } from "@diomhq/diom/internal";
import { registerKvNamespaceCommands } from "./kvNamespace.ts";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerKvCommands(
  y: Argv,
  io: IoContext,
): Argv {
  y.command(
    "namespace",
    "",
    (y2) => {
      registerKvNamespaceCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  

  
  
  y.command(
    `set <key> <value> [body]`,
    `KV Set`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "ttl_ms": "...",
  "behavior": "...",
  "version": "..."
}`,
          "",
          `Example response:
{
  "success": "...",
  "version": "..."
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
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const kvSetIn =
        bodyRaw === undefined
          ? {}
          : KvSetInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.kv.set(
        key,
        value,
        kvSetIn,
      );
      printWireJson(KvSetOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `get <key> [body]`,
    `KV Get`,
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
  "value": "...",
  "version": "..."
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
      const kvGetIn =
        bodyRaw === undefined
          ? {}
          : KvGetInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.kv.get(
        key,
        kvGetIn,
      );
      printWireJson(KvGetOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `delete <key> [body]`,
    `KV Delete`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "version": "..."
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
      const kvDeleteIn =
        bodyRaw === undefined
          ? {}
          : KvDeleteInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.kv.delete(
        key,
        kvDeleteIn,
      );
      printWireJson(KvDeleteOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}