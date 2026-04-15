// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { AdminRoleConfigureInSerializer, AdminRoleConfigureOutSerializer, AdminRoleDeleteInSerializer, AdminRoleDeleteOutSerializer, AdminRoleGetInSerializer, AdminRoleOutSerializer, AdminRoleListInSerializer, ListResponseAdminRoleOutSerializer } from "@diomhq/diom/internal";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerAdminAuthRoleCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `configure <body>`,
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
      const client = io.diom;
      
      
      const adminRoleConfigureIn = AdminRoleConfigureInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authRole.configure(
        adminRoleConfigureIn,
      );
      printWireJson(AdminRoleConfigureOutSerializer._toJsonObject(resp));
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
      const client = io.diom;
      
      
      const adminRoleDeleteIn = AdminRoleDeleteInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authRole.delete(
        adminRoleDeleteIn,
      );
      printWireJson(AdminRoleDeleteOutSerializer._toJsonObject(resp));
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
      const client = io.diom;
      
      
      const adminRoleGetIn = AdminRoleGetInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authRole.get(
        adminRoleGetIn,
      );
      printWireJson(AdminRoleOutSerializer._toJsonObject(resp));
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
      const client = io.diom;
      
      
      const bodyRaw = argv.body as string | undefined;
      const adminRoleListIn =
        bodyRaw === undefined
          ? {}
          : AdminRoleListInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.admin.authRole.list(
        adminRoleListIn,
      );
      printWireJson(ListResponseAdminRoleOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}