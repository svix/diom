import yargsFactory from "yargs";
import type { Argv } from "yargs";
import { Diom } from "../../javascript/src/client.js";
import { diomConfigFilePath, mergeCliConfig } from "./config.js";
import { readCliVersion } from "./version.js";
import type { IoContext } from "./io.js";
import { registerAdminCommands } from "./generated/admin.js";
import { registerCacheCommands } from "./generated/cache.js";
import { registerHealthCommands } from "./generated/health.js";
import { registerIdempotencyCommands } from "./generated/idempotency.js";
import { registerKvCommands } from "./generated/kv.js";
import { registerMsgsCommands } from "./generated/msgs.js";
import { registerRateLimitCommands } from "./generated/rateLimit.js";

const VERSION = readCliVersion();

/** Map clap-style `-v` / `-vv` to repeated `--verbose` for yargs `count`. */
function expandVerboseArgv(argv: string[]): string[] {
  let n = 0;
  const out: string[] = [];
  for (const a of argv) {
    if (/^-v+$/.test(a)) {
      n += a.length - 1;
    } else {
      out.push(a);
    }
  }
  for (let i = 0; i < n; i++) {
    out.push("--verbose");
  }
  return out;
}

function attachTopLevel(
  y: Argv,
  wrap: (reg: (a: Argv, b: IoContext) => Argv) => (y2: Argv) => Argv,
) {
  y.command("cache", "", wrap(registerCacheCommands));
  y.command("idempotency", "", wrap(registerIdempotencyCommands));
  y.command("kv", "", wrap(registerKvCommands));
  y.command("msgs", "", wrap(registerMsgsCommands));
  y.command("rate-limit", "", wrap(registerRateLimitCommands));
  y.command("health", "", wrap(registerHealthCommands));
  y.command(
    "raw-admin",
    "Send raw administrative commands",
    wrap(registerAdminCommands),
  );
}

export async function runCli(rawArgv: string[], io: IoContext): Promise<number> {
  const args = expandVerboseArgv(rawArgv);

  const y = yargsFactory(args)
    .scriptName("diom")
    .exitProcess(false)
    .usage("$0 <command> [options]")
    .option("color", {
      type: "string",
      choices: ["auto", "always", "never"] as const,
      default: "auto",
      global: true,
      describe: "Controls when to use color",
    })
    .option("verbose", {
      type: "count",
      global: true,
      describe: "Log more. This option may be repeated up to 3 times",
    })
    .option("server-url", {
      alias: "s",
      type: "string",
      global: true,
      describe:
        "Base url for server. Overrides any config file. If not passed, http://localhost:8050 is used",
    })
    .option("auth-token", {
      type: "string",
      global: true,
      describe: "Authentication token. Overrides any config file.",
    })
    .version(VERSION)
    .alias("V", "version")
    .help("h")
    .alias("h", "help")
    .completion(
      "completion",
      "Generate the autocompletion script for bash/zsh",
    )
    .strict()
    .demandCommand(1, "A command is required")
    .middleware((argv: Record<string, unknown>) => {
      const merged = mergeCliConfig(
        {
          serverUrl: argv.serverUrl as string | undefined,
          authToken: argv.authToken as string | undefined,
        },
        diomConfigFilePath(),
      );
      io._diom = new Diom(merged.authToken, {
        serverUrl: merged.serverUrl,
        fetch: io.fetch,
      });
    }, true);

  const wrap =
    (reg: (a: Argv, b: IoContext) => Argv) =>
      (y2: Argv): Argv =>
        reg(y2, io);

  attachTopLevel(y, wrap);

  y.command(
    "version",
    "Get the version of the Diom CLI",
    (cmdY) => cmdY.strict(false),
    () => {
      console.log(VERSION);
    },
  );

  let exitCode = 0;
  let hasFailed = false;
  y.fail((msg, err, failYargs) => {
    if (hasFailed) return;
    hasFailed = true;
    if (msg) {
      console.error(msg);
    } else if (err) {
      console.error(err.message);
    }
    // Extract just the usage line from the help text (first non-empty line).
    (failYargs.showHelp as (fn: (s: string) => void) => void)((help) => {
      const usageLine = help.split("\n").find((l) => l.trim());
      if (usageLine) {
        console.error(`\nUsage: ${usageLine.trim()}`);
      }
    });
    console.error("\nFor more information, try '--help'.");
    exitCode = 1;
  });

  try {
    await y.parseAsync(args);
  } catch (e) {
    if (!hasFailed) {
      const msg = e instanceof Error ? e.message : String(e);
      if (msg) {
        console.error(msg);
      }
      console.error("\nFor more information, try '--help'.");
    }
    exitCode = 1;
  }
  return exitCode;
}
