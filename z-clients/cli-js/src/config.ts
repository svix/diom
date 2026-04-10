import { existsSync, readFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { parse } from "smol-toml";

export type MergedConfig = {
  serverUrl?: string;
  authToken: string;
};

function configDir(): string {
  if (process.platform === "darwin") {
    return path.join(os.homedir(), "Library", "Application Support");
  }
  if (process.platform === "win32") {
    return process.env.APPDATA ?? path.join(os.homedir(), "AppData", "Roaming");
  }
  return process.env.XDG_CONFIG_HOME ?? path.join(os.homedir(), ".config");
}

export function diomConfigFilePath(): string {
  return path.join(configDir(), "diom", "diom-cli-config.toml");
}

type TomlConfig = {
  server_url?: string;
  auth_token?: string;
};

export function mergeCliConfig(
  argv: { serverUrl?: string; authToken?: string },
  filePath: string,
): MergedConfig {
  let file: TomlConfig = {};
  if (existsSync(filePath)) {
    const raw = readFileSync(filePath, "utf8");
    file = parse(raw) as TomlConfig;
  }

  const envUrl = process.env.DIOM_SERVER_URL;
  const envTok =
    process.env.DIOM_AUTH_TOKEN ?? process.env.DIOM_ADMIN_TOKEN ?? undefined;

  const serverUrl =
    argv.serverUrl ?? envUrl ?? file.server_url ?? undefined;
  const authToken =
    argv.authToken ?? envTok ?? file.auth_token ?? "xxx";

  return { serverUrl, authToken };
}
