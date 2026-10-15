<script lang="ts">
  import { editorState, DESCRIPTORS } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  // Scrubbing drag state
  let activeScrubProp = $state<string | null>(null);
  let scrubStartX = $state<number>(0);
  let scrubStartVal = $state<number>(0);

  const PRESET_COLORS = ['#36F2CB', '#00FF00', '#FF3864', '#FFFFFF', '#FFE600', '#FF4FD8', '#00B2FF'];

  const SVG_PRESETS = [
    { label: 'Arrow', path: 'M 0,-8 L 6,6 L 0,3 L -6,6 Z' },
    { label: 'Diamond', path: 'M 0,-8 L 8,0 L 0,8 L -8,0 Z' },
    { label: 'Triangle', path: 'M 0,-8 L 7,6 L -7,6 Z' },
    { label: 'Chevron', path: 'M -6,5 L 0,0 L 6,5' },
    { label: 'Crosshair', path: 'M -8,0 L -2,0 M 2,0 L 8,0 M 0,-8 L 0,-2 M 0,2 L 0,8' },
  ];

  function getPropertyValue(name: string): number {
    const layer = editorState.selectedLayer;
    if (!layer) return 0;

    if (name === 'line.length' || name === 'shape.length') return layer.shape.length ?? 6;
    if (name === 'line.thickness' || name === 'shape.thickness') return layer.shape.thickness ?? 2;
    if (name === 'line.gap' || name === 'shape.gap') return layer.shape.gap ?? 4;
    if (name === 'line.arm_count' || name === 'shape.arm_count') return layer.shape.armCount ?? 4;
    if (name === 'dot.diameter' || name === 'shape.diameter') return layer.shape.diameter ?? 2;
    if (name === 'shape.radius') return layer.shape.radius ?? 8;
    if (name === 'shape.inner_radius') return layer.shape.innerRadius ?? 4;
    if (name === 'shape.points') return layer.shape.points ?? 6;
    if (name === 'shape.angle') return layer.shape.angle ?? 90;
    if (name === 'shape.width') return layer.shape.width ?? 12;
    if (name === 'shape.height') return layer.shape.height ?? 8;
    if (name === 'shape.corner_radius') return layer.shape.cornerRadius ?? 0;
    if (name === 'shape.base') return layer.shape.base ?? 10;
    if (name === 'shape.start_angle') return layer.shape.startAngle ?? 0;
    if (name === 'shape.end_angle') return layer.shape.endAngle ?? 180;
    if (name === 'shape.center_hole') return layer.shape.centerHole ?? 0;
    if (name === 'effect.shadow_blur') return layer.shadowBlur ?? 0;
    if (name === 'effect.shadow_offset_x') return layer.shadowOffsetX ?? 0;
    if (name === 'effect.shadow_offset_y') return layer.shadowOffsetY ?? 0;
    if (name === 'effect.blur_radius') return layer.blurRadius ?? 0;
    if (name === 'effect.glow_radius') return layer.glowRadius ?? 0;
    if (name === 'layer.offset_x') return layer.offsetX ?? 0;
    if (name === 'layer.offset_y') return layer.offsetY ?? 0;
    if (name === 'layer.rotation') return layer.rotation ?? 0;
    if (name === 'global.opacity' || name === 'layer.opacity') return layer.opacity ?? 1.0;
    return 0;
  }

  function handleSliderChange(name: string, e: Event) {
    const target = e.target as HTMLInputElement;
    const val = parseFloat(target.value);
    editorState.updateProperty(name, val);
  }

  function handleNumberChange(name: string, e: Event) {
    const target = e.target as HTMLInputElement;
    const val = parseFloat(target.value);
    if (!isNaN(val)) {
      editorState.updateProperty(name, val);
    }
  }

  function startScrub(name: string, e: MouseEvent) {
    activeScrubProp = name;
    scrubStartX = e.clientX;
    scrubStartVal = getPropertyValue(name);
    window.addEventListener('mousemove', onScrubMove);
    window.addEventListener('mouseup', endScrub);
  }

  function onScrubMove(e: MouseEvent) {
    if (!activeScrubProp) return;
    const deltaX = e.clientX - scrubStartX;
    const step = e.altKey ? 0.1 : e.shiftKey ? 2.0 : 0.5;
    const stepsMoved = Math.round(deltaX / 5);
    let newVal = scrubStartVal + stepsMoved * step;
    newVal = Math.round(newVal * 100) / 100;
    editorState.updateProperty(activeScrubProp, newVal);
  }

  function endScrub() {
    activeScrubProp = null;
    window.removeEventListener('mousemove', onScrubMove);
    window.removeEventListener('mouseup', endScrub);
  }
