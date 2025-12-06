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

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}
/**
 * @returns {World}
 */
export function setup() {
    const ret = wasm.setup();
    return World.__wrap(ret);
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
    get ap() {
        const ret = wasm.__wbg_get_cell_ap(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set ap(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_cell_ap(this.__wbg_ptr, ptr0);
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
     * @returns {number}
     */
    get material_idx() {
        const ret = wasm.__wbg_get_cell_material_idx(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set material_idx(arg0) {
        wasm.__wbg_set_cell_material_idx(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get mass() {
        const ret = wasm.__wbg_get_cell_mass(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set mass(arg0) {
        wasm.__wbg_set_cell_mass(this.__wbg_ptr, arg0);
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
     * @returns {number}
     */
    get fixed() {
        const ret = wasm.__wbg_get_cell_fixed(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set fixed(arg0) {
        wasm.__wbg_set_cell_fixed(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get collision_count() {
        const ret = wasm.__wbg_get_cell_collision_count(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set collision_count(arg0) {
        wasm.__wbg_set_cell_collision_count(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get padding() {
        const ret = wasm.__wbg_get_cell_padding(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set padding(arg0) {
        wasm.__wbg_set_cell_padding(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    static size() {
        const ret = wasm.cell_size();
        return ret >>> 0;
    }
    /**
     * @param {number} material_idx
     * @param {number} x
     * @param {number} y
     * @param {number} diameter
     * @returns {Cell}
     */
    static new(material_idx, x, y, diameter) {
        const ret = wasm.cell_new(material_idx, x, y, diameter);
        return Cell.__wrap(ret);
    }
}
if (Symbol.dispose) Cell.prototype[Symbol.dispose] = Cell.prototype.free;

const ColorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_color_free(ptr >>> 0, 1));

export class Color {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Color.prototype);
        obj.__wbg_ptr = ptr;
        ColorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ColorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_color_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get r() {
        const ret = wasm.__wbg_get_color_r(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set r(arg0) {
        wasm.__wbg_set_color_r(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get g() {
        const ret = wasm.__wbg_get_color_g(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set g(arg0) {
        wasm.__wbg_set_color_g(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get b() {
        const ret = wasm.__wbg_get_color_b(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set b(arg0) {
        wasm.__wbg_set_color_b(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Color.prototype[Symbol.dispose] = Color.prototype.free;

const LinkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_link_free(ptr >>> 0, 1));

export class Link {

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
    static size() {
        const ret = wasm.link_size();
        return ret >>> 0;
    }
}
if (Symbol.dispose) Link.prototype[Symbol.dispose] = Link.prototype.free;

const MaterialFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_material_free(ptr >>> 0, 1));

export class Material {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MaterialFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_material_free(ptr, 0);
    }
    /**
     * @returns {Color}
     */
    get color() {
        const ret = wasm.__wbg_get_material_color(this.__wbg_ptr);
        return Color.__wrap(ret);
    }
    /**
     * @param {Color} arg0
     */
    set color(arg0) {
        _assertClass(arg0, Color);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_material_color(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {number}
     */
    get density() {
        const ret = wasm.__wbg_get_material_density(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set density(arg0) {
        wasm.__wbg_set_material_density(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    static size() {
        const ret = wasm.material_size();
        return ret >>> 0;
    }
}
if (Symbol.dispose) Material.prototype[Symbol.dispose] = Material.prototype.free;

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
     * @returns {number}
     */
    static size() {
        const ret = wasm.point_size();
        return ret >>> 0;
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
     * @returns {Point}
     */
    normalize() {
        const ret = wasm.point_normalize(this.__wbg_ptr);
        return Point.__wrap(ret);
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
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

const StatJsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_statjs_free(ptr >>> 0, 1));

export class StatJs {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StatJs.prototype);
        obj.__wbg_ptr = ptr;
        StatJsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StatJsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_statjs_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get avg() {
        const ret = wasm.__wbg_get_statjs_avg(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set avg(arg0) {
        wasm.__wbg_set_statjs_avg(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get p99() {
        const ret = wasm.__wbg_get_statjs_p99(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set p99(arg0) {
        wasm.__wbg_set_statjs_p99(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) StatJs.prototype[Symbol.dispose] = StatJs.prototype.free;

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
     * @returns {WorldConfig}
     */
    get c() {
        const ret = wasm.__wbg_get_world_c(this.__wbg_ptr);
        return WorldConfig.__wrap(ret);
    }
    /**
     * @param {WorldConfig} arg0
     */
    set c(arg0) {
        _assertClass(arg0, WorldConfig);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_world_c(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {number}
     */
    get perf_array_len() {
        const ret = wasm.__wbg_get_world_perf_array_len(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set perf_array_len(arg0) {
        wasm.__wbg_set_world_perf_array_len(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get collision_count() {
        const ret = wasm.__wbg_get_world_collision_count(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set collision_count(arg0) {
        wasm.__wbg_set_world_collision_count(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get pairs_count() {
        const ret = wasm.__wbg_get_world_pairs_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set pairs_count(arg0) {
        wasm.__wbg_set_world_pairs_count(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get zones_count() {
        const ret = wasm.__wbg_get_world_zones_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set zones_count(arg0) {
        wasm.__wbg_set_world_zones_count(this.__wbg_ptr, arg0);
    }
    /**
     * @param {string} blueprint_str
     * @param {number} x
     * @param {number} y
     */
    add_from_blueprint(blueprint_str, x, y) {
        const ptr0 = passStringToWasm0(blueprint_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.world_add_from_blueprint(this.__wbg_ptr, ptr0, len0, x, y);
    }
    /**
     * @param {string} url
     * @param {string} definition
     */
    add_material(url, definition) {
        const ptr0 = passStringToWasm0(url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(definition, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.world_add_material(this.__wbg_ptr, ptr0, len0, ptr1, len1);
    }
    /**
     * @returns {number}
     */
    materials_count() {
        const ret = wasm.world_materials_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    materials() {
        const ret = wasm.world_materials(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    positions_count() {
        const ret = wasm.world_positions_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    positions() {
        const ret = wasm.world_positions(this.__wbg_ptr);
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
     * @returns {number}
     */
    links() {
        const ret = wasm.world_links(this.__wbg_ptr);
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
     * @param {string} material_url
     * @param {number} idx_1
     * @param {number} idx_2
     * @param {number} diameter
     * @returns {number}
     */
    add_cell_2(material_url, idx_1, idx_2, diameter) {
        const ptr0 = passStringToWasm0(material_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.world_add_cell_2(this.__wbg_ptr, ptr0, len0, idx_1, idx_2, diameter);
        return ret >>> 0;
    }
    /**
     * @param {string} material_url
     * @param {number} idx
     * @param {number} diameter
     * @returns {number}
     */
    add_cell_left(material_url, idx, diameter) {
        const ptr0 = passStringToWasm0(material_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.world_add_cell_left(this.__wbg_ptr, ptr0, len0, idx, diameter);
        return ret >>> 0;
    }
    /**
     * @param {string} material_url
     * @param {number} idx
     * @param {number} diameter
     * @returns {number}
     */
    add_cell_right(material_url, idx, diameter) {
        const ptr0 = passStringToWasm0(material_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.world_add_cell_right(this.__wbg_ptr, ptr0, len0, idx, diameter);
        return ret >>> 0;
    }
    /**
     * @param {string} material_url
     * @param {number} idx
     * @param {number} diameter
     * @returns {number}
     */
    add_cell_down(material_url, idx, diameter) {
        const ptr0 = passStringToWasm0(material_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.world_add_cell_down(this.__wbg_ptr, ptr0, len0, idx, diameter);
        return ret >>> 0;
    }
    /**
     * @param {string} material_url
     * @param {number} idx
     * @param {number} diameter
     * @returns {number}
     */
    add_cell_up(material_url, idx, diameter) {
        const ptr0 = passStringToWasm0(material_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.world_add_cell_up(this.__wbg_ptr, ptr0, len0, idx, diameter);
        return ret >>> 0;
    }
    /**
     * @param {string} material_url
     * @param {number} x
     * @param {number} y
     * @param {number} diameter
     * @returns {number}
     */
    add_cell(material_url, x, y, diameter) {
        const ptr0 = passStringToWasm0(material_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.world_add_cell(this.__wbg_ptr, ptr0, len0, x, y, diameter);
        return ret >>> 0;
    }
    /**
     * @param {number} idx
     */
    set_cell_fixed(idx) {
        wasm.world_set_cell_fixed(this.__wbg_ptr, idx);
    }
    /**
     * @param {number} idx
     * @param {number} diameter
     */
    set_cell_diameter(idx, diameter) {
        wasm.world_set_cell_diameter(this.__wbg_ptr, idx, diameter);
    }
    tick_06() {
        wasm.world_tick_06(this.__wbg_ptr);
    }
    tick_05() {
        wasm.world_tick_05(this.__wbg_ptr);
    }
    tick_04() {
        wasm.world_tick_04(this.__wbg_ptr);
    }
    tick_03() {
        wasm.world_tick_03(this.__wbg_ptr);
    }
    tick_02() {
        wasm.world_tick_02(this.__wbg_ptr);
    }
    tick_01() {
        wasm.world_tick_01(this.__wbg_ptr);
    }
    /**
     * @param {string} id
     * @returns {StatJs}
     */
    get_stats(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.world_get_stats(this.__wbg_ptr, ptr0, len0);
        return StatJs.__wrap(ret);
    }
    /**
     * @param {string} id
     */
    add_stat(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.world_add_stat(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} id
     * @param {number} value
     */
    add_duration(id, value) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.world_add_duration(this.__wbg_ptr, ptr0, len0, value);
    }
    tick_events() {
        wasm.world_tick_events(this.__wbg_ptr);
    }
    tick_links() {
        wasm.world_tick_links(this.__wbg_ptr);
    }
    tick_save_positions() {
        wasm.world_tick_save_positions(this.__wbg_ptr);
    }
    /**
     * @param {number} idx
     * @returns {Point[]}
     */
    get_positions(idx) {
        const ret = wasm.world_get_positions(this.__wbg_ptr, idx);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    tick() {
        wasm.world_tick(this.__wbg_ptr);
    }
    /**
     * @param {number} n
     */
    tick_n(n) {
        wasm.world_tick_n(this.__wbg_ptr, n);
    }
    /**
     * @param {number} value
     */
    set_link_strength_dv(value) {
        wasm.world_set_link_strength_dv(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set_link_strength_dp(value) {
        wasm.world_set_link_strength_dp(this.__wbg_ptr, value);
    }
    /**
     * @param {boolean} value
     */
    set_c2c_colision(value) {
        wasm.world_set_c2c_colision(this.__wbg_ptr, value);
    }
    /**
     * @param {boolean} value
     */
    set_c2c_gravity(value) {
        wasm.world_set_c2c_gravity(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set_crdv(value) {
        wasm.world_set_crdv(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set_crdp(value) {
        wasm.world_set_crdp(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set_rdv(value) {
        wasm.world_set_rdv(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set_rdp(value) {
        wasm.world_set_rdp(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set_gravity(value) {
        wasm.world_set_gravity(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set_gravity_2(value) {
        wasm.world_set_gravity_2(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set_zonesize(value) {
        wasm.world_set_zonesize(this.__wbg_ptr, value);
    }
    /**
     * @param {number} a
     * @param {number} b
     */
    add_link(a, b) {
        wasm.world_add_link(this.__wbg_ptr, a, b);
    }
    /**
     * @param {number} idx
     */
    save_positions(idx) {
        wasm.world_save_positions(this.__wbg_ptr, idx);
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
    get_tick() {
        const ret = wasm.world_get_tick(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} tick
     * @param {string} func
     * @param {number} cell_id
     * @param {number} value
     */
    add_event(tick, func, cell_id, value) {
        const ptr0 = passStringToWasm0(func, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.world_add_event(this.__wbg_ptr, tick, ptr0, len0, cell_id, value);
    }
}
if (Symbol.dispose) World.prototype[Symbol.dispose] = World.prototype.free;

const WorldConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_worldconfig_free(ptr >>> 0, 1));

export class WorldConfig {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WorldConfig.prototype);
        obj.__wbg_ptr = ptr;
        WorldConfigFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WorldConfigFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_worldconfig_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get rdp() {
        const ret = wasm.__wbg_get_statjs_avg(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set rdp(arg0) {
        wasm.__wbg_set_statjs_avg(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get rdv() {
        const ret = wasm.__wbg_get_statjs_p99(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set rdv(arg0) {
        wasm.__wbg_set_statjs_p99(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get zonesize() {
        const ret = wasm.__wbg_get_worldconfig_zonesize(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set zonesize(arg0) {
        wasm.__wbg_set_worldconfig_zonesize(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get gravity() {
        const ret = wasm.__wbg_get_worldconfig_gravity(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set gravity(arg0) {
        wasm.__wbg_set_worldconfig_gravity(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get gravity_2() {
        const ret = wasm.__wbg_get_worldconfig_gravity_2(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set gravity_2(arg0) {
        wasm.__wbg_set_worldconfig_gravity_2(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get crdp() {
        const ret = wasm.__wbg_get_worldconfig_crdp(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set crdp(arg0) {
        wasm.__wbg_set_worldconfig_crdp(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get crdv() {
        const ret = wasm.__wbg_get_worldconfig_crdv(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set crdv(arg0) {
        wasm.__wbg_set_worldconfig_crdv(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get rdv_during_colision() {
        const ret = wasm.__wbg_get_worldconfig_rdv_during_colision(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set rdv_during_colision(arg0) {
        wasm.__wbg_set_worldconfig_rdv_during_colision(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get c2c_gravity() {
        const ret = wasm.__wbg_get_worldconfig_c2c_gravity(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set c2c_gravity(arg0) {
        wasm.__wbg_set_worldconfig_c2c_gravity(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get c2c_colision() {
        const ret = wasm.__wbg_get_worldconfig_c2c_colision(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set c2c_colision(arg0) {
        wasm.__wbg_set_worldconfig_c2c_colision(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get link_strength_dp() {
        const ret = wasm.__wbg_get_worldconfig_link_strength_dp(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set link_strength_dp(arg0) {
        wasm.__wbg_set_worldconfig_link_strength_dp(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get link_strength_dv() {
        const ret = wasm.__wbg_get_worldconfig_link_strength_dv(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set link_strength_dv(arg0) {
        wasm.__wbg_set_worldconfig_link_strength_dv(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) WorldConfig.prototype[Symbol.dispose] = WorldConfig.prototype.free;

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
    imports.wbg.__wbg___wbindgen_throw_b855445ff6a94295 = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
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
    imports.wbg.__wbg_log_d6fe57f38f390f5c = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_now_793306c526e2e3b6 = function() {
        const ret = Date.now();
        return ret;
    };
    imports.wbg.__wbg_point_new = function(arg0) {
        const ret = Point.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_externrefs;
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
        module_or_path = new URL('gravitle_slingshot_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
