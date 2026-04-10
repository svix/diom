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
export function registerMsgsNamespaceCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `create <name> [body]`,
    `Creates or updates a msgs namespace with the given name.`,
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
  "name": "...",
  "retention": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      const name = String(
        argv["name"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const msgNamespaceCreateIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.msgs.namespace.create(
        name,
        msgNamespaceCreateIn,
      );
      printJsonOutput(resp);
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
  "name": "...",
  "retention": "...",
  "created": "...",
  "updated": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      const name = String(
        argv["name"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const msgNamespaceGetIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.msgs.namespace.get(
        name,
        msgNamespaceGetIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}