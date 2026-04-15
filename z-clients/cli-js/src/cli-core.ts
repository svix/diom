import type { Argv } from 'yargs'
import { ApiException } from '@diomhq/diom'
// import { registerAdminCommands } from './generated/admin.ts'
import { registerCacheCommands } from './generated/cache.ts'
import { registerHealthCommands } from './generated/health.ts'
import { registerIdempotencyCommands } from './generated/idempotency.ts'
import { registerKvCommands } from './generated/kv.ts'
import { registerMsgsCommands } from './generated/msgs.ts'
import { registerRateLimitCommands } from './generated/rateLimit.ts'
import type { IoContext } from './io.ts'

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
export async function executeYargs(
	y: Argv,
	args: string[],
): Promise<number> {
	let exitCode = 0
	let hasFailed = false

	y.fail((msg, err, failYargs) => {
		if (hasFailed) return
		hasFailed = true
		if (err instanceof ApiException) {
			console.error(`HTTP-Code: ${err.code}`)
			if (err.body != null) {
				console.error(JSON.stringify(err.body, null, 4))
			}
			exitCode = 1
			return
		}
		if (msg) {
			console.error(msg)
		} else if (err) {
			console.error(err.message)
		}
		; (failYargs.showHelp as (fn: (s: string) => void) => void)((help) => {
			const usageLine = help.split('\n').find((l) => l.trim())
			if (usageLine) {
				console.error(`\nUsage: ${usageLine.trim()}`)
			}
		})
		console.error("\nFor more information, try '--help'.")
		exitCode = 1
	})

	try {
		await y.parseAsync(args)
	} catch (e) {
		if (!hasFailed) {
			if (e instanceof ApiException) {
				console.error(`HTTP-Code: ${e.code}`)
				if (e.body != null) {
					console.error(JSON.stringify(e.body, null, 4))
				}
			} else {
				const msg = e instanceof Error ? e.message : String(e)
				if (msg) {
					console.error(msg)
				}
				console.error("\nFor more information, try '--help'.")
			}
			exitCode = 1
		}
	}

	return exitCode
}
