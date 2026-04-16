import { Diom } from '@diomhq/diom'
import yargsFactory from 'yargs'
import {
	attachCommands,
	configureCommonOptions,
	executeYargs,
	expandVerbose,
} from './cli-core.ts'
import { diomConfigFilePath, mergeCliConfig } from './config.ts'
import type { IoContext } from './io.ts'
import { readCliVersion } from './version.ts'

const VERSION = readCliVersion()

export async function runCli(
	rawArgv: string[],
	io: IoContext,
): Promise<number> {
	const args = expandVerbose(rawArgv)

	const y = configureCommonOptions(yargsFactory(args))
		.option('server-url', {
			describe:
				'Base url for server. Overrides any config file. If not passed, http://localhost:8050 is used',
		})
		.option('auth-token', {
			describe: 'Authentication token. Overrides any config file.',
		})
		.version(VERSION)
		.alias('V', 'version')
		.middleware((argv: Record<string, unknown>) => {
			const merged = mergeCliConfig(
				{
					serverUrl: argv.serverUrl as string | undefined,
					authToken: argv.authToken as string | undefined,
				},
				diomConfigFilePath(),
			)
			io.diom = new Diom(merged.authToken, {
				serverUrl: merged.serverUrl,
			})
		}, true)

	attachCommands(y, io)

	y.command(
		'version',
		'Get the version of the Diom CLI',
		(cmdY) => cmdY.strict(false),
		() => {
			console.log(VERSION)
		},
	)

	return executeYargs(y, args)
}
