// @ts-nocheck
// this file is @generated


import type { Argv } from "yargs";
import type { IoContext } from "../io.ts";
import { registerAdminAuthPolicyCommands } from "./adminAuthPolicy.ts";
import { registerAdminAuthRoleCommands } from "./adminAuthRole.ts";
import { registerAdminAuthTokenCommands } from "./adminAuthToken.ts";


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
  

  

  return y.demandCommand(1).strict();
}