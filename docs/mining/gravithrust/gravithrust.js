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

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
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

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6}
 */
export const Action = Object.freeze({
    ResetTarget: 0, "0": "ResetTarget",
    CollectCoal: 1, "1": "CollectCoal",
    DeliverCoal: 2, "2": "DeliverCoal",
    DeliverIronOre: 3, "3": "DeliverIronOre",
    CollectIronOre: 4, "4": "CollectIronOre",
    CollectEnergy: 5, "5": "CollectEnergy",
    DeliverEnergy: 6, "6": "DeliverEnergy",
});
/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8}
 */
export const Condition = Object.freeze({
    CoalStorageFull: 0, "0": "CoalStorageFull",
    CoalStorageEmpty: 1, "1": "CoalStorageEmpty",
    IronOreStorageEmpty: 2, "2": "IronOreStorageEmpty",
    IronOreStorageFull: 3, "3": "IronOreStorageFull",
    EnergyStorageEmpty: 4, "4": "EnergyStorageEmpty",
    EnergyStorageFull: 5, "5": "EnergyStorageFull",
    Random1Per1000: 6, "6": "Random1Per1000",
    Random1Per10: 7, "7": "Random1Per10",
    Random1Per100: 8, "8": "Random1Per100",
});
/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41}
 */
export const Kind = Object.freeze({
    Light: 0, "0": "Light",
    PlasmaRefineryOutput: 1, "1": "PlasmaRefineryOutput",
    Luciole: 2, "2": "Luciole",
    Generator: 3, "3": "Generator",
    CoalDepot: 4, "4": "CoalDepot",
    IceAsteroid: 5, "5": "IceAsteroid",
    Sun: 6, "6": "Sun",
    IronOreCargo: 7, "7": "IronOreCargo",
    IronCollector: 8, "8": "IronCollector",
    Armor: 9, "9": "Armor",
    PlasmaElectroFieldCollector: 10, "10": "PlasmaElectroFieldCollector",
    Flower: 11, "11": "Flower",
    PlasmaRawCollector: 12, "12": "PlasmaRawCollector",
    IronAsteroid: 13, "13": "IronAsteroid",
    IronGangueCollector: 14, "14": "IronGangueCollector",
    EnergyDepot: 15, "15": "EnergyDepot",
    IceCollector: 16, "16": "IceCollector",
    CoalCargo: 17, "17": "CoalCargo",
    Core: 18, "18": "Core",
    Target: 19, "19": "Target",
    ElectroFieldLauncher: 20, "20": "ElectroFieldLauncher",
    PlasmaRefineryInput: 21, "21": "PlasmaRefineryInput",
    SunCore: 22, "22": "SunCore",
    IceMelter: 23, "23": "IceMelter",
    IronOreCollector: 24, "24": "IronOreCollector",
    HeatCollector: 25, "25": "HeatCollector",
    CoalAsteroid: 26, "26": "CoalAsteroid",
    Cargo: 27, "27": "Cargo",
    IceCargo: 28, "28": "IceCargo",
    CoalCollector: 29, "29": "CoalCollector",
    PlasmaCargo: 30, "30": "PlasmaCargo",
    Water: 31, "31": "Water",
    Anchor: 32, "32": "Anchor",
    WaterCollector: 33, "33": "WaterCollector",
    IronFurnace: 34, "34": "IronFurnace",
    Booster: 35, "35": "Booster",
    PlasmaRawDepot: 36, "36": "PlasmaRawDepot",
    IronOreDepot: 37, "37": "IronOreDepot",
    Static: 38, "38": "Static",
    EnergyCollector: 39, "39": "EnergyCollector",
    EnergyCargo: 40, "40": "EnergyCargo",
    Battery: 41, "41": "Battery",
});
/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12}
 */
export const QuantityKind = Object.freeze({
    Invalid: 0, "0": "Invalid",
    None: 1, "1": "None",
    Heat: 2, "2": "Heat",
    Energy: 3, "3": "Energy",
    Matter: 4, "4": "Matter",
    Iron: 5, "5": "Iron",
    Water: 6, "6": "Water",
    Ice: 7, "7": "Ice",
    IronGangue: 8, "8": "IronGangue",
    Coal: 9, "9": "Coal",
    IronOre: 10, "10": "IronOre",
    WaterDroplet: 11, "11": "WaterDroplet",
    Nectar: 12, "12": "Nectar",
});

const BlueprintFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_blueprint_free(ptr >>> 0, 1));

export class Blueprint {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BlueprintFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_blueprint_free(ptr, 0);
    }
}
if (Symbol.dispose) Blueprint.prototype[Symbol.dispose] = Blueprint.prototype.free;

const DurationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_duration_free(ptr >>> 0, 1));

export class Duration {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Duration.prototype);
        obj.__wbg_ptr = ptr;
        DurationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DurationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_duration_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get a() {
        const ret = wasm.__wbg_get_duration_a(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set a(arg0) {
        wasm.__wbg_set_duration_a(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get b() {
        const ret = wasm.__wbg_get_duration_b(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set b(arg0) {
        wasm.__wbg_set_duration_b(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get c() {
        const ret = wasm.__wbg_get_duration_c(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set c(arg0) {
        wasm.__wbg_set_duration_c(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get d() {
        const ret = wasm.__wbg_get_duration_d(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set d(arg0) {
        wasm.__wbg_set_duration_d(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get e() {
        const ret = wasm.__wbg_get_duration_e(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set e(arg0) {
        wasm.__wbg_set_duration_e(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get f() {
        const ret = wasm.__wbg_get_duration_f(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set f(arg0) {
        wasm.__wbg_set_duration_f(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Duration.prototype[Symbol.dispose] = Duration.prototype.free;

const GravithrustFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gravithrust_free(ptr >>> 0, 1));

export class Gravithrust {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Gravithrust.prototype);
        obj.__wbg_ptr = ptr;
        GravithrustFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GravithrustFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gravithrust_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get diameter() {
        const ret = wasm.__wbg_get_gravithrust_diameter(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set diameter(arg0) {
        wasm.__wbg_set_gravithrust_diameter(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get points() {
        const ret = wasm.__wbg_get_gravithrust_points(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set points(arg0) {
        wasm.__wbg_set_gravithrust_points(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get step() {
        const ret = wasm.__wbg_get_gravithrust_step(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set step(arg0) {
        wasm.__wbg_set_gravithrust_step(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get sub_steps() {
        const ret = wasm.__wbg_get_gravithrust_sub_steps(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set sub_steps(arg0) {
        wasm.__wbg_set_gravithrust_sub_steps(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get max_rotation_speed() {
        const ret = wasm.__wbg_get_gravithrust_max_rotation_speed(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set max_rotation_speed(arg0) {
        wasm.__wbg_set_gravithrust_max_rotation_speed(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get max_speed_at_target() {
        const ret = wasm.__wbg_get_gravithrust_max_speed_at_target(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set max_speed_at_target(arg0) {
        wasm.__wbg_set_gravithrust_max_speed_at_target(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get forward_max_speed() {
        const ret = wasm.__wbg_get_gravithrust_forward_max_speed(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set forward_max_speed(arg0) {
        wasm.__wbg_set_gravithrust_forward_max_speed(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get forward_max_angle() {
        const ret = wasm.__wbg_get_gravithrust_forward_max_angle(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set forward_max_angle(arg0) {
        wasm.__wbg_set_gravithrust_forward_max_angle(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get slow_down_max_angle() {
        const ret = wasm.__wbg_get_gravithrust_slow_down_max_angle(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set slow_down_max_angle(arg0) {
        wasm.__wbg_set_gravithrust_slow_down_max_angle(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get slow_down_max_speed_to_target_ratio() {
        const ret = wasm.__wbg_get_gravithrust_slow_down_max_speed_to_target_ratio(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set slow_down_max_speed_to_target_ratio(arg0) {
        wasm.__wbg_set_gravithrust_slow_down_max_speed_to_target_ratio(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get booster_acceleration() {
        const ret = wasm.__wbg_get_gravithrust_booster_acceleration(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set booster_acceleration(arg0) {
        wasm.__wbg_set_gravithrust_booster_acceleration(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Duration}
     */
    get avg_duration() {
        const ret = wasm.__wbg_get_gravithrust_avg_duration(this.__wbg_ptr);
        return Duration.__wrap(ret);
    }
    /**
     * @param {Duration} arg0
     */
    set avg_duration(arg0) {
        _assertClass(arg0, Duration);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_gravithrust_avg_duration(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {number} diameter
     * @param {number} sub_steps
     * @param {number} max_rotation_speed
     * @param {number} grid_side
     * @param {number} max_speed_at_target
     * @param {number} forward_max_speed
     * @param {number} forward_max_angle
     * @param {number} slow_down_max_angle
     * @param {number} slow_down_max_speed_to_target_ratio
     * @param {number} booster_acceleration
     * @returns {Gravithrust}
     */
    static new(diameter, sub_steps, max_rotation_speed, grid_side, max_speed_at_target, forward_max_speed, forward_max_angle, slow_down_max_angle, slow_down_max_speed_to_target_ratio, booster_acceleration) {
        const ret = wasm.gravithrust_new(diameter, sub_steps, max_rotation_speed, grid_side, max_speed_at_target, forward_max_speed, forward_max_angle, slow_down_max_angle, slow_down_max_speed_to_target_ratio, booster_acceleration);
        return Gravithrust.__wrap(ret);
    }
    /**
     * @param {number} pid
     */
    print_particle(pid) {
        wasm.gravithrust_print_particle(this.__wbg_ptr, pid);
    }
    /**
     * @param {number} pid
     * @returns {number}
     */
    get_particle_kind(pid) {
        const ret = wasm.gravithrust_get_particle_kind(this.__wbg_ptr, pid);
        return ret >>> 0;
    }
    /**
     * @param {string} yml_blueprint
     * @param {number} x
     * @param {number} y
     * @returns {number}
     */
    add_ship(yml_blueprint, x, y) {
        const ptr0 = passStringToWasm0(yml_blueprint, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.gravithrust_add_ship(this.__wbg_ptr, ptr0, len0, x, y);
        return ret >>> 0;
    }
    /**
     * @param {string} yml_blueprint
     * @param {number} x
     * @param {number} y
     * @returns {number}
     */
    add_structure(yml_blueprint, x, y) {
        const ptr0 = passStringToWasm0(yml_blueprint, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.gravithrust_add_structure(this.__wbg_ptr, ptr0, len0, x, y);
        return ret >>> 0;
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {string} kind
     * @returns {number}
     */
    add_particle(x, y, kind) {
        const ptr0 = passStringToWasm0(kind, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.gravithrust_add_particle(this.__wbg_ptr, x, y, ptr0, len0);
        return ret >>> 0;
    }
    /**
     * @param {number} sid
     * @param {number} pid
     */
    set_anchor(sid, pid) {
        wasm.gravithrust_set_anchor(this.__wbg_ptr, sid, pid);
    }
    /**
     * @param {number} sid
     * @param {number} pid
     */
    set_target(sid, pid) {
        wasm.gravithrust_set_target(this.__wbg_ptr, sid, pid);
    }
    /**
     * @param {Blueprint} blueprint
     * @param {Vector} position
     * @param {number | null} [sid]
     * @returns {Uint32Array}
     */
    add_structure_internal(blueprint, position, sid) {
        _assertClass(blueprint, Blueprint);
        _assertClass(position, Vector);
        var ptr0 = position.__destroy_into_raw();
        const ret = wasm.gravithrust_add_structure_internal(this.__wbg_ptr, blueprint.__wbg_ptr, ptr0, isLikeNone(sid) ? 0x100000001 : (sid) >>> 0);
        var v2 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v2;
    }
    /**
     * @param {Blueprint} blueprint
     * @param {Vector} position
     * @returns {number}
     */
    add_ship_internal(blueprint, position) {
        _assertClass(blueprint, Blueprint);
        _assertClass(position, Vector);
        var ptr0 = position.__destroy_into_raw();
        const ret = wasm.gravithrust_add_ship_internal(this.__wbg_ptr, blueprint.__wbg_ptr, ptr0);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    particles_size() {
        const ret = wasm.gravithrust_particles_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    particle_size() {
        const ret = wasm.gravithrust_particle_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    static particle_size_internal() {
        const ret = wasm.gravithrust_particle_size_internal();
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    particles_count() {
        const ret = wasm.gravithrust_particles_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    particles() {
        const ret = wasm.gravithrust_particles(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    ships_size() {
        const ret = wasm.gravithrust_ships_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    ship_size() {
        const ret = wasm.gravithrust_ship_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    static ship_size_internal() {
        const ret = wasm.gravithrust_ship_size_internal();
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    ships_count() {
        const ret = wasm.gravithrust_ships_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    ships() {
        const ret = wasm.gravithrust_ships(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    link_js_size() {
        const ret = wasm.gravithrust_link_js_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    link_js_size_() {
        const ret = wasm.gravithrust_link_js_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    links_js() {
        const ret = wasm.gravithrust_links_js(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    links_count() {
        const ret = wasm.gravithrust_links_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    links_js_size() {
        const ret = wasm.gravithrust_links_js_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    average_durations_pointer() {
        const ret = wasm.gravithrust_average_durations_pointer(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    average_durations_size_unit() {
        const ret = wasm.gravithrust_average_durations_size_unit(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    average_durations_size_full() {
        const ret = wasm.gravithrust_average_durations_size_full(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    average_durations_count() {
        const ret = wasm.gravithrust_average_durations_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {string}
     */
    ticks() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.gravithrust_ticks(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} sid
     * @param {string} job_json
     */
    set_job(sid, job_json) {
        const ptr0 = passStringToWasm0(job_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.gravithrust_set_job(this.__wbg_ptr, sid, ptr0, len0);
    }
}
if (Symbol.dispose) Gravithrust.prototype[Symbol.dispose] = Gravithrust.prototype.free;

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
}
if (Symbol.dispose) Link.prototype[Symbol.dispose] = Link.prototype.free;

const LinkJSFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_linkjs_free(ptr >>> 0, 1));

export class LinkJS {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LinkJSFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_linkjs_free(ptr, 0);
    }
    /**
     * @returns {Kind}
     */
    get ak() {
        const ret = wasm.__wbg_get_linkjs_ak(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Kind} arg0
     */
    set ak(arg0) {
        wasm.__wbg_set_linkjs_ak(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Kind}
     */
    get bk() {
        const ret = wasm.__wbg_get_linkjs_bk(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Kind} arg0
     */
    set bk(arg0) {
        wasm.__wbg_set_linkjs_bk(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Vector}
     */
    get p() {
        const ret = wasm.__wbg_get_linkjs_p(this.__wbg_ptr);
        return Vector.__wrap(ret);
    }
    /**
     * @param {Vector} arg0
     */
    set p(arg0) {
        _assertClass(arg0, Vector);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_linkjs_p(this.__wbg_ptr, ptr0);
    }
}
if (Symbol.dispose) LinkJS.prototype[Symbol.dispose] = LinkJS.prototype.free;

const ParticleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_particle_free(ptr >>> 0, 1));

export class Particle {

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
     * @returns {Quantities}
     */
    get qs() {
        const ret = wasm.__wbg_get_particle_qs(this.__wbg_ptr);
        return Quantities.__wrap(ret);
    }
    /**
     * @param {Quantities} arg0
     */
    set qs(arg0) {
        _assertClass(arg0, Quantities);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_qs(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Vector}
     */
    get p() {
        const ret = wasm.__wbg_get_particle_p(this.__wbg_ptr);
        return Vector.__wrap(ret);
    }
    /**
     * @param {Vector} arg0
     */
    set p(arg0) {
        _assertClass(arg0, Vector);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_p(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Vector}
     */
    get v() {
        const ret = wasm.__wbg_get_particle_v(this.__wbg_ptr);
        return Vector.__wrap(ret);
    }
    /**
     * @param {Vector} arg0
     */
    set v(arg0) {
        _assertClass(arg0, Vector);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_v(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Vector}
     */
    get pp() {
        const ret = wasm.__wbg_get_particle_pp(this.__wbg_ptr);
        return Vector.__wrap(ret);
    }
    /**
     * @param {Vector} arg0
     */
    set pp(arg0) {
        _assertClass(arg0, Vector);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_pp(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Vector}
     */
    get direction() {
        const ret = wasm.__wbg_get_particle_direction(this.__wbg_ptr);
        return Vector.__wrap(ret);
    }
    /**
     * @param {Vector} arg0
     */
    set direction(arg0) {
        _assertClass(arg0, Vector);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_direction(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {number}
     */
    get m() {
        const ret = wasm.__wbg_get_particle_m(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set m(arg0) {
        wasm.__wbg_set_particle_m(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Kind}
     */
    get k() {
        const ret = wasm.__wbg_get_particle_k(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Kind} arg0
     */
    set k(arg0) {
        wasm.__wbg_set_particle_k(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get a() {
        const ret = wasm.__wbg_get_particle_a(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set a(arg0) {
        wasm.__wbg_set_particle_a(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get live() {
        const ret = wasm.__wbg_get_particle_live(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set live(arg0) {
        wasm.__wbg_set_particle_live(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get grid_id() {
        const ret = wasm.__wbg_get_particle_grid_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set grid_id(arg0) {
        wasm.__wbg_set_particle_grid_id(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get idx() {
        const ret = wasm.__wbg_get_particle_idx(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set idx(arg0) {
        wasm.__wbg_set_particle_idx(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Particle.prototype[Symbol.dispose] = Particle.prototype.free;

const ParticleBlueprintFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_particleblueprint_free(ptr >>> 0, 1));

export class ParticleBlueprint {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ParticleBlueprintFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_particleblueprint_free(ptr, 0);
    }
}
if (Symbol.dispose) ParticleBlueprint.prototype[Symbol.dispose] = ParticleBlueprint.prototype.free;

const QuantitiesFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_quantities_free(ptr >>> 0, 1));

export class Quantities {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Quantities.prototype);
        obj.__wbg_ptr = ptr;
        QuantitiesFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        QuantitiesFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_quantities_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get q1() {
        const ret = wasm.__wbg_get_quantities_q1(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set q1(arg0) {
        wasm.__wbg_set_quantities_q1(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get q2() {
        const ret = wasm.__wbg_get_quantities_q2(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set q2(arg0) {
        wasm.__wbg_set_quantities_q2(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get q3() {
        const ret = wasm.__wbg_get_quantities_q3(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set q3(arg0) {
        wasm.__wbg_set_quantities_q3(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get q4() {
        const ret = wasm.__wbg_get_quantities_q4(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set q4(arg0) {
        wasm.__wbg_set_quantities_q4(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get q5() {
        const ret = wasm.__wbg_get_quantities_q5(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set q5(arg0) {
        wasm.__wbg_set_quantities_q5(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get q6() {
        const ret = wasm.__wbg_get_quantities_q6(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set q6(arg0) {
        wasm.__wbg_set_quantities_q6(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Quantities.prototype[Symbol.dispose] = Quantities.prototype.free;

const VectorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vector_free(ptr >>> 0, 1));

export class Vector {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Vector.prototype);
        obj.__wbg_ptr = ptr;
        VectorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VectorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vector_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_vector_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_vector_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_vector_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_vector_y(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Vector.prototype[Symbol.dispose] = Vector.prototype.free;

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
    imports.wbg.__wbg_error_acd01c75aee2da9d = function(arg0, arg1) {
        console.error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_getRandomValues_3c9c0d586e575a16 = function() { return handleError(function (arg0, arg1) {
        globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
    }, arguments) };
    imports.wbg.__wbg_log_36839896a3b0f5e8 = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_now_1e80617bcee43265 = function() {
        const ret = Date.now();
        return ret;
    };
    imports.wbg.__wbg_wbindgenthrow_451ec1a8469d7eb6 = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_2;
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
    cachedUint32ArrayMemory0 = null;
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
        module_or_path = new URL('gravithrust_bg.wasm', import.meta.url);
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
