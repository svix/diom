// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { AdminAuthTokenCreateInSerializer, AdminAuthTokenCreateOutSerializer, AdminAuthTokenExpireInSerializer, AdminAuthTokenExpireOutSerializer, AdminAuthTokenRotateInSerializer, AdminAuthTokenRotateOutSerializer, AdminAuthTokenDeleteInSerializer, AdminAuthTokenDeleteOutSerializer, AdminAuthTokenListInSerializer, ListResponseAdminAuthTokenOutSerializer, AdminAuthTokenUpdateInSerializer, AdminAuthTokenUpdateOutSerializer, AdminAuthTokenWhoamiInSerializer, AdminAuthTokenWhoamiOutSerializer } from "@diomhq/diom/internal";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerAdminAuthTokenCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `create <body>`,
    `Create an auth token`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "name": "...",
  "role": "...",
  "expiry_ms": "...",
  "enabled": "..."
}`,
          "",
          `Example response:
{
  "id": "...",
  "token": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      
      const adminAuthTokenCreateIn = AdminAuthTokenCreateInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authToken.create(
        adminAuthTokenCreateIn,
      );
      printWireJson(AdminAuthTokenCreateOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `expire <body>`,
    `Expire an auth token`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "id": "...",
  "expiry_ms": "..."
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
      
      
      const adminAuthTokenExpireIn = AdminAuthTokenExpireInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authToken.expire(
        adminAuthTokenExpireIn,
      );
      printWireJson(AdminAuthTokenExpireOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `rotate <body>`,
    `Rotate an auth token, invalidating the old one and issuing a new secret`,
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
  "token": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      
      const adminAuthTokenRotateIn = AdminAuthTokenRotateInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authToken.rotate(
        adminAuthTokenRotateIn,
      );
      printWireJson(AdminAuthTokenRotateOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `delete <body>`,
    `Delete an auth token`,
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
      
      
      const adminAuthTokenDeleteIn = AdminAuthTokenDeleteInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authToken.delete(
        adminAuthTokenDeleteIn,
      );
      printWireJson(AdminAuthTokenDeleteOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `list [body]`,
    `List auth tokens for a given owner`,
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
      const adminAuthTokenListIn =
        bodyRaw === undefined
          ? {}
          : AdminAuthTokenListInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.admin.authToken.list(
        adminAuthTokenListIn,
      );
      printWireJson(ListResponseAdminAuthTokenOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `update <body>`,
    `Update an auth token's properties`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "id": "...",
  "name": "...",
  "expiry_ms": "...",
  "enabled": "..."
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
      
      
      const adminAuthTokenUpdateIn = AdminAuthTokenUpdateInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.admin.authToken.update(
        adminAuthTokenUpdateIn,
      );
      printWireJson(AdminAuthTokenUpdateOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `whoami [body]`,
    `Return the role of the currently authenticated token`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
}`,
          "",
          `Example response:
{
  "role": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      
      const bodyRaw = argv.body as string | undefined;
      const adminAuthTokenWhoamiIn =
        bodyRaw === undefined
          ? {}
          : AdminAuthTokenWhoamiInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.admin.authToken.whoami(
        adminAuthTokenWhoamiIn,
      );
      printWireJson(AdminAuthTokenWhoamiOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}