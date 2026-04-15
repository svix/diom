#!/usr/bin/env node
import type { IoContext } from './io.ts'
import { runCli } from './run-cli.ts'

async function readStdinAll(): Promise<string> {
  if (process.stdin.isTTY) {
    return ''
  }
  const chunks: Buffer[] = []
  for await (const chunk of process.stdin) {
    chunks.push(chunk as Buffer)
  }
  return Buffer.concat(chunks).toString('utf8')
}

const io: IoContext = {
  readStdin: readStdinAll,
}

const code = await runCli(process.argv.slice(2), io)
process.exit(code)
