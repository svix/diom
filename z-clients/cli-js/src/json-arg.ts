import type { IoContext } from './io.ts'

/**
 * Read a raw JSON argument from a CLI string or stdin (when "-").
 * Returns the parsed object with keys as-is (snake_case) — the caller
 * is expected to pass it through a Serializer._fromJsonObject().
 */
export async function readJsonArg(
	raw: string,
	readStdin: IoContext['readStdin'],
	// biome-ignore lint/suspicious/noExplicitAny: raw JSON
): Promise<any> {
	const text = raw === '-' ? await readStdin() : raw
	return JSON.parse(text)
}
