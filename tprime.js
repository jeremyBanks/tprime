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

export function __wbg_setText_9d22b8a5cd3ae9c1(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    setText(varg0);
}

export function __wbg_drawLine_6dc70d87830823e3(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getStringFromWasm(arg0, arg1);
    
    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);
    
    drawLine(varg0, arg2, arg3, arg4, arg5);
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
* The root application state.
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
    * A new frame!
    * @returns {void}
    */
    tick() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        return wasm.application_tick(this.ptr);
    }
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

