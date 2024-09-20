/* tslint:disable */
/* eslint-disable */
/**
* @returns {State}
*/
export function create_state(): State;
/**
*/
export class InputEventInterface {
  free(): void;
/**
* @param {string} id
* @param {string} data
*/
  on_input(id: string, data: string): void;
/**
* @param {string} id
* @param {string} data
*/
  on_change(id: string, data: string): void;
}
/**
*/
export class MouseEventInterface {
  free(): void;
/**
* @param {string} id
*/
  on_mouse_down(id: string): void;
/**
* @param {string} id
*/
  on_mouse_up(id: string): void;
}
/**
*/
export class State {
  free(): void;
/**
*/
  load_last_grades(): void;
/**
* @param {number} index
* @param {number} val
*/
  write_grade(index: number, val: number): void;
/**
*/
  generate_pdf(): void;
}
/**
*/
export class WindowEventInterface {
  free(): void;
/**
*/
  on_window_change(): void;
/**
*/
  on_window_close(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly create_state: () => number;
  readonly __wbg_state_free: (a: number, b: number) => void;
  readonly state_load_last_grades: (a: number) => void;
  readonly state_write_grade: (a: number, b: number, c: number) => void;
  readonly state_generate_pdf: (a: number) => void;
  readonly __wbg_mouseeventinterface_free: (a: number, b: number) => void;
  readonly mouseeventinterface_on_mouse_down: (a: number, b: number, c: number) => void;
  readonly mouseeventinterface_on_mouse_up: (a: number, b: number, c: number) => void;
  readonly __wbg_inputeventinterface_free: (a: number, b: number) => void;
  readonly inputeventinterface_on_input: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly inputeventinterface_on_change: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbg_windoweventinterface_free: (a: number, b: number) => void;
  readonly windoweventinterface_on_window_change: (a: number) => void;
  readonly windoweventinterface_on_window_close: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
