import { Packr } from "msgpackr";
import { describe, expect, it, vi } from "vitest";
import { runCli } from "./program.js";
import type { IoContext } from "./io.js";
import { parseByteString } from "./byte-string.js";
import { parseJsonArg } from "./json-arg.js";

const MSGPACK_CODEC = new Packr({
  useRecords: false,
  encodeUndefinedAsNil: true,
  skipValues: [undefined],
} as Record<string, unknown>);

function normalizeHelp(s: string): string {
  return s
    .replace(/\r\n/g, "\n")
    .replace(/ +$/gm, "")
    .replace(/\n{3,}/g, "\n\n")
    .trim();
}

describe("runCli", () => {
  it("lists top-level commands in help", async () => {
    const log = vi.spyOn(console, "log").mockImplementation(() => {});
    const err = vi.spyOn(console, "error").mockImplementation(() => {});
    const code = await runCli(["--help"], {
      readStdin: async () => "",
      fetch: globalThis.fetch,
    });
    expect(code).toBe(0);
    const out = [...log.mock.calls, ...err.mock.calls]
      .map((c) => String(c[0]))
      .join("\n");
    log.mockRestore();
    err.mockRestore();
    expect(normalizeHelp(out)).toContain("cache");
    expect(normalizeHelp(out)).toContain("rate-limit");
    expect(normalizeHelp(out)).toContain("raw-admin");
    expect(normalizeHelp(out)).not.toContain("benchmark");
    expect(normalizeHelp(out)).not.toContain("cluster-admin");
  });

  it("nested cache namespace create appears in help", async () => {
    const log = vi.spyOn(console, "log").mockImplementation(() => {});
    const err = vi.spyOn(console, "error").mockImplementation(() => {});
    const code = await runCli(["cache", "namespace", "create", "--help"], {
      readStdin: async () => "",
      fetch: globalThis.fetch,
    });
    expect(code).toBe(0);
    const out = [...log.mock.calls, ...err.mock.calls]
      .map((c) => String(c[0]))
      .join("\n");
    log.mockRestore();
    err.mockRestore();
    expect(normalizeHelp(out)).toMatch(/create|namespace/i);
    expect(normalizeHelp(out)).toContain("Example body:");
  });

  it("health ping with mock fetch prints JSON", async () => {
    const log = vi.spyOn(console, "log").mockImplementation(() => {});
    const io: IoContext = {
      readStdin: async () => "",
      fetch: async (input) => {
        const u = String(input instanceof URL ? input.href : typeof input === "string" ? input : (input as Request).url);
        if (!u.includes("/api/v1.health.ping")) {
          throw new Error(`unexpected URL ${u}`);
        }
        const body = MSGPACK_CODEC.pack({ ok: true }) as Uint8Array;
        return new Response(body, {
          status: 200,
          headers: { "content-type": "application/msgpack" },
        });
      },
    };
    const code = await runCli(["health", "ping"], io);
    expect(code).toBe(0);
    const printed = log.mock.calls.map((c) => c.join(" ")).join("\n");
    log.mockRestore();
    const obj = JSON.parse(printed) as { ok: boolean };
    expect(obj.ok).toBe(true);
  });

  it("fails on invalid JSON body", async () => {
    const err = vi.spyOn(console, "error").mockImplementation(() => {});
    const io: IoContext = {
      readStdin: async () => "",
      fetch: async () => new Response(null, { status: 500 }),
    };
    const code = await runCli(
      ["cache", "namespace", "create", "not-json{"],
      io,
    );
    expect(code).toBe(1);
    err.mockRestore();
  });
});

describe("parseByteString", () => {
  it("parses bracket byte list", () => {
    expect([...parseByteString("[1, 2, 3]")]).toEqual([1, 2, 3]);
  });

  it("parses plain string as utf-8", () => {
    expect([...parseByteString("ab")]).toEqual([0x61, 0x62]);
  });
});

describe("parseJsonArg", () => {
  it("maps snake_case keys to camelCase", async () => {
    const v = await parseJsonArg<{ ttlMs: number }>(
      '{"ttl_ms": 5}',
      async () => "",
    );
    expect(v.ttlMs).toBe(5);
  });
});
