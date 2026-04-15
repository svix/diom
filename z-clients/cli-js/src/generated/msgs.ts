// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { MsgPublishInSerializer, MsgPublishOutSerializer } from "@diomhq/diom";
import { registerMsgsNamespaceCommands } from "./msgsNamespace.ts";
import { registerMsgsQueueCommands } from "./msgsQueue.ts";
import { registerMsgsStreamCommands } from "./msgsStream.ts";
import { registerMsgsTopicCommands } from "./msgsTopic.ts";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerMsgsCommands(
  y: Argv,
  io: IoContext,
): Argv {
  y.command(
    "namespace",
    "",
    (y2) => {
      registerMsgsNamespaceCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  y.command(
    "queue",
    "",
    (y2) => {
      registerMsgsQueueCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  y.command(
    "stream",
    "",
    (y2) => {
      registerMsgsStreamCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  y.command(
    "topic",
    "",
    (y2) => {
      registerMsgsTopicCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  

  
  
  y.command(
    `publish <topic> <body>`,
    `Publishes messages to a topic within a namespace.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "msgs": "...",
  "idempotency_key": "..."
}`,
          "",
          `Example response:
{
  "topics": "..."
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
      
      
      
      const msgPublishIn = MsgPublishInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.msgs.publish(
        topic,
        msgPublishIn,
      );
      printWireJson(MsgPublishOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}