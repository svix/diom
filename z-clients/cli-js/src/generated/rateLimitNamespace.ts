// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { RateLimitConfigureNamespaceInSerializer, RateLimitConfigureNamespaceOutSerializer, RateLimitGetNamespaceInSerializer, RateLimitGetNamespaceOutSerializer } from "@diomhq/diom/internal";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerRateLimitNamespaceCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `configure <body>`,
    `Configure rate limiter namespace`,
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
      
      
      const rateLimitConfigureNamespaceIn = RateLimitConfigureNamespaceInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.rateLimit.namespace.configure(
        rateLimitConfigureNamespaceIn,
      );
      printWireJson(RateLimitConfigureNamespaceOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `get <body>`,
    `Get rate limiter namespace`,
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
      
      
      const rateLimitGetNamespaceIn = RateLimitGetNamespaceInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.rateLimit.namespace.get(
        rateLimitGetNamespaceIn,
      );
      printWireJson(RateLimitGetNamespaceOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}