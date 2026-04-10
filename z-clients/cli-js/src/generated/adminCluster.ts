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
export function registerAdminClusterCommands(
  y: Argv,
  io: IoContext,
): Argv {
  

  
  y.command(
    "status",
    `Get information about the current cluster`,
    (cmdY) => {
      cmdY.epilog(
        `Example response:
{
  "cluster_id": "...",
  "cluster_name": "...",
  "this_node_id": "...",
  "this_node_state": "...",
  "this_node_last_committed_timestamp": "...",
  "this_node_last_snapshot_id": "...",
  "nodes": "..."
}`,
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      const resp = await client.admin.cluster.status();
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `initialize [body]`,
    `Initialize this node as the leader of a new cluster

This operation may only be performed against a node which has not been
initialized and is not currently a member of a cluster.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
}`,
          "",
          `Example response:
{
  "cluster_id": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const bodyRaw = argv.body as string | undefined;
      const clusterInitializeIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.admin.cluster.initialize(
        clusterInitializeIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `remove-node <body>`,
    `Remove a node from the cluster.

This operation executes immediately and the node must be wiped and reset
before it can safely be added to the cluster.`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
  "node_id": "..."
}`,
          "",
          `Example response:
{
  "node_id": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const clusterRemoveNodeIn = await parseJsonArg(
        String(argv.body),
        io.readStdin,
      );
      
      const resp = await client.admin.cluster.removeNode(
        clusterRemoveNodeIn,
      );
      printJsonOutput(resp);
    },
  );
  
  
  
  y.command(
    `force-snapshot [body]`,
    `Force the cluster to take a snapshot immediately`,
    (cmdY) => {
      cmdY.epilog(
        [
          `Example body:
{
}`,
          "",
          `Example response:
{
  "snapshot_time": "...",
  "snapshot_log_index": "...",
  "snapshot_id": "..."
}`,
        ].join("\n"),
      );
      return cmdY;
    },
    async (argv) => {
      const client = getCliDiom(io);
      
      
      const bodyRaw = argv.body as string | undefined;
      const clusterForceSnapshotIn =
        bodyRaw === undefined
          ? {}
          : await parseJsonArg(bodyRaw, io.readStdin);
      
      const resp = await client.admin.cluster.forceSnapshot(
        clusterForceSnapshotIn,
      );
      printJsonOutput(resp);
    },
  );
  
  

  return y.demandCommand(1).strict();
}