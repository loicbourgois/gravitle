/* @ts-self-types="./engine.d.ts" */

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11}
 */
export const Kind = Object.freeze({
    Moon: 0, "0": "Moon",
    Pipe: 1, "1": "Pipe",
    Tank: 2, "2": "Tank",
    Filter: 3, "3": "Filter",
    Pump: 4, "4": "Pump",
    SolarPanel: 5, "5": "SolarPanel",
    Battery: 6, "6": "Battery",
    Wire: 7, "7": "Wire",
    Switch: 8, "8": "Switch",
    Led: 9, "9": "Led",
    Blower: 10, "10": "Blower",
    Valve: 11, "11": "Valve",
});

export class Link {
    static __wrap(ptr) {
        const obj = Object.create(Link.prototype);
        obj.__wbg_ptr = ptr;
        LinkFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LinkFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_link_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get a() {
        const ret = wasm.__wbg_get_link_a(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get b() {
        const ret = wasm.__wbg_get_link_b(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get idx() {
        const ret = wasm.__wbg_get_link_idx(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} idx
     * @param {number} a
     * @param {number} b
     * @returns {Link}
     */
    static new(idx, a, b) {
        const ret = wasm.link_new(idx, a, b);
        return Link.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    static size() {
        const ret = wasm.link_size();
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set a(arg0) {
        wasm.__wbg_set_link_a(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set b(arg0) {
        wasm.__wbg_set_link_b(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set idx(arg0) {
        wasm.__wbg_set_link_idx(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Link.prototype[Symbol.dispose] = Link.prototype.free;

export class Particle {
    static __wrap(ptr) {
        const obj = Object.create(Particle.prototype);
        obj.__wbg_ptr = ptr;
        ParticleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ParticleFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_particle_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get activation() {
        const ret = wasm.__wbg_get_particle_activation(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get collision_response_count() {
        const ret = wasm.__wbg_get_particle_collision_response_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {Point}
     */
    get collision_response() {
        const ret = wasm.__wbg_get_particle_collision_response(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get diameter() {
        const ret = wasm.__wbg_get_particle_diameter(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Point}
     */
    get direction() {
        const ret = wasm.__wbg_get_particle_direction(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {Point}
     */
    get dp() {
        const ret = wasm.__wbg_get_particle_dp(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {Point}
     */
    get dv() {
        const ret = wasm.__wbg_get_particle_dv(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get electricity_c() {
        const ret = wasm.__wbg_get_particle_electricity_c(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get electricity_q() {
        const ret = wasm.__wbg_get_particle_electricity_q(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get electricity_qd() {
        const ret = wasm.__wbg_get_particle_electricity_qd(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get idx() {
        const ret = wasm.__wbg_get_particle_idx(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {Kind}
     */
    get kind() {
        const ret = wasm.__wbg_get_particle_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Point}
     */
    get link_response() {
        const ret = wasm.__wbg_get_particle_link_response(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get moon_c() {
        const ret = wasm.__wbg_get_particle_moon_c(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get moon_q() {
        const ret = wasm.__wbg_get_particle_moon_q(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get moon_qd() {
        const ret = wasm.__wbg_get_particle_moon_qd(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Point}
     */
    get np() {
        const ret = wasm.__wbg_get_particle_np(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {Point}
     */
    get p() {
        const ret = wasm.__wbg_get_particle_p(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get padding() {
        const ret = wasm.__wbg_get_particle_padding(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {Point}
     */
    get pp() {
        const ret = wasm.__wbg_get_particle_pp(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get pressure_c() {
        const ret = wasm.__wbg_get_particle_pressure_c(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get pressure_q() {
        const ret = wasm.__wbg_get_particle_pressure_q(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get pressure_qn() {
        const ret = wasm.__wbg_get_particle_pressure_qn(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} idx
     * @param {Kind} kind
     * @param {number} x
     * @param {number} y
     * @returns {Particle}
     */
    static new(idx, kind, x, y) {
        const ret = wasm.particle_new(idx, kind, x, y);
        return Particle.__wrap(ret);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    set_position(x, y) {
        wasm.particle_set_position(this.__wbg_ptr, x, y);
    }
    /**
     * @returns {number}
     */
    static size() {
        const ret = wasm.particle_size();
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set activation(arg0) {
        wasm.__wbg_set_particle_activation(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set collision_response_count(arg0) {
        wasm.__wbg_set_particle_collision_response_count(this.__wbg_ptr, arg0);
    }
    /**
     * @param {Point} arg0
     */
    set collision_response(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_collision_response(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {number} arg0
     */
    set diameter(arg0) {
        wasm.__wbg_set_particle_diameter(this.__wbg_ptr, arg0);
    }
    /**
     * @param {Point} arg0
     */
    set direction(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_direction(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {Point} arg0
     */
    set dp(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_dp(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {Point} arg0
     */
    set dv(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_dv(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {number} arg0
     */
    set electricity_c(arg0) {
        wasm.__wbg_set_particle_electricity_c(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set electricity_q(arg0) {
        wasm.__wbg_set_particle_electricity_q(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set electricity_qd(arg0) {
        wasm.__wbg_set_particle_electricity_qd(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set idx(arg0) {
        wasm.__wbg_set_particle_idx(this.__wbg_ptr, arg0);
    }
    /**
     * @param {Kind} arg0
     */
    set kind(arg0) {
        wasm.__wbg_set_particle_kind(this.__wbg_ptr, arg0);
    }
    /**
     * @param {Point} arg0
     */
    set link_response(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_link_response(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {number} arg0
     */
    set moon_c(arg0) {
        wasm.__wbg_set_particle_moon_c(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set moon_q(arg0) {
        wasm.__wbg_set_particle_moon_q(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set moon_qd(arg0) {
        wasm.__wbg_set_particle_moon_qd(this.__wbg_ptr, arg0);
    }
    /**
     * @param {Point} arg0
     */
    set np(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_np(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {Point} arg0
     */
    set p(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_p(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {number} arg0
     */
    set padding(arg0) {
        wasm.__wbg_set_particle_padding(this.__wbg_ptr, arg0);
    }
    /**
     * @param {Point} arg0
     */
    set pp(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_pp(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {number} arg0
     */
    set pressure_c(arg0) {
        wasm.__wbg_set_particle_pressure_c(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set pressure_q(arg0) {
        wasm.__wbg_set_particle_pressure_q(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set pressure_qn(arg0) {
        wasm.__wbg_set_particle_pressure_qn(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Particle.prototype[Symbol.dispose] = Particle.prototype.free;

export class Point {
    static __wrap(ptr) {
        const obj = Object.create(Point.prototype);
        obj.__wbg_ptr = ptr;
        PointFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PointFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_point_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_point_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Point} b
     * @returns {number}
     */
    distance(b) {
        const ptr = this.__destroy_into_raw();
        _assertClass(b, Point);
        var ptr0 = b.__destroy_into_raw();
        const ret = wasm.point_distance(ptr, ptr0);
        return ret;
    }
    /**
     * @param {Point} b
     * @returns {number}
     */
    distance_squared_2(b) {
        const ptr = this.__destroy_into_raw();
        _assertClass(b, Point);
        var ptr0 = b.__destroy_into_raw();
        const ret = wasm.point_distance_squared_2(ptr, ptr0);
        return ret;
    }
    /**
     * @param {number} x
     * @param {number} y
     * @returns {number}
     */
    distance_squared(x, y) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.point_distance_squared(ptr, x, y);
        return ret;
    }
    /**
     * @returns {number}
     */
    length() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.point_length(ptr);
        return ret;
    }
    /**
     * @param {number} x
     * @param {number} y
     * @returns {Point}
     */
    static new(x, y) {
        const ret = wasm.point_new(x, y);
        return Point.__wrap(ret);
    }
    /**
     * @param {number} length
     * @returns {Point}
     */
    normalize_2(length) {
        const ret = wasm.point_normalize_2(this.__wbg_ptr, length);
        return Point.__wrap(ret);
    }
    /**
     * @returns {Point}
     */
    normalize() {
        const ret = wasm.point_normalize(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @returns {Point}
     */
    normalized() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.point_normalized(ptr);
        return Point.__wrap(ret);
    }
    reset() {
        wasm.point_reset(this.__wbg_ptr);
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_point_x(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_point_y(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

export class World {
    static __wrap(ptr) {
        const obj = Object.create(World.prototype);
        obj.__wbg_ptr = ptr;
        WorldFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WorldFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_world_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get step() {
        const ret = wasm.__wbg_get_world_step(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set step(arg0) {
        wasm.__wbg_set_world_step(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} a
     * @param {number} b
     */
    add_link(a, b) {
        wasm.world_add_link(this.__wbg_ptr, a, b);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {Kind} k
     * @returns {number}
     */
    add_particle(x, y, k) {
        const ret = wasm.world_add_particle(this.__wbg_ptr, x, y, k);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    links() {
        const ret = wasm.world_links(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    links_count() {
        const ret = wasm.world_links_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {World}
     */
    static new() {
        const ret = wasm.world_new();
        return World.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    particles() {
        const ret = wasm.world_particles(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    particles_count() {
        const ret = wasm.world_particles_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} pidx
     * @param {number} v
     */
    set_activation(pidx, v) {
        wasm.world_set_activation(this.__wbg_ptr, pidx, v);
    }
    tick() {
        wasm.world_tick(this.__wbg_ptr);
    }
}
if (Symbol.dispose) World.prototype[Symbol.dispose] = World.prototype.free;

/**
 * @param {Kind} k
 * @returns {string}
 */
export function base_color(k) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.base_color(k);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

export function setup() {
    wasm.setup();
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_41e9ee4f547fc59a: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_error_757e9472f8410341: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_log_9cbc4889b2aa9438: function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./engine_bg.js": import0,
    };
}

const LinkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_link_free(ptr, 1));
const ParticleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_particle_free(ptr, 1));
const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr, 1));
const WorldFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_world_free(ptr, 1));

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

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
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (!module.ok) {
            throw new Error(`failed to fetch Wasm: ${module.status} ${module.statusText} fetching '${module.url}'`);
        }

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
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

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('engine_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
