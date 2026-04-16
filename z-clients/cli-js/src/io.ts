import type { Diom } from '@diomhq/diom'

export type IoContext = {
	readStdin: () => Promise<string>
	diom: Diom
}
