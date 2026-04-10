import type { Diom } from "../../javascript/src/client.js";

export type IoContext = {
  readStdin: () => Promise<string>;
  fetch: typeof fetch;
  /** Set by `runCli` before handlers run; not used by callers. */
  _diom?: Diom;
};
