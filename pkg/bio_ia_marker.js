let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
* @returns {State}
*/
export function create_state() {
    const ret = wasm.create_state();
    return State.__wrap(ret);
}

const InputEventInterfaceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_inputeventinterface_free(ptr >>> 0, 1));
/**
*/
export class InputEventInterface {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InputEventInterface.prototype);
        obj.__wbg_ptr = ptr;
        InputEventInterfaceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        InputEventInterfaceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_inputeventinterface_free(ptr, 0);
    }
    /**
    * @param {string} id
    * @param {string} data
    */
    on_input(id, data) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.inputeventinterface_on_input(this.__wbg_ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * @param {string} id
    * @param {string} data
    */
    on_change(id, data) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(data, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.inputeventinterface_on_change(this.__wbg_ptr, ptr0, len0, ptr1, len1);
    }
}

const MouseEventInterfaceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_mouseeventinterface_free(ptr >>> 0, 1));
/**
*/
export class MouseEventInterface {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MouseEventInterface.prototype);
        obj.__wbg_ptr = ptr;
        MouseEventInterfaceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MouseEventInterfaceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mouseeventinterface_free(ptr, 0);
    }
    /**
    * @param {string} id
    */
    on_mouse_down(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.mouseeventinterface_on_mouse_down(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @param {string} id
    */
    on_mouse_up(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.mouseeventinterface_on_mouse_up(this.__wbg_ptr, ptr0, len0);
    }
}

const StateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_state_free(ptr >>> 0, 1));
/**
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
        wasm.__wbg_state_free(ptr, 0);
    }
    /**
    */
    load_last_grades() {
        wasm.state_load_last_grades(this.__wbg_ptr);
    }
    /**
    * @param {number} index
    * @param {number} val
    */
    write_grade(index, val) {
        wasm.state_write_grade(this.__wbg_ptr, index, val);
    }
    /**
    */
    generate_pdf() {
        wasm.state_generate_pdf(this.__wbg_ptr);
    }
}

const WindowEventInterfaceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_windoweventinterface_free(ptr >>> 0, 1));
/**
*/
export class WindowEventInterface {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WindowEventInterface.prototype);
        obj.__wbg_ptr = ptr;
        WindowEventInterfaceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WindowEventInterfaceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_windoweventinterface_free(ptr, 0);
    }
    /**
    */
    on_window_change() {
        wasm.windoweventinterface_on_window_change(this.__wbg_ptr);
    }
    /**
    */
    on_window_close() {
        wasm.windoweventinterface_on_window_close(this.__wbg_ptr);
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
    imports.wbg.__wbg_externrestyle_b2c77ed9a7aac6a1 = function(arg0, arg1, arg2, arg3) {
        const ret = extern_restyle(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_externsetinputvalue_da6ce274971978fe = function(arg0, arg1, arg2, arg3) {
        extern_set_input_value(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_confirm_b187edba0fe32636 = function(arg0, arg1) {
        const ret = confirm(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_savevalbyte_c76065a010dfa640 = function(arg0, arg1, arg2) {
        save_val_byte(getStringFromWasm0(arg0, arg1), arg2);
    };
    imports.wbg.__wbg_savevalstring_990c461ff32edb0e = function(arg0, arg1, arg2, arg3) {
        save_val_string(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_externsetcontenttext_371cf77e7775c905 = function(arg0, arg1, arg2, arg3) {
        extern_set_content_text(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_log_4adf65d93d911367 = function(arg0, arg1) {
        log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_externmoveafter_ce16a03a07322a00 = function(arg0, arg1, arg2, arg3) {
        const ret = extern_move_after(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_externcreatechild_3801afe6da02bcb6 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        const ret = extern_create_child(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5));
        return ret;
    };
    imports.wbg.__wbg_crash_7353361dac912c46 = function(arg0, arg1) {
        crash(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_checkcomponentexists_8887969b21ab0d1e = function(arg0, arg1) {
        const ret = check_component_exists(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_externlistenformouse_51674330d642b80e = function(arg0, arg1, arg2) {
        extern_listen_for_mouse(getStringFromWasm0(arg0, arg1), MouseEventInterface.__wrap(arg2));
    };
    imports.wbg.__wbg_externlistenforinput_5e402ba478e88eab = function(arg0, arg1, arg2) {
        extern_listen_for_input(getStringFromWasm0(arg0, arg1), InputEventInterface.__wrap(arg2));
    };
    imports.wbg.__wbg_externlistenforwindowevents_41fb3fe95e7b2f04 = function(arg0) {
        extern_listen_for_window_events(WindowEventInterface.__wrap(arg0));
    };
    imports.wbg.__wbg_readvalbyte_464a08e0d6f73196 = function(arg0, arg1) {
        const ret = read_val_byte(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_readvalstring_451c076fd2574ef4 = function(arg0, arg1, arg2) {
        const ret = read_val_string(getStringFromWasm0(arg1, arg2));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_externsimulateclick_222a123d420bf67c = function(arg0, arg1) {
        extern_simulate_click(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_alert_3ffb8e08c4291997 = function(arg0, arg1) {
        alert(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_savepdf_efe1fddb6be83057 = function(arg0, arg1) {
        var v0 = getArrayU8FromWasm0(arg0, arg1).slice();
        wasm.__wbindgen_free(arg0, arg1 * 1, 1);
        save_pdf(v0);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;



    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined' && Object.getPrototypeOf(module) === Object.prototype)
    ({module} = module)
    else
    console.warn('using deprecated parameters for `initSync()`; pass a single object instead')

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined' && Object.getPrototypeOf(module_or_path) === Object.prototype)
    ({module_or_path} = module_or_path)
    else
    console.warn('using deprecated parameters for the initialization function; pass a single object instead')

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('bio_ia_marker_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
