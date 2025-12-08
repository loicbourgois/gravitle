## Reasoning
The `TODO` explicitly requests the addition of a slider ranging from 0 to 10000. To fulfill this, an `input` element of type `range` is the most suitable choice. I've added a label for clarity and a `span` to display the current value of the slider, which is a common pattern for range inputs to provide immediate feedback to the user. The slider is initialized to a midpoint value (5000). This new UI component is placed where the `TODO` comment was, just before the `uniforms` paragraph.

## Updates

### `~/github.com/loicbourgois/gravitle/front/electricity/index.html`

#### Add slider from 0 to 10000
```html
        <p class="pre">  render............ <span id="frame_budget_render"></span>%</p>
        <p class="pre">  stats............. <span id="frame_budget_stats"></span>%</p>
        <p id="info" class="pre"></p>
        <label for="slider-input">Slider Value: <span id="slider-value">5000</span></label>
        <input type="range" id="slider-input" min="0" max="10000" value="5000" style="width: 100%;">
        <p id="uniforms" class="pre"></p>
      </div>
    </div>
```