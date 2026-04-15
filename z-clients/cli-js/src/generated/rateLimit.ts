// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { RateLimitCheckInSerializer, RateLimitCheckOutSerializer, RateLimitGetRemainingInSerializer, RateLimitGetRemainingOutSerializer, RateLimitResetInSerializer, RateLimitResetOutSerializer } from "@diomhq/diom/internal";
import { registerRateLimitNamespaceCommands } from "./rateLimitNamespace.ts";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerRateLimitCommands(
  y: Argv,
  io: IoContext,
): Argv {
  y.command(
    "namespace",
    "",
    (y2) => {
      registerRateLimitNamespaceCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  

  
  
  y.command(
    `limit <body>`,
    `Rate Limiter Check and Consume`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "key": "some_key",
  "tokens": "...",
  "config": "..."
}`,
          "",
          `Example response:
{
  "allowed": "...",
  "remaining": "...",
  "retry_after_ms": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      
      const rateLimitCheckIn = RateLimitCheckInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.rateLimit.limit(
        rateLimitCheckIn,
      );
      printWireJson(RateLimitCheckOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `get-remaining <body>`,
    `Rate Limiter Get Remaining`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "key": "some_key",
  "config": "..."
}`,
          "",
          `Example response:
{
  "remaining": "...",
  "retry_after_ms": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      
      const rateLimitGetRemainingIn = RateLimitGetRemainingInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.rateLimit.getRemaining(
        rateLimitGetRemainingIn,
      );
      printWireJson(RateLimitGetRemainingOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `reset <body>`,
    `Rate Limiter Reset`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "key": "some_key",
  "config": "..."
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
      
      
      const rateLimitResetIn = RateLimitResetInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.rateLimit.reset(
        rateLimitResetIn,
      );
      printWireJson(RateLimitResetOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}