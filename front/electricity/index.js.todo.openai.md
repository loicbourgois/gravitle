## Reasoning

The `setup_slider` function includes a TODO comment:  
```js
// TODO: update html to set min max and value
```

Currently, the function retrieves the slider’s input element (`input`) and the value display element (`value`), sets up an event listener to display changes, and initializes it with a call to `update()`. However, it never actually applies the `min`, `max`, and `value` arguments to the input element itself. This means the slider would rely on default or previously hardcoded HTML attributes instead of being dynamically configured.

To resolve this TODO, we must explicitly assign `input.min`, `input.max`, and `input.value` from the function parameters. This ensures that whenever `setup_slider` is called, the slider element is properly configured, improving configurability and consistency.

## Updates

### ~/github.com/loicbourgois/gravitle/front/electricity/index.js

#### <change #1>
```javascript
const setup_slider = (id, min, max, value) => {
	const input = document.getElementById(`${id}-slider-input`)
	const valueEl = document.getElementById(`${id}-slider-value`)

	// Apply slider configuration (resolves TODO)
	input.min = min;
	input.max = max;
	input.value = value;

	const update = () => {
		valueEl.textContent = padn(input.value, 5);
	}

	input.addEventListener("input", update)
	update()
}
```