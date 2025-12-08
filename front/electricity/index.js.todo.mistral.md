## Reasoning

The TODO in the code is related to the `setup_slider` function where it mentions updating the HTML to set `min`, `max`, and `value` attributes on the slider input element. Currently, these attributes are not being set programmatically, which could lead to inconsistencies between the JavaScript logic and the actual HTML attributes.

The function takes `min`, `max`, and `value` parameters but doesn't apply them to the input element. This means:
1. The slider might not respect the intended range
2. The initial value might not match what's expected
3. The HTML attributes won't reflect the JavaScript configuration

We should update the function to set these attributes on the input element to ensure consistency between the JavaScript configuration and the HTML representation.

## Updates

### ~/github.com/loicbourgois/gravitle/front/electricity/index.js

#### Slider configuration update
```javascript
const setup_slider = (id, min, max, value) => {
	const input = document.getElementById(`${id}-slider-input`);
	const valueDisplay = document.getElementById(`${id}-slider-value`);
	// Set HTML attributes to match the function parameters
	input.min = min;
	input.max = max;
	input.value = value;
	const update = () => {
		valueDisplay.textContent = padn(input.value, 5);
	};
	input.addEventListener("input", update);
	update();
};
```

The change:
1. Renamed the `value` variable to `valueDisplay` to avoid shadowing the parameter
2. Added lines to set the `min`, `max`, and `value` attributes on the input element
3. This ensures the slider's HTML attributes match the JavaScript configuration passed to the function