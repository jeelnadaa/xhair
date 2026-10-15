<script lang="ts">
  import { editorState, DESCRIPTORS } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  // Scrubbing drag state
  let activeScrubProp = $state<string | null>(null);
  let scrubStartX = $state<number>(0);
  let scrubStartVal = $state<number>(0);

  function getPropertyValue(name: string): number {
    const layer = editorState.selectedLayer;
    if (!layer) return 0;

    if (name === 'line.length') return layer.shape.length ?? 6;
    if (name === 'line.thickness') return layer.shape.thickness ?? 2;
    if (name === 'line.gap') return layer.shape.gap ?? 4;
    if (name === 'line.arm_count') return layer.shape.armCount ?? 4;
    if (name === 'dot.diameter') return layer.shape.diameter ?? 2;
    if (name === 'global.opacity') return layer.opacity ?? 1.0;
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
    const desc = DESCRIPTORS.find((d) => d.name === activeScrubProp);
    if (!desc) return;

    const deltaX = e.clientX - scrubStartX;
    const step = e.altKey ? desc.fine_step : e.shiftKey ? desc.step * 10 : desc.step;
    const stepsMoved = Math.round(deltaX / 5);
    let newVal = scrubStartVal + stepsMoved * step;
    newVal = Math.max(desc.min, Math.min(desc.max, newVal));
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
    {@const shapeType = editorState.selectedLayer.shape.type}
    <div class="properties-list">
      {#each DESCRIPTORS as desc}
        <!-- Filter descriptors relevant to selected layer type -->
        {#if (desc.name.startsWith('line.') && shapeType === 'LineArms') || (desc.name.startsWith('dot.') && shapeType === 'Dot') || desc.name.startsWith('global.')}
          {@const val = getPropertyValue(desc.name)}
          {@const percent = Math.min(100, Math.max(0, ((val - desc.min) / (desc.max - desc.min)) * 100))}

          <div class="property-row">
            <div class="row">
              <Tooltip
                title={desc.label}
                description={desc.tip}
                details="Default: {desc.default_value}{desc.unit ? ` ${desc.unit}` : ''} · Range: {desc.min}–{desc.max}"
              >
                <span
                  class="property-label {activeScrubProp === desc.name ? 'scrubbing' : ''}"
                  onmousedown={(e) => startScrub(desc.name, e)}
                  role="slider"
                  tabindex="0"
                  aria-valuenow={val}
                  aria-valuemin={desc.min}
                  aria-valuemax={desc.max}
                  aria-label={desc.label}
                >
                  {desc.label} ↔
                </span>
              </Tooltip>

              <div class="input-with-unit">
                <input
                  type="number"
                  class="input-text mono prop-number-input"
                  min={desc.min}
                  max={desc.max}
                  step={desc.step}
                  value={val}
                  oninput={(e) => handleNumberChange(desc.name, e)}
                  aria-label="{desc.label} numeric input"
                />
                {#if desc.unit}
                  <span class="unit-label">{desc.unit}</span>
                {/if}
              </div>
            </div>

            <!-- Slider Control with Fill Track -->
            <div class="slider-wrapper">
              <input
                type="range"
                class="prop-slider"
                min={desc.min}
                max={desc.max}
                step={desc.step}
                value={val}
                oninput={(e) => handleSliderChange(desc.name, e)}
                aria-label="{desc.label} slider"
              />
              <div class="slider-fill-visual" style="width: {percent}%;"></div>
            </div>
          </div>
        {/if}
      {/each}

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
</style>
