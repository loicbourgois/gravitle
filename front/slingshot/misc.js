
const has_webgpu_support = async () => {
	const adapter = await navigator.gpu?.requestAdapter();
	const device = await adapter?.requestDevice();
	if (device) {
		return true;
	} else {
		return false;
	}
};

const get_avg_fps = (tick_starts) => {
	return Math.round(
		1000 / ((tick_starts.at(-1) - tick_starts[0]) / (tick_starts.length - 1)),
	);
};

const as_frame_budget = (value) => {
	return as_percent_str( value / (1000/120) )
}

const as_percent_str = (value) => {
	const s = `${(value * 100 )?.toFixed(1)}`
    return s.padStart(4);
}

const pad5 = (value) => {
	return `${value}`.padStart(5);
}

export {
    has_webgpu_support,
    get_avg_fps,
    as_frame_budget,
    pad5,
    as_percent_str,
}
