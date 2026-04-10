// @ts-nocheck
// this file is @generated
import { registerMsgsNamespaceCommands } from "./msgsNamespace.js";
import { registerMsgsQueueCommands } from "./msgsQueue.js";
import { registerMsgsStreamCommands } from "./msgsStream.js";
import { registerMsgsTopicCommands } from "./msgsTopic.js";
import type { Argv } from "yargs";
import type { IoContext } from "../io.js";
import { getCliDiom } from "../diom-holder.js";
import { parseByteString } from "../byte-string.js";
import { parseJsonArg } from "../json-arg.js";
import { printJsonOutput } from "../print-json.js";

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
  "namespace": "...",
  "msgs": "..."
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
      const client = getCliDiom(io);
      
      const topic = String(
        argv["topic"],
      );
      
      
      
      const msgPublishIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.msgs.publish(
        topic,
        msgPublishIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}