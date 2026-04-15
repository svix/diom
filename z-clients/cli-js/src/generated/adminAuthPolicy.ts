// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { AdminAccessPolicyConfigureInSerializer, AdminAccessPolicyConfigureOutSerializer, AdminAccessPolicyDeleteInSerializer, AdminAccessPolicyDeleteOutSerializer, AdminAccessPolicyGetInSerializer, AdminAccessPolicyOutSerializer, AdminAccessPolicyListInSerializer, ListResponseAdminAccessPolicyOutSerializer } from "@diomhq/diom";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerAdminAuthPolicyCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `configure <body>`,
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
      const client = io.diom;
      
      
      const adminAccessPolicyConfigureIn = AdminAccessPolicyConfigureInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authPolicy.configure(
        adminAccessPolicyConfigureIn,
      );
      printWireJson(AdminAccessPolicyConfigureOutSerializer._toJsonObject(resp));
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
      const client = io.diom;
      
      
      const adminAccessPolicyDeleteIn = AdminAccessPolicyDeleteInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authPolicy.delete(
        adminAccessPolicyDeleteIn,
      );
      printWireJson(AdminAccessPolicyDeleteOutSerializer._toJsonObject(resp));
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
      const client = io.diom;
      
      
      const adminAccessPolicyGetIn = AdminAccessPolicyGetInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authPolicy.get(
        adminAccessPolicyGetIn,
      );
      printWireJson(AdminAccessPolicyOutSerializer._toJsonObject(resp));
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
      const client = io.diom;
      
      
      const bodyRaw = argv.body as string | undefined;
      const adminAccessPolicyListIn =
        bodyRaw === undefined
          ? {}
          : AdminAccessPolicyListInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.admin.authPolicy.list(
        adminAccessPolicyListIn,
      );
      printWireJson(ListResponseAdminAccessPolicyOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}