export type GithubError = GithubErrorCustom;
export interface GithubErrorCustom {
  tag: 'custom',
  val: string,
}
/**
* # Variants
* 
* ## `"all"`
* 
* ## `"github"`
* 
* ## `"npm"`
* 
* ## `"cargo"`
*/
export type GithubTarget = 'all' | 'github' | 'npm' | 'cargo';
import { WasiCliEnvironment } from './interfaces/wasi-cli-environment.js';
import { WasiCliExit } from './interfaces/wasi-cli-exit.js';
import { WasiCliStderr } from './interfaces/wasi-cli-stderr.js';
import { WasiCliStdin } from './interfaces/wasi-cli-stdin.js';
import { WasiCliStdout } from './interfaces/wasi-cli-stdout.js';
declare const WasiClocksWallClock: {};
import { WasiFilesystemPreopens } from './interfaces/wasi-filesystem-preopens.js';
import { WasiFilesystemTypes } from './interfaces/wasi-filesystem-types.js';
import { WasiIoError } from './interfaces/wasi-io-error.js';
import { WasiIoPoll } from './interfaces/wasi-io-poll.js';
import { WasiIoStreams } from './interfaces/wasi-io-streams.js';
import { WasiRandomRandom } from './interfaces/wasi-random-random.js';
import { WasiSocketsTcp } from './interfaces/wasi-sockets-tcp.js';
import { WasiSocketsUdp } from './interfaces/wasi-sockets-udp.js';
export function runWithConfig(config: string, target: GithubTarget): void;
