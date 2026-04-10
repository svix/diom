// @ts-nocheck
// this file is @generated
import { registerRateLimitNamespaceCommands } from "./rateLimitNamespace.js";
import type { Argv } from "yargs";
import type { IoContext } from "../io.js";
import { getCliDiom } from "../diom-holder.js";
import { parseByteString } from "../byte-string.js";
import { parseJsonArg } from "../json-arg.js";
import { printJsonOutput } from "../print-json.js";

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
  "namespace": "...",
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
      const client = getCliDiom(io);
      
      
      const rateLimitCheckIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.rateLimit.limit(
        rateLimitCheckIn,
      );
      printJsonOutput(resp);
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
  "namespace": "...",
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
      const client = getCliDiom(io);
      
      
      const rateLimitGetRemainingIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.rateLimit.getRemaining(
        rateLimitGetRemainingIn,
      );
      printJsonOutput(resp);
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
  "namespace": "...",
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
      const client = getCliDiom(io);
      
      
      const rateLimitResetIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.rateLimit.reset(
        rateLimitResetIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}