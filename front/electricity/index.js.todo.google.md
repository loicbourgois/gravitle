## Reasoning
The `TODO` comment explicitly states "update html to set min max and value". This indicates that the `setup_slider` function, which receives `min`, `max`, and `value` as parameters, should programmatically set these attributes on the corresponding HTML input element.

Additionally, there's a variable shadowing issue where the `value` parameter of `setup_slider` is shadowed by a `const value = document.getElementById(...)` declaration. This needs to be resolved to ensure the `value` parameter (representing the initial slider value) can be correctly used. Renaming the DOM element variable to `valueOutput` clarifies its purpose and resolves the conflict.

The changes involve:
1.  Renaming the `value` parameter to `initialValue` and the DOM element variable to `valueOutput` to avoid shadowing and improve clarity.
2.  Setting the `min`, `max`, and `initialValue` properties of the `input` element directly from the function parameters.
3.  Ensuring the `update` function is called after setting the `input.value` to correctly display the initial value.

## Updates

### front/electricity/index.js

#### Update `setup_slider` function
```javascript
const setup_slider = (id, min, max, initialValue) => {
	const input = document.getElementById(`${id}-slider-input`);
	const valueOutput = document.getElementById(`${id}-slider-value`);

	// Set min, max, and initial value for the input element
	input.min = min;
	input.max = max;
	input.value = initialValue;

	const update = () => {
		valueOutput.textContent = padn(input.value, 5);
	};

	input.addEventListener("input", update);
	update(); // Call update to set the initial display value
};
```