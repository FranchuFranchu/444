let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const InterfaceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_interface_free(ptr >>> 0));
/**
*/
export class Interface {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InterfaceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_interface_free(ptr);
    }
    /**
    */
    constructor() {
        const ret = wasm.interface_new();
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {number}
    */
    history_len() {
        const ret = wasm.interface_history_len(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {State}
    */
    get_last() {
        const ret = wasm.interface_get_last(this.__wbg_ptr);
        return State.__wrap(ret);
    }
    /**
    * @returns {boolean}
    */
    player_id() {
        const ret = wasm.interface_player_id(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    */
    play_ai() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.interface_play_ai(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    play_at(x, y) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.interface_play_at(retptr, this.__wbg_ptr, x, y);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {State | undefined}
    */
    undo() {
        const ret = wasm.interface_undo(this.__wbg_ptr);
        return ret === 0 ? undefined : State.__wrap(ret);
    }
    /**
    * @param {number} len
    */
    undo_to(len) {
        wasm.interface_undo_to(this.__wbg_ptr, len);
    }
    /**
    * @param {number} step
    * @returns {State | undefined}
    */
    peek(step) {
        const ret = wasm.interface_peek(this.__wbg_ptr, step);
        return ret === 0 ? undefined : State.__wrap(ret);
    }
    /**
    * @returns {WinResult | undefined}
    */
    winner() {
        const ret = wasm.interface_winner(this.__wbg_ptr);
        return ret === 0 ? undefined : WinResult.__wrap(ret);
    }
}

const StateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_state_free(ptr >>> 0));
/**
* Layout:
* [     z = 3    ][     z = 2    ][     z = 1    ][     z = 0    ] z
* [y3][y2][y1][y0][y3][y2][y1][y0][y3][y2][y1][y0][y3][y2][y1][y0] y
* 3210321032103210321032103210321032103210321032103210321032103210 x
*/
export class State {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(State.prototype);
        obj.__wbg_ptr = ptr;
        StateFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StateFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_state_free(ptr);
    }
    /**
    * @returns {bigint}
    */
    get 0() {
        const ret = wasm.__wbg_get_state_0(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @param {bigint} arg0
    */
    set 0(arg0) {
        wasm.__wbg_set_state_0(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {bigint}
    */
    get 1() {
        const ret = wasm.__wbg_get_state_1(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @param {bigint} arg0
    */
    set 1(arg0) {
        wasm.__wbg_set_state_1(this.__wbg_ptr, arg0);
    }
    /**
    */
    constructor() {
        const ret = wasm.state_empty();
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {State}
    */
    wasm_clone() {
        const ret = wasm.state_wasm_clone(this.__wbg_ptr);
        return State.__wrap(ret);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @param {number} z
    * @returns {boolean | undefined}
    */
    get_at(x, y, z) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.state_get_at(ptr, x, y, z);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
    * @param {number} x
    * @param {number} y
    * @param {number} z
    * @param {boolean | undefined} [val]
    * @returns {State}
    */
    set_at(x, y, z, val) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.state_set_at(ptr, x, y, z, isLikeNone(val) ? 0xFFFFFF : val ? 1 : 0);
        return State.__wrap(ret);
    }
}

const WinResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_winresult_free(ptr >>> 0));
/**
*/
export class WinResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WinResult.prototype);
        obj.__wbg_ptr = ptr;
        WinResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WinResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_winresult_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    get 0() {
        const ret = wasm.__wbg_get_winresult_0(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set 0(arg0) {
        wasm.__wbg_set_winresult_0(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {State}
    */
    get 1() {
        const ret = wasm.__wbg_get_winresult_1(this.__wbg_ptr);
        return State.__wrap(ret);
    }
    /**
    * @param {State} arg0
    */
    set 1(arg0) {
        _assertClass(arg0, State);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_winresult_1(this.__wbg_ptr, ptr0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('cuatro_cuatro_cuatro_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
