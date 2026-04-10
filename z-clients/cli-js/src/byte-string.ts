/**
 * Match Rust diom-cli `ByteString`: `[1, 2, 3]` → bytes; otherwise UTF-8 string as bytes.
 */
export function parseByteString(s: string): Uint8Array {
  const t = s.trim();
  if (t.startsWith("[") && t.endsWith("]")) {
    const inner = t.slice(1, -1);
    if (inner.trim() === "") {
      return new Uint8Array();
    }
    const parts = inner.split(",").map((p) => p.trim());
    return new Uint8Array(parts.map((p) => Number.parseInt(p, 10)));
  }
  return new TextEncoder().encode(s);
}
