<script lang="ts">
  import { editorState, DESCRIPTORS } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';
  import ScrubInput from './ScrubInput.svelte';

  // Section Accordion Collapsed States
  let transformOpen = $state(true);
  let geometryOpen = $state(true);
  let fillStrokeOpen = $state(true);
  let effectsOpen = $state(true);
  let stylingOpen = $state(false);

  // Preset Colors
  const PRESET_COLORS = [
    '#36F2CB', // Cyan
    '#00FF00', // Green
    '#FF3864', // Red / Coral
    '#FFFFFF', // White
    '#FFE600', // Yellow
    '#FF4FD8', // Magenta
    '#00B2FF', // Deep Sky Blue
    '#FF9F1C', // Amber
  ];

  const COLORBLIND_COLORS = [
    '#0072B2', // Blue
    '#E69F00', // Orange
    '#56B4E9', // Sky Blue
    '#009E73', // Bluish Green
    '#F0E442', // Yellow
    '#D55E00', // Vermilion
    '#CC79A7', // Reddish Purple
    '#999999', // Gray
  ];

  const SVG_PRESETS = [
    { label: 'Arrow', path: 'M 0,-8 L 6,6 L 0,3 L -6,6 Z' },
    { label: 'Diamond', path: 'M 0,-8 L 8,0 L 0,8 L -8,0 Z' },
    { label: 'Triangle', path: 'M 0,-8 L 7,6 L -7,6 Z' },
    { label: 'Chevron', path: 'M -6,5 L 0,0 L 6,5' },
    { label: 'Crosshair', path: 'M -8,0 L -2,0 M 2,0 L 8,0 M 0,-8 L 0,-2 M 0,2 L 0,8' },
  ];

  let newPresetName = $state('');

  function handleSavePreset() {
    if (!newPresetName.trim()) return;
    editorState.saveStylePreset(newPresetName.trim());
    newPresetName = '';
  }

  function handleColorChange(color: string) {
    editorState.setLayerColor(color, true);
  }
</script>

