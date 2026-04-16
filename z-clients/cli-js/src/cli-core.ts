import { ApiException } from '@diomhq/diom'
import type { Argv } from 'yargs'
// import { registerAdminCommands } from './generated/admin.ts'
import { registerCacheCommands } from './generated/cache.ts'
import { registerHealthCommands } from './generated/health.ts'
import { registerIdempotencyCommands } from './generated/idempotency.ts'
import { registerKvCommands } from './generated/kv.ts'
import { registerMsgsCommands } from './generated/msgs.ts'
import { registerRateLimitCommands } from './generated/rateLimit.ts'
import type { IoContext } from './io.ts'

function logApiException(err: ApiException<unknown>): void {
	console.error(`HTTP-Code: ${err.code}`)
	if (err.body != null) {
		console.error(JSON.stringify(err.body, null, 4))
	}
}

/** Without a throw, yargs keeps parsing and may still run command handlers after validation errors. */
function toAbortError(msg: string | undefined, err: unknown): Error {
	if (err instanceof Error) return err
	if (msg) return new Error(msg)
	return new Error('Command failed')
}

/** Map clap-style `-v` / `-vv` to repeated `--verbose` for yargs `count`. */
export function expandVerbose(argv: string[]): string[] {
	let n = 0
	const out: string[] = []
	for (const a of argv) {
		if (/^-v+$/.test(a)) {
			n += a.length - 1
		} else {
			out.push(a)
		}
	}
	for (let i = 0; i < n; i++) {
		out.push('--verbose')
	}
	return out
}

/** Apply the CLI options shared by both the Node CLI and the browser REPL. */
export function configureCommonOptions(y: Argv): Argv {
	return y
		.scriptName('diom')
		.exitProcess(false)
		.usage('$0 <command> [options]')
		.option('color', {
			type: 'string',
			choices: ['auto', 'always', 'never'] as const,
			default: 'auto',
			global: true,
			describe: 'Controls when to use color',
		})
		.option('verbose', {
			type: 'count',
			global: true,
			describe: 'Log more. This option may be repeated up to 3 times',
		})
		.option('server-url', {
			alias: 's',
			type: 'string',
			global: true,
			describe: 'Base url for server.',
		})
		.option('auth-token', {
			type: 'string',
			global: true,
			describe: 'Authentication token.',
		})
		.help('h')
		.alias('h', 'help')
		.strict()
		.demandCommand(1, 'A command is required')
}

/** Register all top-level API commands on the yargs instance. */
export function attachCommands(y: Argv, io: IoContext): void {
	const wrap =
		(reg: (a: Argv, b: IoContext) => Argv) =>
			(y2: Argv): Argv =>
				reg(y2, io)

	y.command('cache', '', wrap(registerCacheCommands))
	y.command('idempotency', '', wrap(registerIdempotencyCommands))
	y.command('kv', '', wrap(registerKvCommands))
	y.command('msgs', '', wrap(registerMsgsCommands))
	y.command('rate-limit', '', wrap(registerRateLimitCommands))
	y.command('health', '', wrap(registerHealthCommands))
	// y.command(
	// 	'raw-admin',
	// 	'Send raw administrative commands',
	// 	wrap(registerAdminCommands),
	// )
}

/** Install error handling, run yargs, and return an exit code. */
export async function executeYargs(y: Argv, args: string[]): Promise<number> {
	let exitCode = 0
	let reportedByFailHandler = false

	y.fail((msg, err, failYargs) => {
		if (reportedByFailHandler) throw toAbortError(msg, err)
		reportedByFailHandler = true
		exitCode = 1

		if (err instanceof ApiException) {
			logApiException(err)
			throw err
		}

		if (msg) console.error(msg)
		else if (err)
			console.error(err instanceof Error ? err.message : String(err))

				; (failYargs.showHelp as (fn: (s: string) => void) => void)((help) => {
					const line = help.split('\n').find((l) => l.trim())
					if (line) console.error(`\nUsage: ${line.trim()}`)
				})
		console.error("\nFor more information, try '--help'.")
		throw toAbortError(msg, err)
	})

	try {
		await y.parseAsync(args)
	} catch (e) {
		if (!reportedByFailHandler) {
			if (e instanceof ApiException) logApiException(e)
			else {
				const msg = e instanceof Error ? e.message : String(e)
				if (msg) console.error(msg)
				console.error("\nFor more information, try '--help'.")
			}
			exitCode = 1
		}
	}

	return exitCode
}
