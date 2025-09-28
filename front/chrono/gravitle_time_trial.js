let wasm;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
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

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
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

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

export function setup() {
    wasm.setup();
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
/**
 * @param {Point} a
 * @param {Point} b
 * @returns {WrapAroundResult}
 */
export function wrap_around(a, b) {
    _assertClass(a, Point);
    var ptr0 = a.__destroy_into_raw();
    _assertClass(b, Point);
    var ptr1 = b.__destroy_into_raw();
    const ret = wasm.wrap_around(ptr0, ptr1);
    return WrapAroundResult.__wrap(ret);
}

const CellFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cell_free(ptr >>> 0, 1));

export class Cell {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Cell.prototype);
        obj.__wbg_ptr = ptr;
        CellFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CellFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cell_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get idx() {
        const ret = wasm.__wbg_get_cell_idx(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set idx(arg0) {
        wasm.__wbg_set_cell_idx(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get diameter() {
        const ret = wasm.__wbg_get_cell_diameter(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set diameter(arg0) {
        wasm.__wbg_set_cell_diameter(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Point}
     */
    get p() {
        const ret = wasm.__wbg_get_cell_p(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set p(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cell_p(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Point}
     */
    get pp() {
        const ret = wasm.__wbg_get_cell_pp(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set pp(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cell_pp(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Point}
     */
    get np() {
        const ret = wasm.__wbg_get_cell_np(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set np(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cell_np(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Point}
     */
    get dp() {
        const ret = wasm.__wbg_get_cell_dp(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set dp(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cell_dp(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Point}
     */
    get direction() {
        const ret = wasm.__wbg_get_cell_direction(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set direction(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cell_direction(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Point}
     */
    get dv() {
        const ret = wasm.__wbg_get_cell_dv(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set dv(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cell_dv(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Point}
     */
    get link_response() {
        const ret = wasm.__wbg_get_cell_link_response(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set link_response(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cell_link_response(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Point}
     */
    get collision_response() {
        const ret = wasm.__wbg_get_cell_collision_response(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set collision_response(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cell_collision_response(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {number}
     */
    get collision_response_count() {
        const ret = wasm.__wbg_get_cell_collision_response_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set collision_response_count(arg0) {
        wasm.__wbg_set_cell_collision_response_count(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get activated() {
        const ret = wasm.__wbg_get_cell_activated(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set activated(arg0) {
        wasm.__wbg_set_cell_activated(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get activated_previous() {
        const ret = wasm.__wbg_get_cell_activated_previous(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set activated_previous(arg0) {
        wasm.__wbg_set_cell_activated_previous(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get kind() {
        const ret = wasm.__wbg_get_cell_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set kind(arg0) {
        wasm.__wbg_set_cell_kind(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    set_position(x, y) {
        wasm.cell_set_position(this.__wbg_ptr, x, y);
    }
    /**
     * @returns {number}
     */
    static size() {
        const ret = wasm.cell_size();
        return ret >>> 0;
    }
    /**
     * @param {number} idx
     * @param {number} diameter
     * @param {number} kind
     * @returns {Cell}
     */
    static new(idx, diameter, kind) {
        const ret = wasm.cell_new(idx, diameter, kind);
        return Cell.__wrap(ret);
    }
}
if (Symbol.dispose) Cell.prototype[Symbol.dispose] = Cell.prototype.free;

const LinkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_link_free(ptr >>> 0, 1));

export class Link {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
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
    get idx() {
        const ret = wasm.__wbg_get_link_idx(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set idx(arg0) {
        wasm.__wbg_set_link_idx(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get a() {
        const ret = wasm.__wbg_get_link_a(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set a(arg0) {
        wasm.__wbg_set_link_a(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get b() {
        const ret = wasm.__wbg_get_link_b(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set b(arg0) {
        wasm.__wbg_set_link_b(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get au() {
        const ret = wasm.__wbg_get_link_au(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set au(arg0) {
        wasm.__wbg_set_link_au(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get bu() {
        const ret = wasm.__wbg_get_link_bu(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set bu(arg0) {
        wasm.__wbg_set_link_bu(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    static size() {
        const ret = wasm.link_size();
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
}
if (Symbol.dispose) Link.prototype[Symbol.dispose] = Link.prototype.free;

const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr >>> 0, 1));

export class Point {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
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
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_point_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_point_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_point_y(this.__wbg_ptr, arg0);
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
    reset() {
        wasm.point_reset(this.__wbg_ptr);
    }
    /**
     * @returns {Point}
     */
    normalize() {
        const ret = wasm.point_normalize(this.__wbg_ptr);
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
    normalized() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.point_normalized(ptr);
        return Point.__wrap(ret);
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
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

const WorldFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_world_free(ptr >>> 0, 1));

export class World {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
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
    get victory() {
        const ret = wasm.__wbg_get_world_victory(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set victory(arg0) {
        wasm.__wbg_set_world_victory(this.__wbg_ptr, arg0);
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
     * @returns {number | undefined}
     */
    get victory_duration() {
        const ret = wasm.__wbg_get_world_victory_duration(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set victory_duration(arg0) {
        wasm.__wbg_set_world_victory_duration(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
    /**
     * @returns {number | undefined}
     */
    get victory_end() {
        const ret = wasm.__wbg_get_world_victory_end(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set victory_end(arg0) {
        wasm.__wbg_set_world_victory_end(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
    /**
     * @returns {number | undefined}
     */
    get move_start() {
        const ret = wasm.__wbg_get_world_move_start(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set move_start(arg0) {
        wasm.__wbg_set_world_move_start(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
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
     * @param {number} a
     * @param {number} b
     * @returns {number}
     */
    add_link(a, b) {
        const ret = wasm.world_add_link(this.__wbg_ptr, a, b);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    cells_count() {
        const ret = wasm.world_cells_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    cells() {
        const ret = wasm.world_cells(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} idx
     * @param {number} diameter
     */
    set_cell_diameter(idx, diameter) {
        wasm.world_set_cell_diameter(this.__wbg_ptr, idx, diameter);
    }
    /**
     * @param {number} idx
     * @param {number} y
     */
    set_cell_dp_y(idx, y) {
        wasm.world_set_cell_dp_y(this.__wbg_ptr, idx, y);
    }
    /**
     * @param {number} idx
     * @param {number} x
     */
    set_cell_dp_x(idx, x) {
        wasm.world_set_cell_dp_x(this.__wbg_ptr, idx, x);
    }
    /**
     * @param {number} idx
     * @param {number} y
     */
    set_cell_direction_y(idx, y) {
        wasm.world_set_cell_direction_y(this.__wbg_ptr, idx, y);
    }
    /**
     * @param {number} idx
     * @param {number} x
     */
    set_cell_direction_x(idx, x) {
        wasm.world_set_cell_direction_x(this.__wbg_ptr, idx, x);
    }
    /**
     * @param {number} idx
     * @param {number} y
     */
    set_cell_np_y(idx, y) {
        wasm.world_set_cell_np_y(this.__wbg_ptr, idx, y);
    }
    /**
     * @param {number} idx
     * @param {number} x
     */
    set_cell_np_x(idx, x) {
        wasm.world_set_cell_np_x(this.__wbg_ptr, idx, x);
    }
    /**
     * @param {number} idx
     * @param {number} y
     */
    set_cell_pp_y(idx, y) {
        wasm.world_set_cell_pp_y(this.__wbg_ptr, idx, y);
    }
    /**
     * @param {number} idx
     * @param {number} x
     */
    set_cell_pp_x(idx, x) {
        wasm.world_set_cell_pp_x(this.__wbg_ptr, idx, x);
    }
    /**
     * @param {number} idx
     * @param {number} y
     */
    set_cell_position_y(idx, y) {
        wasm.world_set_cell_position_y(this.__wbg_ptr, idx, y);
    }
    /**
     * @param {number} idx
     * @param {number} x
     */
    set_cell_position_x(idx, x) {
        wasm.world_set_cell_position_x(this.__wbg_ptr, idx, x);
    }
    /**
     * @param {number} idx
     * @param {number} kind
     */
    set_cell_kind(idx, kind) {
        wasm.world_set_cell_kind(this.__wbg_ptr, idx, kind);
    }
    /**
     * @param {number} idx
     */
    switch_cell_activated(idx) {
        wasm.world_switch_cell_activated(this.__wbg_ptr, idx);
    }
    /**
     * @param {number} idx
     * @param {number} activated
     */
    set_cell_activated(idx, activated) {
        wasm.world_set_cell_activated(this.__wbg_ptr, idx, activated);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} diameter
     * @param {number} kind
     * @returns {number}
     */
    add_cell(x, y, diameter, kind) {
        const ret = wasm.world_add_cell(this.__wbg_ptr, x, y, diameter, kind);
        return ret >>> 0;
    }
    /**
     * @returns {string}
     */
    get_activation_events() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.world_get_activation_events(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    update_04() {
        wasm.world_update_04(this.__wbg_ptr);
    }
    update_03() {
        wasm.world_update_03(this.__wbg_ptr);
    }
    update_02() {
        wasm.world_update_02(this.__wbg_ptr);
    }
    update_01() {
        wasm.world_update_01(this.__wbg_ptr);
    }
    run_step() {
        wasm.world_run_step(this.__wbg_ptr);
    }
    /**
     * @param {number} aidx
     * @param {number} bidx
     * @returns {boolean}
     */
    link_exists(aidx, bidx) {
        const ret = wasm.world_link_exists(this.__wbg_ptr, aidx, bidx);
        return ret !== 0;
    }
    /**
     * @returns {World}
     */
    static new() {
        const ret = wasm.world_new();
        return World.__wrap(ret);
    }
}
if (Symbol.dispose) World.prototype[Symbol.dispose] = World.prototype.free;

const WrapAroundResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wraparoundresult_free(ptr >>> 0, 1));

export class WrapAroundResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WrapAroundResult.prototype);
        obj.__wbg_ptr = ptr;
        WrapAroundResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WrapAroundResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wraparoundresult_free(ptr, 0);
    }
    /**
     * @returns {Point}
     */
    get a() {
        const ret = wasm.__wbg_get_wraparoundresult_a(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set a(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wraparoundresult_a(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Point}
     */
    get b() {
        const ret = wasm.__wbg_get_wraparoundresult_b(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set b(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wraparoundresult_b(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {number}
     */
    get d_sqrd() {
        const ret = wasm.__wbg_get_wraparoundresult_d_sqrd(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set d_sqrd(arg0) {
        wasm.__wbg_set_wraparoundresult_d_sqrd(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) WrapAroundResult.prototype[Symbol.dispose] = WrapAroundResult.prototype.free;

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_wbindgenthrow_451ec1a8469d7eb6 = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_3;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
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


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

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


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('gravitle_time_trial_bg.wasm', import.meta.url);
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
