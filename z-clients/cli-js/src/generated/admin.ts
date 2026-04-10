// @ts-nocheck
// this file is @generated
import { registerAdminAuthPolicyCommands } from "./adminAuthPolicy.js";
import { registerAdminAuthRoleCommands } from "./adminAuthRole.js";
import { registerAdminAuthTokenCommands } from "./adminAuthToken.js";
import { registerAdminClusterCommands } from "./adminCluster.js";
import type { Argv } from "yargs";
import type { IoContext } from "../io.js";
import { getCliDiom } from "../diom-holder.js";
import { parseByteString } from "../byte-string.js";
import { parseJsonArg } from "../json-arg.js";
import { printJsonOutput } from "../print-json.js";

/**
 * Register CLI commands for this API resource (nested yargs commands; same shape as the Rust diom-cli).
 */
export function registerAdminCommands(
  y: Argv,
  io: IoContext,
): Argv {
  y.command(
    "auth-policy",
    "",
    (y2) => {
      registerAdminAuthPolicyCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  y.command(
    "auth-role",
    "",
    (y2) => {
      registerAdminAuthRoleCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  y.command(
    "auth-token",
    "",
    (y2) => {
      registerAdminAuthTokenCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  y.command(
    "cluster",
    "",
    (y2) => {
      registerAdminClusterCommands(y2, io);
      return y2.demandCommand(1).strict();
    },
  );
  

  

  return y.demandCommand(1).strict();
}