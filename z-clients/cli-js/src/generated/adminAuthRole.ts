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
export function registerAdminAuthRoleCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `upsert <body>`,
    `Create or update a role`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "id": "...",
  "description": "...",
  "rules": "...",
  "policies": "...",
  "context": "..."
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
      
      
      const adminRoleUpsertIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authRole.upsert(
        adminRoleUpsertIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `delete <body>`,
    `Delete a role`,
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
      
      
      const adminRoleDeleteIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authRole.delete(
        adminRoleDeleteIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `get <body>`,
    `Get a role by ID`,
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
  "policies": "...",
  "context": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const adminRoleGetIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authRole.get(
        adminRoleGetIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `list [body]`,
    `List all roles`,
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
      const adminRoleListIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.admin.authRole.list(
        adminRoleListIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}