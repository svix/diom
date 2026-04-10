import type { IoContext } from "./io.js";

function snakeToCamelKey(key: string): string {
  return key.replace(/_([a-z])/g, (_, c: string) => c.toUpperCase());
}

/** Deep-convert object keys from snake_case to camelCase (CLI JSON matches Rust serde style). */
export function normalizeJsonKeys<T>(value: unknown): T {
  if (value === null || typeof value !== "object") {
    return value as T;
  }
  if (Array.isArray(value)) {
    return value.map((v) => normalizeJsonKeys(v)) as T;
  }
  const out: Record<string, unknown> = {};
  for (const [k, v] of Object.entries(value as Record<string, unknown>)) {
    const nk = snakeToCamelKey(k);
    out[nk] =
      v !== null && typeof v === "object" && !Array.isArray(v)
        ? normalizeJsonKeys(v)
        : Array.isArray(v)
          ? v.map((x) =>
              x !== null && typeof x === "object" && !Array.isArray(x)
                ? normalizeJsonKeys(x)
                : x,
            )
          : v;
  }
  return out as T;
}

export async function parseJsonArg<T>(
  raw: string,
  readStdin: IoContext["readStdin"],
): Promise<T> {
  const text = raw === "-" ? await readStdin() : raw;
  const parsed: unknown = JSON.parse(text);
  return normalizeJsonKeys<T>(parsed);
}
