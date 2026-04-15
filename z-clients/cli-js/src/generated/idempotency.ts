// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { IdempotencyStartInSerializer, IdempotencyStartOutSerializer, IdempotencyCompleteInSerializer, IdempotencyCompleteOutSerializer, IdempotencyAbortInSerializer, IdempotencyAbortOutSerializer } from "@diomhq/diom/internal";
import { registerIdempotencyNamespaceCommands } from "./idempotencyNamespace.ts";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerIdempotencyCommands(
  y: Argv,
  io: IoContext,
): Argv {
  y.command(
    "namespace",
    "",
    (y2) => {
      registerIdempotencyNamespaceCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  

  
  
  y.command(
    `start <key> <body>`,
    `Start an idempotent request`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "lock_period_ms": "..."
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
      
      
      
      const idempotencyStartIn = IdempotencyStartInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.idempotency.start(
        key,
        idempotencyStartIn,
      );
      printWireJson(IdempotencyStartOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `complete <key> <body>`,
    `Complete an idempotent request with a response`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "response": "...",
  "context": "...",
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
      
      
      
      const idempotencyCompleteIn = IdempotencyCompleteInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.idempotency.complete(
        key,
        idempotencyCompleteIn,
      );
      printWireJson(IdempotencyCompleteOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `abort <key> [body]`,
    `Abandon an idempotent request (remove lock without saving response)`,
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
      const idempotencyAbortIn =
        bodyRaw === undefined
          ? {}
          : IdempotencyAbortInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.idempotency.abort(
        key,
        idempotencyAbortIn,
      );
      printWireJson(IdempotencyAbortOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}