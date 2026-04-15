/**
 * JSON.stringify replacer that converts byte arrays (number[]) to
 * UTF-8 strings, matching the Rust CLI's serde byte_array display.
 */
function byteArrayReplacer(_key: string, value: unknown): unknown {
	if (
		Array.isArray(value) &&
		value.length > 0 &&
		value.every((v) => typeof v === 'number' && v >= 0 && v <= 255)
	) {
		return new TextDecoder().decode(new Uint8Array(value))
	}
	return value
}

/**
 * Print a wire-format JSON object (from Serializer._toJsonObject) to stdout,
 * converting byte arrays to UTF-8 strings for human-readable CLI output.
 */
export function printWireJson(wireObj: unknown): void {
	console.log(JSON.stringify(wireObj, byteArrayReplacer, 4))
}
