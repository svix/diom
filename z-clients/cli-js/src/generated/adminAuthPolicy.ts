// @ts-nocheck
// this file is @generated
import type { Argv } from "yargs";
import type { IoContext } from "../io.js";
import { getCliDiom } from "../diom-holder.js";
import { parseByteString } from "../byte-string.js";
import { parseJsonArg } from "../json-arg.js";
import { printJsonOutput } from "../print-json.js";

/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerAdminAuthPolicyCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `upsert <body>`,
    `Create or update an access policy`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "id": "...",
  "description": "...",
  "rules": "..."
}`,
          "",
          `Example response:
{
  "id": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const adminAccessPolicyUpsertIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authPolicy.upsert(
        adminAccessPolicyUpsertIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `delete <body>`,
    `Delete an access policy`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "id": "..."
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
      
      
      const adminAccessPolicyDeleteIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authPolicy.delete(
        adminAccessPolicyDeleteIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `get <body>`,
    `Get an access policy by ID`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "id": "..."
}`,
          "",
          `Example response:
{
  "id": "...",
  "description": "...",
  "rules": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const adminAccessPolicyGetIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authPolicy.get(
        adminAccessPolicyGetIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `list [body]`,
    `List all access policies`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "limit": "...",
  "iterator": "..."
}`,
          "",
          `Example response:
{
  "data": "...",
  "iterator": "...",
  "prev_iterator": "...",
  "done": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const bodyRaw = argv.body as string | undefined;
      const adminAccessPolicyListIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.admin.authPolicy.list(
        adminAccessPolicyListIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}