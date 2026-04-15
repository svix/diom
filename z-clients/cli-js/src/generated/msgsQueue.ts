// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { readJsonArg } from "../json-arg.ts";
import { printWireJson } from "../print-json.ts";
import { MsgQueueReceiveInSerializer, MsgQueueReceiveOutSerializer, MsgQueueAckInSerializer, MsgQueueAckOutSerializer, MsgQueueExtendLeaseInSerializer, MsgQueueExtendLeaseOutSerializer, MsgQueueConfigureInSerializer, MsgQueueConfigureOutSerializer, MsgQueueNackInSerializer, MsgQueueNackOutSerializer, MsgQueueRedriveDlqInSerializer, MsgQueueRedriveDlqOutSerializer } from "@diomhq/diom";


/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerMsgsQueueCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  
  y.command(
    `receive <topic> <consumer-group> [body]`,
    `Receives messages from a topic as competing consumers.

Messages are individually leased for the specified duration. Multiple consumers can receive
different messages from the same topic concurrently. Leased messages are skipped until they
are acked or their lease expires.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "batch_size": "...",
  "lease_duration_ms": "...",
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
      const msgQueueReceiveIn =
        bodyRaw === undefined
          ? {}
          : MsgQueueReceiveInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.msgs.queue.receive(
        topic,
        consumerGroup,
        msgQueueReceiveIn,
      );
      printWireJson(MsgQueueReceiveOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `ack <topic> <consumer-group> <body>`,
    `Acknowledges messages by their opaque msg_ids.

Acked messages are permanently removed from the queue and will never be re-delivered.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "msg_ids": "..."
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
      
      
      
      const msgQueueAckIn = MsgQueueAckInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.msgs.queue.ack(
        topic,
        consumerGroup,
        msgQueueAckIn,
      );
      printWireJson(MsgQueueAckOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `extend-lease <topic> <consumer-group> <body>`,
    `Extends the lease on in-flight messages.

Consumers that need more processing time can call this before the lease expires to prevent the
message from being re-delivered to another consumer.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "msg_ids": "...",
  "lease_duration_ms": "..."
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
      
      
      
      const msgQueueExtendLeaseIn = MsgQueueExtendLeaseInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.msgs.queue.extendLease(
        topic,
        consumerGroup,
        msgQueueExtendLeaseIn,
      );
      printWireJson(MsgQueueExtendLeaseOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `configure <topic> <consumer-group> [body]`,
    `Configures retry and DLQ behavior for a consumer group on a topic.

'retry_schedule' is a list of delays (in millis) between retries after a nack. Once exhausted,
the message is moved to the DLQ (or forwarded to 'dlq_topic' if set).`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "retry_schedule": "...",
  "dlq_topic": "..."
}`,
          "",
          `Example response:
{
  "retry_schedule": "...",
  "dlq_topic": "..."
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
      const msgQueueConfigureIn =
        bodyRaw === undefined
          ? {}
          : MsgQueueConfigureInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.msgs.queue.configure(
        topic,
        consumerGroup,
        msgQueueConfigureIn,
      );
      printWireJson(MsgQueueConfigureOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `nack <topic> <consumer-group> <body>`,
    `Rejects messages, sending them to the dead-letter queue.

Nacked messages will not be re-delivered by 'queue/receive'. Use 'queue/redrive-dlq' to
move them back to the queue for reprocessing.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace",
  "msg_ids": "..."
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
      
      
      
      const msgQueueNackIn = MsgQueueNackInSerializer._fromJsonObject(
        await readJsonArg(String(argv.body), io.readStdin),
      );
      
      const resp = await client.msgs.queue.nack(
        topic,
        consumerGroup,
        msgQueueNackIn,
      );
      printWireJson(MsgQueueNackOutSerializer._toJsonObject(resp));
    },
  );
  
  
  
  y.command(
    `redrive-dlq <topic> <consumer-group> [body]`,
    `Moves all dead-letter queue messages back to the main queue for reprocessing.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "namespace": "some_namespace"
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
      const msgQueueRedriveDlqIn =
        bodyRaw === undefined
          ? {}
          : MsgQueueRedriveDlqInSerializer._fromJsonObject(await readJsonArg(bodyRaw, io.readStdin));
      
      const resp = await client.msgs.queue.redriveDlq(
        topic,
        consumerGroup,
        msgQueueRedriveDlqIn,
      );
      printWireJson(MsgQueueRedriveDlqOutSerializer._toJsonObject(resp));
    },
  );
  
  

  return y.demandCommand(1).strict();
}