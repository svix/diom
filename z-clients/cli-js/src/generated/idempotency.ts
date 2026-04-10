// @ts-nocheck
// this file is @generated
import { registerIdempotencyNamespaceCommands } from "./idempotencyNamespace.js";
import type { Argv } from "yargs";
import type { IoContext } from "../io.js";
import { getCliDiom } from "../diom-holder.js";
import { parseByteString } from "../byte-string.js";
import { parseJsonArg } from "../json-arg.js";
import { printJsonOutput } from "../print-json.js";

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
  "namespace": "...",
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
      const client = getCliDiom(io);
      
      const key = String(
        argv["key"],
      );
      
      
      
      const idempotencyStartIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.idempotency.start(
        key,
        idempotencyStartIn,
      );
      printJsonOutput(resp);
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
  "namespace": "...",
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
      const client = getCliDiom(io);
      
      const key = String(
        argv["key"],
      );
      
      
      
      const idempotencyCompleteIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.idempotency.complete(
        key,
        idempotencyCompleteIn,
      );
      printJsonOutput(resp);
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
  "namespace": "..."
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
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const idempotencyAbortIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.idempotency.abort(
        key,
        idempotencyAbortIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}