<aside class="unified-inspector">
  <!-- Inspector Header -->
  <div class="inspector-header">
    <div class="header-left">
      <span class="header-title">Inspector</span>
      {#if editorState.selectedLayer}
        <span class="selection-badge mono">{editorState.selectedLayer.name}</span>
      {:else}
        <span class="selection-badge mono empty">No Selection</span>
      {/if}
    </div>
    {#if editorState.selectedLayers.length > 1}
      <span class="multi-select-badge mono">{editorState.selectedLayers.length} layers</span>
    {/if}
  </div>

  {#if editorState.selectedLayer}
    {@const layer = editorState.selectedLayer}
    {@const shape = layer.shape}

    <div class="inspector-scrollable">
      <!-- ================= 1. TRANSFORM & POSITION ACCORDION ================= -->
      <section class="accordion-card">
        <button
          class="card-header-btn"
          onclick={() => (transformOpen = !transformOpen)}
          aria-expanded={transformOpen}
        >
          <span class="card-title">Transform & Position</span>
          <span class="card-chevron">{transformOpen ? '▼' : '▶'}</span>
        </button>

        {#if transformOpen}
          <div class="card-content">
            <!-- Relative vs Absolute Toggle -->
            <div class="mode-row">
              <span class="sub-label mono">Mode</span>
              <div class="segmented-pill">
                <button
                  class="seg-btn {editorState.positionMode === 'relative' ? 'active' : ''}"
                  onclick={() => (editorState.positionMode = 'relative')}
                >
                  Relative
                </button>
                <button
                  class="seg-btn {editorState.positionMode === 'absolute' ? 'active' : ''}"
                  onclick={() => (editorState.positionMode = 'absolute')}
                >
                  Absolute
                </button>
              </div>
            </div>

            <!-- Position X & Y in 2-Column Grid -->
            <div class="grid-2col">
              <ScrubInput
                label="X"
                value={layer.offsetX}
                step={0.5}
                unit="px"
                tooltip="Horizontal offset from center (drag label to scrub)"
                onchange={(v) => editorState.updateProperty('layer.offset_x', v)}
              />
              <ScrubInput
                label="Y"
                value={layer.offsetY}
                step={0.5}
                unit="px"
                tooltip="Vertical offset from center (drag label to scrub)"
                onchange={(v) => editorState.updateProperty('layer.offset_y', v)}
              />
            </div>

            <!-- Rotation & Opacity in 2-Column Grid -->
            <div class="grid-2col">
              <ScrubInput
                label="Rot"
                value={layer.rotation}
                min={0}
                max={360}
                step={1}
                unit="°"
                tooltip="Layer rotation angle"
                onchange={(v) => editorState.updateProperty('layer.rotation', v)}
              />
              <ScrubInput
                label="Opacity"
                value={Math.round((layer.opacity ?? 1.0) * 100)}
                min={0}
                max={100}
                step={5}
                unit="%"
                tooltip="Layer opacity"
                onchange={(v) => editorState.updateProperty('layer.opacity', v / 100)}
              />
            </div>

            <!-- Recenter Actions -->
            <div class="recenter-row">
              <button
                class="btn btn-sm btn-recenter"
                onclick={() => editorState.recenterSelection()}
                title="Recenter selection to screen center (Home)"
              >
                Recenter Selection (Home)
              </button>
              <button
                class="btn btn-sm btn-recenter"
                onclick={() => editorState.recenterCrosshair()}
                title="Recenter crosshair (Ctrl+Home)"
              >
                Recenter All (Ctrl+Home)
              </button>
            </div>
          </div>
        {/if}
      </section>

      <!-- ================= 2. SHAPE GEOMETRY ACCORDION ================= -->
      <section class="accordion-card">
        <button
          class="card-header-btn"
          onclick={() => (geometryOpen = !geometryOpen)}
          aria-expanded={geometryOpen}
        >
          <span class="card-title">Shape Geometry ({shape.type})</span>
          <span class="card-chevron">{geometryOpen ? '▼' : '▶'}</span>
        </button>

        {#if geometryOpen}
          <div class="card-content">
            {#if shape.type === 'LineArms'}
              <!-- Line Arms: Length, Thickness, Gap, Arm Count -->
              <div class="grid-2col">
                <ScrubInput
                  label="Length"
                  value={shape.length ?? 6}
                  min={1}
                  max={100}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('line.length', v)}
                />
                <ScrubInput
                  label="Thickness"
                  value={shape.thickness ?? 2}
                  min={1}
                  max={30}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('line.thickness', v)}
                />
              </div>
              <div class="grid-2col">
                <ScrubInput
                  label="Gap"
                  value={shape.gap ?? 4}
                  min={0}
                  max={80}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('line.gap', v)}
                />
                <ScrubInput
                  label="Arms"
                  value={shape.armCount ?? 4}
                  min={1}
                  max={8}
                  step={1}
                  unit=""
                  onchange={(v) => editorState.updateProperty('line.arm_count', v)}
                />
              </div>

              <!-- Taper Toggle -->
              <div class="toggle-row">
                <span class="sub-label">Tapered Arms</span>
                <input
                  type="checkbox"
                  checked={shape.taper ?? false}
                  onchange={(e) => {
                    shape.taper = (e.target as HTMLInputElement).checked;
                    editorState.syncWithOverlay();
                  }}
                />
              </div>

            {:else if shape.type === 'Dot'}
              <!-- Center Dot: Diameter, Square vs Round -->
              <div class="grid-2col">
                <ScrubInput
                  label="Diameter"
                  value={shape.diameter ?? 2}
                  min={1}
                  max={40}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('dot.diameter', v)}
                />
                <div class="field-wrap">
                  <span class="sub-label mono">Shape</span>
                  <div class="segmented-pill">
                    <button
                      class="seg-btn {!shape.isSquare ? 'active' : ''}"
                      onclick={() => {
                        shape.isSquare = false;
                        editorState.syncWithOverlay();
                      }}
                    >
                      Round
                    </button>
                    <button
                      class="seg-btn {shape.isSquare ? 'active' : ''}"
                      onclick={() => {
                        shape.isSquare = true;
                        editorState.syncWithOverlay();
                      }}
                    >
                      Square
                    </button>
                  </div>
                </div>
              </div>

            {:else if shape.type === 'Rect'}
              <!-- Rectangle: Width, Height, Corner Radii -->
              <div class="grid-2col">
                <ScrubInput
                  label="Width"
                  value={shape.width ?? 8}
                  min={1}
                  max={120}
                  step={1}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('shape.width', v)}
                />
                <ScrubInput
                  label="Height"
                  value={shape.height ?? 8}
                  min={1}
                  max={120}
                  step={1}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('shape.height', v)}
                />
              </div>
              <ScrubInput
                label="Corner Radius"
                value={shape.cornerRadius ?? 0}
                min={0}
                max={40}
                step={0.5}
                unit="px"
                onchange={(v) => editorState.updateProperty('shape.corner_radius', v)}
              />

            {:else if shape.type === 'Circle'}
              <!-- Circle / Ring: Radius, Thickness, Center Hole -->
              <div class="grid-2col">
                <ScrubInput
                  label="Radius"
                  value={shape.radius ?? 8}
                  min={1}
                  max={80}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('shape.radius', v)}
                />
                <ScrubInput
                  label="Thickness"
                  value={shape.thickness ?? 2}
                  min={1}
                  max={30}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('shape.thickness', v)}
                />
              </div>
              <ScrubInput
                label="Center Hole"
                value={shape.centerHole ?? 0}
                min={0}
                max={50}
                step={0.5}
                unit="px"
                onchange={(v) => editorState.updateProperty('shape.center_hole', v)}
              />

            {:else if shape.type === 'TShape'}
              <!-- T-Shape: Length, Thickness, Gap -->
              <div class="grid-2col">
                <ScrubInput
                  label="Length"
                  value={shape.length ?? 6}
                  min={1}
                  max={80}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('line.length', v)}
                />
                <ScrubInput
                  label="Thickness"
                  value={shape.thickness ?? 2}
                  min={1}
                  max={20}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('line.thickness', v)}
                />
              </div>
              <ScrubInput
                label="Gap"
                value={shape.gap ?? 4}
                min={0}
                max={60}
                step={0.5}
                unit="px"
                onchange={(v) => editorState.updateProperty('line.gap', v)}
              />

            {:else if shape.type === 'Polygon' || shape.type === 'Star'}
              <!-- Polygon / Star: Points, Radius, Inner Radius -->
              <div class="grid-2col">
                <ScrubInput
                  label="Points"
                  value={shape.points ?? 5}
                  min={3}
                  max={16}
                  step={1}
                  unit=""
                  onchange={(v) => editorState.updateProperty('shape.points', v)}
                />
                <ScrubInput
                  label="Radius"
                  value={shape.radius ?? 10}
                  min={2}
                  max={80}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('shape.radius', v)}
                />
              </div>
              {#if shape.type === 'Star'}
                <ScrubInput
                  label="Inner Rad"
                  value={shape.innerRadius ?? 4}
                  min={1}
                  max={50}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('shape.inner_radius', v)}
                />
              {/if}

            {:else if shape.type === 'Arc'}
              <!-- Arc Segment -->
              <div class="grid-2col">
                <ScrubInput
                  label="Radius"
                  value={shape.radius ?? 8}
                  min={2}
                  max={80}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('shape.radius', v)}
                />
                <ScrubInput
                  label="Thickness"
                  value={shape.thickness ?? 2}
                  min={1}
                  max={20}
                  step={0.5}
                  unit="px"
                  onchange={(v) => editorState.updateProperty('shape.thickness', v)}
                />
              </div>
              <div class="grid-2col">
                <ScrubInput
                  label="Start Angle"
                  value={shape.startAngle ?? 0}
                  min={0}
                  max={360}
                  step={5}
                  unit="°"
                  onchange={(v) => editorState.updateProperty('shape.start_angle', v)}
                />
                <ScrubInput
                  label="End Angle"
                  value={shape.endAngle ?? 180}
                  min={0}
                  max={360}
                  step={5}
                  unit="°"
                  onchange={(v) => editorState.updateProperty('shape.end_angle', v)}
                />
              </div>

            {:else if shape.type === 'SvgPath'}
              <!-- Custom SVG Vector Path -->
              <div class="svg-presets-row">
                {#each SVG_PRESETS as preset}
                  <button
                    class="btn btn-sm btn-preset"
                    onclick={() => {
                      shape.pathData = preset.path;
                      editorState.syncWithOverlay();
                    }}
                  >
                    {preset.label}
                  </button>
                {/each}
              </div>
              <textarea
                class="input-text mono svg-textarea"
                rows="3"
                placeholder="SVG path data (e.g. M 0,-8 L 6,6 Z)"
                bind:value={shape.pathData}
                oninput={() => editorState.syncWithOverlay()}
              ></textarea>
            {/if}
          </div>
        {/if}
      </section>

      <!-- ================= 3. FILL & STROKE ACCORDION ================= -->
      <section class="accordion-card">
        <button
          class="card-header-btn"
          onclick={() => (fillStrokeOpen = !fillStrokeOpen)}
          aria-expanded={fillStrokeOpen}
        >
          <span class="card-title">Fill & Stroke</span>
          <span class="card-chevron">{fillStrokeOpen ? '▼' : '▶'}</span>
        </button>

        {#if fillStrokeOpen}
          <div class="card-content">
            <!-- Curated Palette Swatches -->
            <div class="palette-group">
              <span class="sub-label mono">Tactical High-Contrast</span>
              <div class="swatches-row">
                {#each PRESET_COLORS as c}
                  <button
                    class="swatch-btn {layer.color === c ? 'active' : ''}"
                    style="background: {c};"
                    onclick={() => handleColorChange(c)}
                    aria-label="Color {c}"
                  ></button>
                {/each}
              </div>

              <span class="sub-label mono" style="margin-top: 6px;">Colorblind Safe</span>
              <div class="swatches-row">
                {#each COLORBLIND_COLORS as c}
                  <button
                    class="swatch-btn {layer.color === c ? 'active' : ''}"
                    style="background: {c};"
                    onclick={() => handleColorChange(c)}
                    aria-label="Colorblind {c}"
                  ></button>
                {/each}
              </div>
            </div>

            <!-- Custom Color Picker Row -->
            <div class="color-picker-row">
              <div class="color-preview" style="background: {layer.color};"></div>
              <input
                type="text"
                class="input-text mono hex-input"
                value={layer.color}
                onchange={(e) => handleColorChange((e.target as HTMLInputElement).value)}
                placeholder="#36F2CB"
              />
              <input
                type="color"
                class="native-color-btn"
                value={layer.color}
                oninput={(e) => handleColorChange((e.target as HTMLInputElement).value)}
                aria-label="Color picker"
              />
            </div>

            <!-- Line Cap & Join -->
            <div class="grid-2col">
              <div class="field-wrap">
                <span class="sub-label mono">Line Cap</span>
                <select
                  class="select-dropdown mono"
                  value={shape.strokeCap ?? 'butt'}
                  onchange={(e) => {
                    shape.strokeCap = (e.target as HTMLSelectElement).value as any;
                    editorState.syncWithOverlay();
                  }}
                >
                  <option value="butt">Butt</option>
                  <option value="round">Round</option>
                  <option value="square">Square</option>
                </select>
              </div>

              <div class="field-wrap">
                <span class="sub-label mono">Line Join</span>
                <select
                  class="select-dropdown mono"
                  value={shape.strokeJoin ?? 'miter'}
                  onchange={(e) => {
                    shape.strokeJoin = (e.target as HTMLSelectElement).value as any;
                    editorState.syncWithOverlay();
                  }}
                >
                  <option value="miter">Miter</option>
                  <option value="round">Round</option>
                  <option value="bevel">Bevel</option>
                </select>
              </div>
            </div>
          </div>
        {/if}
      </section>

      <!-- ================= 4. VISUAL EFFECTS & DYNAMICS ACCORDION ================= -->
      <section class="accordion-card">
        <button
          class="card-header-btn"
          onclick={() => (effectsOpen = !effectsOpen)}
          aria-expanded={effectsOpen}
        >
          <span class="card-title">Visual Effects & Dynamics</span>
          <span class="card-chevron">{effectsOpen ? '▼' : '▶'}</span>
        </button>

        {#if effectsOpen}
          <div class="card-content">
            <!-- Glow & Bloom -->
            <div class="grid-2col">
              <ScrubInput
                label="Glow"
                value={layer.glowRadius ?? 0}
                min={0}
                max={30}
                step={0.5}
                unit="px"
                tooltip="Per-layer glow halo radius"
                onchange={(v) => editorState.updateProperty('effect.glow_radius', v)}
              />
              <ScrubInput
                label="Shadow"
                value={layer.shadowBlur ?? 0}
                min={0}
                max={30}
                step={0.5}
                unit="px"
                tooltip="Drop shadow blur radius"
                onchange={(v) => editorState.updateProperty('effect.shadow_blur', v)}
              />
            </div>

            <!-- Dynamic Spread Scrubber -->
            <div class="spread-test-section">
              <div class="spread-header">
                <span class="sub-label mono">Simulated Spread Preview</span>
                <span class="mono" style="font-size: 11px; color: var(--accent);">
                  +{((editorState.simulatedSpread - 1.2) * 2).toFixed(1)}px
                </span>
              </div>
              <ScrubInput
                label="Recoil"
                value={editorState.simulatedSpread}
                min={1.0}
                max={15.0}
                step={0.2}
                unit="°"
                tooltip="Simulate weapon recoil gap expansion dynamically"
                onchange={(v) => (editorState.simulatedSpread = v)}
              />
            </div>
          </div>
        {/if}
      </section>

      <!-- ================= 5. STYLE PRESETS & ACTIONS ACCORDION ================= -->
      <section class="accordion-card">
        <button
          class="card-header-btn"
          onclick={() => (stylingOpen = !stylingOpen)}
          aria-expanded={stylingOpen}
        >
          <span class="card-title">Style Presets & Utilities</span>
          <span class="card-chevron">{stylingOpen ? '▼' : '▶'}</span>
        </button>

        {#if stylingOpen}
          <div class="card-content">
            <!-- Save Preset Form -->
            <div class="preset-create-row">
              <input
                type="text"
                class="input-text mono"
                placeholder="New preset name…"
                bind:value={newPresetName}
                onkeydown={(e) => {
                  if (e.key === 'Enter') handleSavePreset();
                }}
              />
              <button class="btn btn-sm btn-primary" onclick={handleSavePreset}>
                Save
              </button>
            </div>

            <!-- Style Presets List -->
            {#if editorState.stylePresets.length > 0}
              <div class="presets-chips">
                {#each editorState.stylePresets as preset}
                  <button
                    class="preset-chip mono"
                    onclick={() => editorState.applyStylePreset(preset.name)}
                    title="Apply style: {preset.name}"
                  >
                    {preset.name}
                  </button>
                {/each}
              </div>
            {/if}

            <!-- Randomize & Utility Actions -->
            <div class="util-actions-row">
              <button
                class="btn btn-sm"
                onclick={() => editorState.randomizeLayer(layer.id)}
                title="Procedurally randomize this layer"
              >
                🎲 Randomize Layer
              </button>
            </div>
          </div>
        {/if}
      </section>
    </div>
  {:else}
    <!-- Empty State -->
    <div class="empty-inspector">
      <span class="empty-icon">🎯</span>
      <span class="empty-title">No Layer Selected</span>
      <p class="empty-text">
        Click any shape in the canvas or layers rail to inspect and edit its transform, geometry, and styling.
      </p>
    </div>
  {/if}
</aside>

<style>
  .unified-inspector {
    width: 320px;
    flex: none;
    border-left: 1px solid var(--border, #262930);
    background: var(--surface1, #0f1013);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    user-select: none;
  }

  .inspector-header {
    height: 40px;
    border-bottom: 1px solid var(--border, #262930);
    background: var(--surface2, #16181d);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    flex-shrink: 0;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 8px;
    overflow: hidden;
  }

  .header-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text, #f0f6fc);
  }

  .selection-badge {
    font-size: 11px;
    color: var(--accent, #38bdf8);
    background: rgba(56, 189, 248, 0.1);
    padding: 2px 6px;
    border-radius: var(--radius-xs, 4px);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 140px;
  }

  .selection-badge.empty {
    color: var(--text-muted, #8b949e);
    background: var(--surface3, #1e2128);
  }

  .multi-select-badge {
    font-size: 10px;
    color: var(--text-muted, #8b949e);
    background: var(--surface3, #1e2128);
    padding: 2px 6px;
    border-radius: 99px;
  }

  .inspector-scrollable {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1px;
    background: var(--border, #262930);
  }

  .accordion-card {
    background: var(--surface1, #0f1013);
    display: flex;
    flex-direction: column;
  }

  .card-header-btn {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    background: transparent;
    border: none;
    color: var(--text, #f0f6fc);
    cursor: pointer;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    transition: background 100ms ease;
  }

  .card-header-btn:hover {
    background: var(--surface2, #16181d);
  }

  .card-chevron {
    font-size: 10px;
    color: var(--text-muted, #8b949e);
  }

  .card-content {
    padding: 0 12px 12px 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .grid-2col {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .mode-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .sub-label {
    font-size: 10px;
    color: var(--text-muted, #8b949e);
    letter-spacing: 0.03em;
  }

  .segmented-pill {
    display: flex;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-xs, 4px);
    padding: 1px;
  }

  .seg-btn {
    background: transparent;
    border: none;
    color: var(--text-muted, #8b949e);
    font-size: 10px;
    padding: 2px 8px;
    border-radius: 3px;
    cursor: pointer;
  }

  .seg-btn.active {
    background: var(--accent, #38bdf8);
    color: #000;
    font-weight: 600;
  }

  .field-wrap {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .recenter-row {
    display: flex;
    gap: 6px;
    margin-top: 2px;
  }

  .btn-recenter {
    flex: 1;
    font-size: 10px;
    height: 24px;
    padding: 0 6px;
  }

  .toggle-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 0;
  }

  .svg-presets-row {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }

  .btn-preset {
    font-size: 10px;
    padding: 2px 6px;
    height: 22px;
  }

  .svg-textarea {
    width: 100%;
    font-size: 11px;
    resize: vertical;
  }

  .palette-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .swatches-row {
    display: flex;
    gap: 5px;
    flex-wrap: wrap;
  }

  .swatch-btn {
    width: 22px;
    height: 22px;
    border-radius: var(--radius-xs, 4px);
    border: 1px solid rgba(255, 255, 255, 0.15);
    cursor: pointer;
    transition: transform 100ms ease, box-shadow 100ms ease;
  }

  .swatch-btn:hover {
    transform: scale(1.1);
  }

  .swatch-btn.active {
    border-color: #fff;
    box-shadow: 0 0 0 2px var(--accent, #38bdf8);
  }

  .color-picker-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .color-preview {
    width: 26px;
    height: 26px;
    border-radius: var(--radius-xs, 4px);
    border: 1px solid var(--border, #262930);
    flex-shrink: 0;
  }

  .hex-input {
    flex: 1;
    height: 28px;
    font-size: 11px;
  }

  .native-color-btn {
    width: 28px;
    height: 28px;
    padding: 0;
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-xs, 4px);
    background: transparent;
    cursor: pointer;
  }

  .select-dropdown {
    height: 28px;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    color: var(--text, #f0f6fc);
    font-size: 11px;
    padding: 0 6px;
  }

  .spread-test-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    padding: 8px;
  }

  .spread-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .preset-create-row {
    display: flex;
    gap: 6px;
  }

  .presets-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }

  .preset-chip {
    font-size: 10px;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    color: var(--text, #f0f6fc);
    padding: 2px 6px;
    border-radius: var(--radius-xs, 4px);
    cursor: pointer;
  }

  .preset-chip:hover {
    border-color: var(--accent, #38bdf8);
  }

  .util-actions-row {
    display: flex;
    gap: 6px;
  }

  .empty-inspector {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 24px;
    text-align: center;
    color: var(--text-muted, #8b949e);
  }

  .empty-icon {
    font-size: 32px;
    margin-bottom: 12px;
    opacity: 0.6;
  }

  .empty-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text, #f0f6fc);
    margin-bottom: 6px;
  }

  .empty-text {
    font-size: 11px;
    line-height: 1.5;
    margin: 0;
  }
</style>
