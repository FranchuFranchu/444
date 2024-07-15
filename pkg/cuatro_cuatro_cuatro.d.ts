/* tslint:disable */
/* eslint-disable */
/**
*/
export class Interface {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} b64
*/
  from_base64(b64: string): void;
/**
* @returns {string | undefined}
*/
  to_base64(): string | undefined;
/**
* @param {Uint8Array} array
* @param {number} len
*/
  add_moves(array: Uint8Array, len: number): void;
/**
* @returns {Uint8Array | undefined}
*/
  export_moves(): Uint8Array | undefined;
/**
* @returns {number}
*/
  history_len(): number;
/**
* @returns {State}
*/
  get_last(): State;
/**
* @returns {boolean}
*/
  player_id(): boolean;
/**
* @param {number} diff
*/
  play_ai(diff: number): void;
/**
* @param {number} x
* @param {number} y
*/
  play_at(x: number, y: number): void;
/**
* @param {State} state
*/
  push(state: State): void;
/**
* @returns {State | undefined}
*/
  undo(): State | undefined;
/**
* @param {number} len
*/
  undo_to(len: number): void;
/**
* @param {number} step
* @returns {State | undefined}
*/
  peek(step: number): State | undefined;
/**
* @returns {WinResult | undefined}
*/
  winner(): WinResult | undefined;
}
/**
* Layout:
* [     z = 3    ][     z = 2    ][     z = 1    ][     z = 0    ] z
* [y3][y2][y1][y0][y3][y2][y1][y0][y3][y2][y1][y0][y3][y2][y1][y0] y
* 3210321032103210321032103210321032103210321032103210321032103210 x
*/
export class State {
  free(): void;
/**
*/
  constructor();
/**
* @returns {State}
*/
  wasm_clone(): State;
/**
* @param {number} x
* @param {number} y
* @param {number} z
* @returns {boolean | undefined}
*/
  get_at(x: number, y: number, z: number): boolean | undefined;
/**
* @param {number} x
* @param {number} y
* @param {number} z
* @param {boolean | undefined} [val]
* @returns {State}
*/
  set_at(x: number, y: number, z: number, val?: boolean): State;
/**
*/
  0: bigint;
/**
*/
  1: bigint;
}
/**
*/
export class WinResult {
  free(): void;
/**
*/
  0: boolean;
/**
*/
  1: State;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_interface_free: (a: number) => void;
  readonly __wbg_winresult_free: (a: number) => void;
  readonly __wbg_get_winresult_0: (a: number) => number;
  readonly __wbg_set_winresult_0: (a: number, b: number) => void;
  readonly __wbg_get_winresult_1: (a: number) => number;
  readonly __wbg_set_winresult_1: (a: number, b: number) => void;
  readonly interface_new: () => number;
  readonly interface_from_base64: (a: number, b: number, c: number, d: number) => void;
  readonly interface_to_base64: (a: number, b: number) => void;
  readonly interface_add_moves: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly interface_export_moves: (a: number, b: number) => void;
  readonly interface_history_len: (a: number) => number;
  readonly interface_get_last: (a: number) => number;
  readonly interface_player_id: (a: number) => number;
  readonly interface_play_ai: (a: number, b: number, c: number) => void;
  readonly interface_play_at: (a: number, b: number, c: number, d: number) => void;
  readonly interface_push: (a: number, b: number) => void;
  readonly interface_undo: (a: number) => number;
  readonly interface_undo_to: (a: number, b: number) => void;
  readonly interface_peek: (a: number, b: number) => number;
  readonly interface_winner: (a: number) => number;
  readonly __wbg_state_free: (a: number) => void;
  readonly __wbg_get_state_0: (a: number) => number;
  readonly __wbg_set_state_0: (a: number, b: number) => void;
  readonly __wbg_get_state_1: (a: number) => number;
  readonly __wbg_set_state_1: (a: number, b: number) => void;
  readonly state_empty: () => number;
  readonly state_wasm_clone: (a: number) => number;
  readonly state_get_at: (a: number, b: number, c: number, d: number) => number;
  readonly state_set_at: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
