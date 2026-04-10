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
      const client = getCliDiom(io);
      
      
      const adminAuthTokenCreateIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authToken.create(
        adminAuthTokenCreateIn,
      );
      printJsonOutput(resp);
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
      const client = getCliDiom(io);
      
      
      const adminAuthTokenExpireIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authToken.expire(
        adminAuthTokenExpireIn,
      );
      printJsonOutput(resp);
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
      const client = getCliDiom(io);
      
      
      const adminAuthTokenRotateIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authToken.rotate(
        adminAuthTokenRotateIn,
      );
      printJsonOutput(resp);
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
      const client = getCliDiom(io);
      
      
      const adminAuthTokenDeleteIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authToken.delete(
        adminAuthTokenDeleteIn,
      );
      printJsonOutput(resp);
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
      const client = getCliDiom(io);
      
      
      const bodyRaw = argv.body as string | undefined;
      const adminAuthTokenListIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.admin.authToken.list(
        adminAuthTokenListIn,
      );
      printJsonOutput(resp);
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
      const client = getCliDiom(io);
      
      
      const adminAuthTokenUpdateIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.authToken.update(
        adminAuthTokenUpdateIn,
      );
      printJsonOutput(resp);
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
      const client = getCliDiom(io);
      
      
      const bodyRaw = argv.body as string | undefined;
      const adminAuthTokenWhoamiIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.admin.authToken.whoami(
        adminAuthTokenWhoamiIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}