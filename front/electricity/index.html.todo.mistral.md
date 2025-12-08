## Reasoning
The TODO in the HTML file requests adding a slider input ranging from 0 to 10000. This is likely meant to control some simulation parameter (e.g., simulation speed, particle count, or other tunable value). The slider should be added in the overlay section where the TODO is located, as this is where other controls and metrics are displayed.

The slider will need:
1. An `<input type="range">` element with min=0 and max=10000
2. A label to indicate its purpose (though the exact purpose isn't specified, we'll use a generic label)
3. Basic styling to match the existing UI (though styling can be adjusted later via CSS)

## Updates

### ~/github.com/loicbourgois/gravitle/front/electricity/index.html

#### Added slider control
```html
<p class="pre">uniforms............ <span id="uniforms"></span></p>
<!-- Added slider as requested in TODO -->
<p class="pre">
  <label for="simulation-slider">Simulation parameter:</label>
  <input type="range" id="simulation-slider" min="0" max="10000" value="5000">
  <span id="slider-value">5000</span>
</p>
```