</script>

<div class="panel">
  <div class="panel-header">
    <span>Properties</span>
    <span class="mono" style="color: var(--text-muted); font-size: 11px;">
      {editorState.selectedLayer?.name || 'No selection'}
    </span>
  </div>

  {#if editorState.selectedLayer}
    {@const layer = editorState.selectedLayer}
    {@const shape = layer.shape}

    <div class="properties-list">
      <!-- Section: Color & Appearance -->
      <div class="prop-section">
        <span class="section-title">Color & Appearance</span>
        <div class="color-row">
          <div class="swatches">
            {#each PRESET_COLORS as c}
              <button
                class="swatch-btn {layer.color === c ? 'active' : ''}"
                style="background: {c};"
                onclick={() => editorState.setLayerColor(c)}
                aria-label="Color {c}"
              ></button>
            {/each}
          </div>
          <input
            type="color"
            class="native-color-picker"
            value={layer.color}
            oninput={(e) => editorState.setLayerColor((e.target as HTMLInputElement).value)}
            aria-label="Custom color picker"
          />
        </div>
      </div>

      <!-- Section: Transform & Position (2D Movement) -->
      <div class="prop-section">
        <span class="section-title">Position & Transform</span>

        <!-- Position X -->
        <div class="property-row">
          <div class="row">
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <span
              class="property-label {activeScrubProp === 'layer.offset_x' ? 'scrubbing' : ''}"
              onmousedown={(e) => startScrub('layer.offset_x', e)}
            >
              Position X ↔
            </span>
            <div class="input-with-unit">
              <input
                type="number"
                class="input-text mono prop-number-input"
                step="0.5"
                value={layer.offsetX}
                oninput={(e) => handleNumberChange('layer.offset_x', e)}
                aria-label="Position X"
              />
              <span class="unit-label">px</span>
            </div>
          </div>
          <div class="slider-wrapper">
            <input
              type="range"
              class="prop-slider"
              min="-50"
              max="50"
              step="0.5"
              value={layer.offsetX}
              oninput={(e) => handleSliderChange('layer.offset_x', e)}
              aria-label="Position X slider"
            />
            <div class="slider-fill-visual" style="width: {((layer.offsetX + 50) / 100) * 100}%;"></div>
          </div>
        </div>

        <!-- Position Y -->
        <div class="property-row">
          <div class="row">
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <span
              class="property-label {activeScrubProp === 'layer.offset_y' ? 'scrubbing' : ''}"
              onmousedown={(e) => startScrub('layer.offset_y', e)}
            >
              Position Y ↔
            </span>
            <div class="input-with-unit">
              <input
                type="number"
                class="input-text mono prop-number-input"
                step="0.5"
                value={layer.offsetY}
                oninput={(e) => handleNumberChange('layer.offset_y', e)}
                aria-label="Position Y"
              />
              <span class="unit-label">px</span>
            </div>
          </div>
          <div class="slider-wrapper">
            <input
              type="range"
              class="prop-slider"
              min="-50"
              max="50"
              step="0.5"
              value={layer.offsetY}
              oninput={(e) => handleSliderChange('layer.offset_y', e)}
              aria-label="Position Y slider"
            />
            <div class="slider-fill-visual" style="width: {((layer.offsetY + 50) / 100) * 100}%;"></div>
          </div>
        </div>

        <!-- Rotation -->
        <div class="property-row">
          <div class="row">
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <span
              class="property-label {activeScrubProp === 'layer.rotation' ? 'scrubbing' : ''}"
              onmousedown={(e) => startScrub('layer.rotation', e)}
            >
              Rotation ↔
            </span>
            <div class="input-with-unit">
              <input
                type="number"
                class="input-text mono prop-number-input"
                min="0"
                max="360"
                step="1"
                value={layer.rotation}
                oninput={(e) => handleNumberChange('layer.rotation', e)}
                aria-label="Rotation"
              />
              <span class="unit-label">°</span>
            </div>
          </div>
          <div class="slider-wrapper">
            <input
              type="range"
              class="prop-slider"
              min="0"
              max="360"
              step="1"
              value={layer.rotation}
              oninput={(e) => handleSliderChange('layer.rotation', e)}
              aria-label="Rotation slider"
            />
            <div class="slider-fill-visual" style="width: {(layer.rotation / 360) * 100}%;"></div>
          </div>
        </div>

        <!-- Opacity -->
        <div class="property-row">
          <div class="row">
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <span
              class="property-label {activeScrubProp === 'layer.opacity' ? 'scrubbing' : ''}"
              onmousedown={(e) => startScrub('layer.opacity', e)}
            >
              Opacity ↔
            </span>
            <div class="input-with-unit">
              <input
                type="number"
                class="input-text mono prop-number-input"
                min="0"
                max="1"
                step="0.05"
                value={layer.opacity}
                oninput={(e) => handleNumberChange('layer.opacity', e)}
                aria-label="Opacity"
              />
            </div>
          </div>
          <div class="slider-wrapper">
            <input
              type="range"
              class="prop-slider"
              min="0"
              max="1"
              step="0.05"
              value={layer.opacity}
              oninput={(e) => handleSliderChange('layer.opacity', e)}
              aria-label="Opacity slider"
            />
            <div class="slider-fill-visual" style="width: {(layer.opacity || 1) * 100}%;"></div>
          </div>
        </div>
      </div>

      <!-- Section: Shape Geometry -->
      <div class="prop-section">
        <span class="section-title">Shape Geometry ({shape.type})</span>

        <!-- LineArms / TShape Parameters -->
        {#if shape.type === 'LineArms' || shape.type === 'TShape'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Length</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="40"
                  step="1"
                  value={shape.length ?? 6}
                  oninput={(e) => handleNumberChange('shape.length', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="1"
                max="40"
                step="1"
                value={shape.length ?? 6}
                oninput={(e) => handleSliderChange('shape.length', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.length ?? 6) - 1) / 39) * 100}%;"></div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Thickness</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="12"
                  step="1"
                  value={shape.thickness ?? 2}
                  oninput={(e) => handleNumberChange('shape.thickness', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="1"
                max="12"
                step="1"
                value={shape.thickness ?? 2}
                oninput={(e) => handleSliderChange('shape.thickness', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.thickness ?? 2) - 1) / 11) * 100}%;"></div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Gap</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="0"
                  max="30"
                  step="0.5"
                  value={shape.gap ?? 4}
                  oninput={(e) => handleNumberChange('shape.gap', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="0"
                max="30"
                step="0.5"
                value={shape.gap ?? 4}
                oninput={(e) => handleSliderChange('shape.gap', e)}
              />
              <div class="slider-fill-visual" style="width: {((shape.gap ?? 4) / 30) * 100}%;"></div>
            </div>
          </div>
        {/if}

        <!-- Dot Parameters -->
        {#if shape.type === 'Dot'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Diameter</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="16"
                  step="0.5"
                  value={shape.diameter ?? 2}
                  oninput={(e) => handleNumberChange('shape.diameter', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="1"
                max="16"
                step="0.5"
                value={shape.diameter ?? 2}
                oninput={(e) => handleSliderChange('shape.diameter', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.diameter ?? 2) - 1) / 15) * 100}%;"></div>
            </div>
          </div>

          <div class="row" style="margin-top: 4px;">
            <span class="property-label">Dot Style</span>
            <div class="chips-group">
              <button
                class="pill-chip {!shape.isSquare ? 'active' : ''}"
                onclick={() => editorState.setDotSquare(false)}
              >
                Round
              </button>
              <button
                class="pill-chip {shape.isSquare ? 'active' : ''}"
                onclick={() => editorState.setDotSquare(true)}
              >
                Square
              </button>
            </div>
          </div>
        {/if}

        <!-- Circle / Ring Parameters -->
        {#if shape.type === 'Circle' || shape.type === 'Ring'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Radius</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="2"
                  max="40"
                  step="0.5"
                  value={shape.radius ?? 8}
                  oninput={(e) => handleNumberChange('shape.radius', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="2"
                max="40"
                step="0.5"
                value={shape.radius ?? 8}
                oninput={(e) => handleSliderChange('shape.radius', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.radius ?? 8) - 2) / 38) * 100}%;"></div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Thickness</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="10"
                  step="1"
                  value={shape.thickness ?? 2}
                  oninput={(e) => handleNumberChange('shape.thickness', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="1"
                max="10"
                step="1"
                value={shape.thickness ?? 2}
                oninput={(e) => handleSliderChange('shape.thickness', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.thickness ?? 2) - 1) / 9) * 100}%;"></div>
            </div>
          </div>
        {/if}

        <!-- Chevron Parameters -->
        {#if shape.type === 'Chevron'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Length</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="2"
                  max="30"
                  step="1"
                  value={shape.length ?? 7}
                  oninput={(e) => handleNumberChange('shape.length', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Angle</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="30"
                  max="150"
                  step="5"
                  value={shape.angle ?? 90}
                  oninput={(e) => handleNumberChange('shape.angle', e)}
                />
                <span class="unit-label">°</span>
              </div>
            </div>
          </div>
        {/if}

        <!-- Polygon Parameters -->
        {#if shape.type === 'Polygon'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Radius</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="3"
                  max="40"
                  step="1"
                  value={shape.radius ?? 8}
                  oninput={(e) => handleNumberChange('shape.radius', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Sides / Points</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="3"
                  max="12"
                  step="1"
                  value={shape.points ?? 6}
                  oninput={(e) => handleNumberChange('shape.points', e)}
                />
              </div>
            </div>
          </div>
        {/if}

        <!-- Star Parameters -->
        {#if shape.type === 'Star'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Outer Radius</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="4"
                  max="40"
                  step="1"
                  value={shape.radius ?? 9}
                  oninput={(e) => handleNumberChange('shape.radius', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Inner Radius</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="30"
                  step="1"
                  value={shape.innerRadius ?? 4}
                  oninput={(e) => handleNumberChange('shape.inner_radius', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Star Points</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="3"
                  max="10"
                  step="1"
                  value={shape.points ?? 4}
                  oninput={(e) => handleNumberChange('shape.points', e)}
                />
              </div>
            </div>
          </div>
        {/if}

        <!-- SvgPath Parameters -->
        {#if shape.type === 'SvgPath'}
          <div class="property-row">
            <span class="property-label">Vector Presets:</span>
            <div class="chips-group" style="flex-wrap: wrap;">
              {#each SVG_PRESETS as p}
                <button
                  class="pill-chip {shape.pathData === p.path ? 'active' : ''}"
                  onclick={() => editorState.setShapePathData(p.path)}
                >
                  {p.label}
                </button>
              {/each}
            </div>

            <span class="property-label" style="margin-top: 6px;">Path Data (SVG):</span>
            <textarea
              class="input-text mono path-textarea"
              rows="2"
              value={shape.pathData ?? ''}
              oninput={(e) => editorState.setShapePathData((e.target as HTMLTextAreaElement).value)}
            ></textarea>
          </div>
        {/if}

        <!-- Rect Parameters -->
        {#if shape.type === 'Rect'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Width</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="60"
                  step="1"
                  value={shape.width ?? 12}
                  oninput={(e) => handleNumberChange('shape.width', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="1"
                max="60"
                step="1"
                value={shape.width ?? 12}
                oninput={(e) => handleSliderChange('shape.width', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.width ?? 12) - 1) / 59) * 100}%;"></div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Height</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="60"
                  step="1"
                  value={shape.height ?? 8}
                  oninput={(e) => handleNumberChange('shape.height', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="1"
                max="60"
                step="1"
                value={shape.height ?? 8}
                oninput={(e) => handleSliderChange('shape.height', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.height ?? 8) - 1) / 59) * 100}%;"></div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Corner Radius</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="0"
                  max="20"
                  step="0.5"
                  value={shape.cornerRadius ?? 0}
                  oninput={(e) => handleNumberChange('shape.corner_radius', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="0"
                max="20"
                step="0.5"
                value={shape.cornerRadius ?? 0}
                oninput={(e) => handleSliderChange('shape.corner_radius', e)}
              />
              <div class="slider-fill-visual" style="width: {((shape.cornerRadius ?? 0) / 20) * 100}%;"></div>
            </div>
          </div>
        {/if}

        <!-- Arc Parameters -->
        {#if shape.type === 'Arc'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Radius</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="2"
                  max="40"
                  step="0.5"
                  value={shape.radius ?? 10}
                  oninput={(e) => handleNumberChange('shape.radius', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="2"
                max="40"
                step="0.5"
                value={shape.radius ?? 10}
                oninput={(e) => handleSliderChange('shape.radius', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.radius ?? 10) - 2) / 38) * 100}%;"></div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Thickness</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="10"
                  step="1"
                  value={shape.thickness ?? 2}
                  oninput={(e) => handleNumberChange('shape.thickness', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="1"
                max="10"
                step="1"
                value={shape.thickness ?? 2}
                oninput={(e) => handleSliderChange('shape.thickness', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.thickness ?? 2) - 1) / 9) * 100}%;"></div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Start Angle</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="0"
                  max="360"
                  step="5"
                  value={shape.startAngle ?? 0}
                  oninput={(e) => handleNumberChange('shape.start_angle', e)}
                />
                <span class="unit-label">°</span>
              </div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">End Angle</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="0"
                  max="360"
                  step="5"
                  value={shape.endAngle ?? 180}
                  oninput={(e) => handleNumberChange('shape.end_angle', e)}
                />
                <span class="unit-label">°</span>
              </div>
            </div>
          </div>
        {/if}

        <!-- Triangle Parameters -->
        {#if shape.type === 'Triangle'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Base</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="2"
                  max="40"
                  step="1"
                  value={shape.base ?? 10}
                  oninput={(e) => handleNumberChange('shape.base', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="2"
                max="40"
                step="1"
                value={shape.base ?? 10}
                oninput={(e) => handleSliderChange('shape.base', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.base ?? 10) - 2) / 38) * 100}%;"></div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Height</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="2"
                  max="40"
                  step="1"
                  value={shape.height ?? 12}
                  oninput={(e) => handleNumberChange('shape.height', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min="2"
                max="40"
                step="1"
                value={shape.height ?? 12}
                oninput={(e) => handleSliderChange('shape.height', e)}
              />
              <div class="slider-fill-visual" style="width: {(((shape.height ?? 12) - 2) / 38) * 100}%;"></div>
            </div>
          </div>
        {/if}

        <!-- Plus Parameters -->
        {#if shape.type === 'Plus'}
          <div class="property-row">
            <div class="row">
              <span class="property-label">Length</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="40"
                  step="1"
                  value={shape.length ?? 6}
                  oninput={(e) => handleNumberChange('shape.length', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Thickness</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="1"
                  max="12"
                  step="1"
                  value={shape.thickness ?? 2}
                  oninput={(e) => handleNumberChange('shape.thickness', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Gap</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="0"
                  max="30"
                  step="0.5"
                  value={shape.gap ?? 2}
                  oninput={(e) => handleNumberChange('shape.gap', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
          </div>

          <div class="property-row">
            <div class="row">
              <span class="property-label">Center Hole</span>
              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min="0"
                  max="20"
                  step="0.5"
                  value={shape.centerHole ?? 0}
                  oninput={(e) => handleNumberChange('shape.center_hole', e)}
                />
                <span class="unit-label">px</span>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Section: Visual Effects & Filters (§4.6) -->
      <div class="prop-section">
        <span class="section-title">Visual Effects & Filters</span>

        <!-- Glow Effect -->
        <div class="property-row">
          <div class="row">
            <span class="property-label">Glow Radius</span>
            <div class="input-with-unit">
              <input
                type="number"
                class="input-text mono prop-number-input"
                min="0"
                max="20"
                step="0.5"
                value={layer.glowRadius ?? 0}
                oninput={(e) => handleNumberChange('effect.glow_radius', e)}
              />
              <span class="unit-label">px</span>
            </div>
          </div>
          <div class="slider-wrapper">
            <input
              type="range"
              class="prop-slider"
              min="0"
              max="20"
              step="0.5"
              value={layer.glowRadius ?? 0}
              oninput={(e) => handleSliderChange('effect.glow_radius', e)}
            />
            <div class="slider-fill-visual" style="width: {((layer.glowRadius ?? 0) / 20) * 100}%;"></div>
          </div>
        </div>

        <!-- Drop Shadow -->
        <div class="property-row">
          <div class="row">
            <span class="property-label">Drop Shadow Blur</span>
            <div class="input-with-unit">
              <input
                type="number"
                class="input-text mono prop-number-input"
                min="0"
                max="20"
                step="0.5"
                value={layer.shadowBlur ?? 0}
                oninput={(e) => handleNumberChange('effect.shadow_blur', e)}
              />
              <span class="unit-label">px</span>
            </div>
          </div>
          <div class="slider-wrapper">
            <input
              type="range"
              class="prop-slider"
              min="0"
              max="20"
              step="0.5"
              value={layer.shadowBlur ?? 0}
              oninput={(e) => handleSliderChange('effect.shadow_blur', e)}
            />
            <div class="slider-fill-visual" style="width: {((layer.shadowBlur ?? 0) / 20) * 100}%;"></div>
          </div>
        </div>

        <div class="property-row">
          <div class="row">
            <span class="property-label">Shadow Offset X / Y</span>
            <div style="display: flex; gap: 6px;">
              <input
                type="number"
                class="input-text mono prop-number-input"
                style="width: 50px;"
                min="-20"
                max="20"
                step="0.5"
                value={layer.shadowOffsetX ?? 0}
                oninput={(e) => handleNumberChange('effect.shadow_offset_x', e)}
                placeholder="X"
              />
              <input
                type="number"
                class="input-text mono prop-number-input"
                style="width: 50px;"
                min="-20"
                max="20"
                step="0.5"
                value={layer.shadowOffsetY ?? 0}
                oninput={(e) => handleNumberChange('effect.shadow_offset_y', e)}
                placeholder="Y"
              />
            </div>
          </div>
        </div>

        <!-- Softening Blur -->
        <div class="property-row">
          <div class="row">
            <span class="property-label">Softening Blur</span>
            <div class="input-with-unit">
              <input
                type="number"
                class="input-text mono prop-number-input"
                min="0"
                max="8"
                step="0.25"
                value={layer.blurRadius ?? 0}
                oninput={(e) => handleNumberChange('effect.blur_radius', e)}
              />
              <span class="unit-label">px</span>
            </div>
          </div>
          <div class="slider-wrapper">
            <input
              type="range"
              class="prop-slider"
              min="0"
              max="8"
              step="0.25"
              value={layer.blurRadius ?? 0}
              oninput={(e) => handleSliderChange('effect.blur_radius', e)}
            />
            <div class="slider-fill-visual" style="width: {((layer.blurRadius ?? 0) / 8) * 100}%;"></div>
          </div>
        </div>
      </div>

      <!-- Dynamic Binding Live Preview Scrubber (§12.5) -->
      <div class="dynamic-scrubber-section">
        <div class="dynamic-header">
          <span class="dyn-badge">DYNAMIC SPREAD PREVIEW</span>
          <Tooltip
            title="Live Spread Scrubber"
            description="Drag to preview how crosshair layers react to simulated weapon spread without needing game input (§12.5)"
          >
            <span class="info-icon" style="cursor: help; font-size: 11px;">ℹ️</span>
          </Tooltip>
        </div>

        <div class="property-row">
          <div class="row">
            <span class="property-label">Simulated Spread</span>
            <div class="val-group">
              <span class="mono" style="font-size: 11px; color: var(--accent); font-weight: 700;">
                {editorState.simulatedSpread.toFixed(1)} px
              </span>
            </div>
          </div>
          <div class="slider-wrapper">
            <input
              type="range"
              class="prop-slider"
              min="0"
              max="15"
              step="0.1"
              bind:value={editorState.simulatedSpread}
              aria-label="Simulated spread scrubber"
            />
            <div class="slider-fill-visual" style="width: {(editorState.simulatedSpread / 15) * 100}%;"></div>
          </div>
        </div>

        <div class="binding-chip">
          <span class="binding-path">layers[0].gap</span>
          <span class="binding-op">+=</span>
          <span class="binding-driver">spread × 0.8</span>
        </div>
      </div>
    </div>
  {:else}
    <div style="color: var(--text-muted); font-size: 12px; padding: 12px 0;">
      Select a layer to inspect its properties.
    </div>
  {/if}
</div>

<style>
  .properties-list {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .property-row {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .property-label {
    cursor: ew-resize;
    font-size: 13px;
    color: var(--text);
    user-select: none;
  }

  .property-label:hover {
    color: var(--accent);
  }

  .property-label.scrubbing {
    color: var(--accent);
    font-weight: 700;
  }

  .input-with-unit {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .prop-number-input {
    width: 64px;
    height: 26px;
    text-align: right;
    padding: 0 6px;
  }

  .unit-label {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-muted);
  }

  .slider-wrapper {
    position: relative;
    height: 6px;
    display: flex;
    align-items: center;
  }

  .prop-slider {
    position: absolute;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
    z-index: 2;
    margin: 0;
  }

  .slider-wrapper::before {
    content: '';
    position: absolute;
    left: 0;
    right: 0;
    height: 4px;
    background: var(--border);
    border-radius: 2px;
  }

  .slider-fill-visual {
    position: absolute;
    left: 0;
    height: 4px;
    background: var(--accent);
    border-radius: 2px;
    pointer-events: none;
  }

  .dynamic-scrubber-section {
    margin-top: 10px;
    padding-top: 12px;
    border-top: 1px dashed var(--border);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .dynamic-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .dyn-badge {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.05em;
    color: var(--accent);
  }

  .binding-chip {
    display: flex;
    align-items: center;
    gap: 6px;
    background: rgba(54, 242, 203, 0.06);
    border: 1px solid rgba(54, 242, 203, 0.2);
    border-radius: 4px;
    padding: 4px 8px;
    font-family: var(--font-mono);
    font-size: 10px;
  }

  .binding-path {
    color: var(--text);
  }

  .binding-op {
    color: var(--accent);
  }

  .binding-driver {
    color: var(--text-muted);
  }

  .prop-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-bottom: 12px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .section-title {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .color-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .swatches {
    display: flex;
    gap: 6px;
    flex: 1;
  }

  .swatch-btn {
    width: 22px;
    height: 22px;
    border-radius: var(--radius-sm);
    border: 2px solid transparent;
    cursor: pointer;
    transition: transform 100ms ease, border-color 100ms ease;
  }

  .swatch-btn:hover {
    transform: scale(1.15);
  }

  .swatch-btn.active {
    border-color: #FFFFFF;
    box-shadow: 0 0 6px rgba(255, 255, 255, 0.6);
  }

  .native-color-picker {
    width: 28px;
    height: 24px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: transparent;
    cursor: pointer;
    padding: 0;
  }

  .path-textarea {
    width: 100%;
    font-size: 11px;
    resize: vertical;
    min-height: 40px;
    box-sizing: border-box;
  }

  .chips-group {
    display: flex;
    gap: 4px;
  }

  .pill-chip {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid var(--border);
    color: var(--text-muted);
    font-size: 10px;
    padding: 2px 6px;
    border-radius: var(--radius-pill);
    cursor: pointer;
    font-family: var(--font-mono);
  }

  .pill-chip:hover {
    color: var(--text);
  }

  .pill-chip.active {
    background: var(--accent-dim);
    color: var(--accent);
    border-color: var(--accent);
    font-weight: 600;
  }
</style>
