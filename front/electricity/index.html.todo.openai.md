## Reasoning

The HTML code currently displays performance and diagnostic data. A `TODO` comment specifies that a slider should be added with a range from 0 to 10,000. Given the surrounding structure—especially that other controls and debug stats are laid out within the `#overlay-right` div—it’s reasonable to assume that the slider belongs in this section as part of the performance/debug interface.

The solution should:
- Add a labeled HTML `<input type="range">` element with minimum 0 and maximum 10,000.
- Optionally display the current slider value beside it, consistent with how other metrics are shown.
- Be placed right below the “TODO” comment’s location (i.e., between the debug info and uniforms section).

No JavaScript change is required unless the slider needs custom behavior—here we only fulfill the HTML TODO by adding the visual and structural element.

## Updates

### ~/github.com/loicbourgois/gravitle/front/electricity/index.html

#### Add slider from 0 to 10,000
```html
        <p id="info" class="pre"></p>
        <label for="debug-slider" class="pre">value (0–10000): </label>
        <input id="debug-slider" type="range" min="0" max="10000" value="5000" step="1" />
        <span id="debug-slider-value">5000</span>
        <p id="uniforms" class="pre"></p>

        <script>
          const slider = document.getElementById('debug-slider');
          const valueEl = document.getElementById('debug-slider-value');
          slider.addEventListener('input', () => {
            valueEl.textContent = slider.value;
          });
        </script>
```