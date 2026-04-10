// @ts-nocheck
// this file is @generated
import { registerKvNamespaceCommands } from "./kvNamespace.js";
import type { Argv } from "yargs";
import type { IoContext } from "../io.js";
import { getCliDiom } from "../diom-holder.js";
import { parseByteString } from "../byte-string.js";
import { parseJsonArg } from "../json-arg.js";
import { printJsonOutput } from "../print-json.js";

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
  "namespace": "...",
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
      const client = getCliDiom(io);
      
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
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.kv.set(
        key,
        value,
        kvSetIn,
      );
      printJsonOutput(resp);
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
  "namespace": "...",
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
      const client = getCliDiom(io);
      
      const key = String(
        argv["key"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const kvGetIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.kv.get(
        key,
        kvGetIn,
      );
      printJsonOutput(resp);
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
  "namespace": "...",
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
      const client = getCliDiom(io);
      
      const key = String(
        argv["key"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const kvDeleteIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.kv.delete(
        key,
        kvDeleteIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}