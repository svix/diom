// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { MsgTopicConfigureInSerializer, MsgTopicConfigureOutSerializer } from "@diomhq/diom/internal";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerMsgsTopicCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `configure <topic> <body>`,
    `Configures the number of partitions for a topic.

Partition count can only be increased, never decreased. The default for a new topic is 1.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "partitions": "..."
}`,
          "",
          `Example response:
{
  "partitions": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = io.diom;
      
      const topic = String(
        argv["topic"],
      );
      
      
      
      const msgTopicConfigureIn = MsgTopicConfigureInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.msgs.topic.configure(
        topic,
        msgTopicConfigureIn,
      );
      printWireJson(MsgTopicConfigureOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}