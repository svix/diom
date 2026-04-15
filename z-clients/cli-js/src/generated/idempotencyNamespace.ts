// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { IdempotencyConfigureNamespaceInSerializer, IdempotencyConfigureNamespaceOutSerializer, IdempotencyGetNamespaceInSerializer, IdempotencyGetNamespaceOutSerializer } from "@diomhq/diom/internal";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerIdempotencyNamespaceCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `configure <body>`,
    `Configure idempotency namespace`,
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
      
      
      const idempotencyConfigureNamespaceIn = IdempotencyConfigureNamespaceInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.idempotency.namespace.configure(
        idempotencyConfigureNamespaceIn,
      );
      printWireJson(IdempotencyConfigureNamespaceOutSerializer._toJsonObject(resp));
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
      
      
      const idempotencyGetNamespaceIn = IdempotencyGetNamespaceInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.idempotency.namespace.get(
        idempotencyGetNamespaceIn,
      );
      printWireJson(IdempotencyGetNamespaceOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}