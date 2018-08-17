/* tslint:disable */
import * as wasm from './tprime_bg';

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_setTitle_c103b36468a67b49(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    setTitle(varg0);
}

const stack = [];

const slab = [{ obj: undefined }, { obj: null }, { obj: true }, { obj: false }];

function getObject(idx) {
    if ((idx & 1) === 1) {
        return stack[idx >> 1];
    } else {
        const val = slab[idx >> 1];
        
        return val.obj;
        
    }
}

let slab_next = slab.length;

function dropRef(idx) {
    
    idx = idx >> 1;
    if (idx < 4) return;
    let obj = slab[idx];
    
    obj.cnt -= 1;
    if (obj.cnt > 0) return;
    
    // If we hit 0 then free up our space in the slab
    slab[idx] = slab_next;
    slab_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropRef(idx);
    return ret;
}

const __wbg_error_f2a0bd9ef53b7c1c_target = console.error;

export function __wbg_error_f2a0bd9ef53b7c1c(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getStringFromWasm(arg0, arg1);
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = getStringFromWasm(arg4, arg5);
    __wbg_error_f2a0bd9ef53b7c1c_target(varg0, varg2, varg4);
}

const __wbg_warn_95d0935fb9ff30d1_target = console.warn;

export function __wbg_warn_95d0935fb9ff30d1(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getStringFromWasm(arg0, arg1);
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = getStringFromWasm(arg4, arg5);
    __wbg_warn_95d0935fb9ff30d1_target(varg0, varg2, varg4);
}

const __wbg_info_f618f84201099909_target = console.info;

export function __wbg_info_f618f84201099909(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getStringFromWasm(arg0, arg1);
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = getStringFromWasm(arg4, arg5);
    __wbg_info_f618f84201099909_target(varg0, varg2, varg4);
}

const __wbg_debug_5df5ad4c879e8016_target = console.debug;

export function __wbg_debug_5df5ad4c879e8016(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getStringFromWasm(arg0, arg1);
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = getStringFromWasm(arg4, arg5);
    __wbg_debug_5df5ad4c879e8016_target(varg0, varg2, varg4);
}

const __wbg_now_adfcbf9bd4d7b348_target = Date.now  || function() {
    throw new Error(`wasm-bindgen: Date.now does not exist`);
} ;

export function __wbg_now_adfcbf9bd4d7b348() {
    return __wbg_now_adfcbf9bd4d7b348_target();
}

class ConstructorToken {
    constructor(ptr) {
        this.ptr = ptr;
    }
}
/**
* The root application state, exposed to JavaScript.
*/
export class Application {
    
    static __construct(ptr) {
        return new Application(new ConstructorToken(ptr));
    }
    
    constructor(...args) {
        if (args.length === 1 && args[0] instanceof ConstructorToken) {
            this.ptr = args[0].ptr;
            return;
        }
        
        // This invocation of new will call this constructor with a ConstructorToken
        let instance = Application.new(...args);
        this.ptr = instance.ptr;
    }
    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        wasm.__wbg_application_free(ptr);
    }
    /**
    * Instantitates everything.
    * @returns {Application}
    */
    static new() {
        return Application.__construct(wasm.application_new());
    }
    /**
    * @returns {any}
    */
    tick() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return takeObject(wasm.application_tick(this.ptr));
    }
}

function addHeapObject(obj) {
    if (slab_next === slab.length) slab.push(slab.length + 1);
    const idx = slab_next;
    const next = slab[idx];
    
    slab_next = next;
    
    slab[idx] = { obj, cnt: 1 };
    return idx << 1;
}

export function __wbindgen_json_parse(ptr, len) {
    return addHeapObject(JSON.parse(getStringFromWasm(ptr, len)));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

