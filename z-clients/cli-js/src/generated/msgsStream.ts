// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { MsgStreamReceiveInSerializer, MsgStreamReceiveOutSerializer, MsgStreamCommitInSerializer, MsgStreamCommitOutSerializer, MsgStreamSeekInSerializer, MsgStreamSeekOutSerializer } from "@diomhq/diom/internal";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerMsgsStreamCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `receive <topic> <consumer-group> [body]`,
    `Receives messages from a topic using a consumer group.

Each consumer in the group reads from all partitions. Messages are locked by leases for the
specified duration to prevent duplicate delivery within the same consumer group.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "batch_size": "...",
  "lease_duration_ms": "...",
  "default_starting_position": "...",
  "batch_wait_ms": "..."
}`,
          "",
          `Example response:
{
  "msgs": "..."
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
      
      
      const consumerGroup = String(
        argv["consumer-group"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const msgStreamReceiveIn =
        bodyRaw === undefined
          ? {}
          : MsgStreamReceiveInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.msgs.stream.receive(
        topic,
        consumerGroup,
        msgStreamReceiveIn,
      );
      printWireJson(MsgStreamReceiveOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `commit <topic> <consumer-group> <body>`,
    `Commits an offset for a consumer group on a specific partition.

The topic must be a partition-level topic (e.g. 'ns:my-topic~3'). The offset is the last
successfully processed offset; future receives will start after it.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "offset": "..."
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
      
      const topic = String(
        argv["topic"],
      );
      
      
      const consumerGroup = String(
        argv["consumer-group"],
      );
      
      
      
      const msgStreamCommitIn = MsgStreamCommitInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.msgs.stream.commit(
        topic,
        consumerGroup,
        msgStreamCommitIn,
      );
      printWireJson(MsgStreamCommitOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `seek <topic> <consumer-group> [body]`,
    `Repositions a consumer group's read cursor on a topic.

Provide exactly one of 'offset' or 'position'. When using 'offset', the topic must include a
partition suffix (e.g. 'ns:my-topic~0'). The 'position' field accepts '"earliest"' or
'"latest"' and may be used with or without a partition suffix.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "offset": "...",
  "position": "..."
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
      
      const topic = String(
        argv["topic"],
      );
      
      
      const consumerGroup = String(
        argv["consumer-group"],
      );
      
      
      
      const bodyRaw = argv.body as string | undefined;
      const msgStreamSeekIn =
        bodyRaw === undefined
          ? {}
          : MsgStreamSeekInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.msgs.stream.seek(
        topic,
        consumerGroup,
        msgStreamSeekIn,
      );
      printWireJson(MsgStreamSeekOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}