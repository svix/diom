// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { MsgNamespaceConfigureInSerializer, MsgNamespaceConfigureOutSerializer, MsgNamespaceGetInSerializer, MsgNamespaceGetOutSerializer } from "@diomhq/diom/internal";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerMsgsNamespaceCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `configure <name> [body]`,
    `Configures a msgs namespace with the given name.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "retention": "..."
}`,
          "",
          `Example response:
{
  "name": "some_namespace",
  "retention": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      const name = String(
        argv["name"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const msgNamespaceConfigureIn =
        bodyRaw === undefined
          ? {}
          : MsgNamespaceConfigureInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.msgs.namespace.configure(
        name,
        msgNamespaceConfigureIn,
      );
      printWireJson(MsgNamespaceConfigureOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `get <name> [body]`,
    `Gets a msgs namespace by name.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
}`,
          "",
          `Example response:
{
  "name": "some_namespace",
  "retention": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      const name = String(
        argv["name"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const msgNamespaceGetIn =
        bodyRaw === undefined
          ? {}
          : MsgNamespaceGetInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.msgs.namespace.get(
        name,
        msgNamespaceGetIn,
      );
      printWireJson(MsgNamespaceGetOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}