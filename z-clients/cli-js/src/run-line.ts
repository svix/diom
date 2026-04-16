import { Diom } from '@diomhq/diom'
import stringArgv from 'string-argv'
import type { Argv } from 'yargs'
import yargsFactory from 'yargs/browser'
import {
	attachCommands,
	configureCommonOptions,
	executeYargs,
	expandVerbose,
} from './cli-core.ts'
import type { IoContext } from './io.ts'

export type RunLineHost = {
	stdout: (s: string) => void
	stderr: (s: string) => void
	readStdin?: () => Promise<string>
	serverUrl: string
	authToken: string
}

function stripLeadingCliName(parts: string[]): string[] {
	const first = parts[0]?.toLowerCase()
	if (first === 'diom' || first === 'coyote') {
		return parts.slice(1)
	}
	return parts
}

/**
 * Browser-compatible REPL runner: parse one line, execute the CLI command,
 * and capture output via the provided host callbacks.
 */
export async function runLine(
	line: string,
	host: RunLineHost,
): Promise<number> {
	const trimmed = line.trim()
	if (!trimmed) return 0

	let parts = stringArgv(trimmed)
	parts = stripLeadingCliName(parts)
	if (parts.length === 0) return 0

	const args = expandVerbose(parts)

	const io: IoContext = {
		readStdin: host.readStdin ?? (async () => ''),
		diom: new Diom(host.authToken, {
			serverUrl: host.serverUrl,
		}),
	}

	const origLog = console.log
	const origError = console.error
	console.log = (...a: unknown[]) => {
		host.stdout(`${a.map(String).join(' ')}\n`)
	}
	console.error = (...a: unknown[]) => {
		host.stderr(`${a.map(String).join(' ')}\n`)
	}

	try {
		const y = configureCommonOptions(yargsFactory(args) as Argv)
		attachCommands(y, io)
		return await executeYargs(y, args)
	} finally {
		console.log = origLog
		console.error = origError
	}
}
