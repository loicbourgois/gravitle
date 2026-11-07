class NoiseGenerator extends AudioWorkletProcessor {
	process(inputs, outputs) {
		const output = outputs[0];
		const channel = output[0];
		for (let i = 0; i < channel.length; i++) {
			channel[i] = Math.random() * 2 - 1;
		}
		return true;
	}
}

registerProcessor("noise-generator", NoiseGenerator);
