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
  "namespace": "...",
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
      const client = getCliDiom(io);
      
      const topic = String(
        argv["topic"],
      );
      
      
      
      const msgTopicConfigureIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.msgs.topic.configure(
        topic,
        msgTopicConfigureIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}