<script lang="ts">
  import { editorState, DESCRIPTORS } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';
  import LayersPanel from './LayersPanel.svelte';
  import NudgePad from './NudgePad.svelte';
  import ColorPicker from './ColorPicker.svelte';

  const DOCK_TABS = [
    'Design',
    'Shape',
    'Fill & Stroke',
    'Effects',
    'Color',
    'Layers',
    'Nudge',
    'Animation',
    'Presets',
    'Properties',
    'MCP Activity',
  ];

  const LAYOUT_PRESETS = ['Designer', 'Compact', 'Pixel Art', 'Agent Watch', 'Minimal'];

  let layoutMenuOpen = $state(false);

  // Property search in Properties tab
  let searchQuery = $state('');
  let modifiedOnly = $state(false);

  // Style preset input
  let newPresetName = $state('');

  function getDescriptorValue(name: string): number {
    const layer = editorState.selectedLayer;
    if (!layer) return 0;
    if (name === 'line.thickness') return layer.shape.thickness ?? 2;
    if (name === 'line.length') return layer.shape.length ?? 6;
    if (name === 'line.gap') return layer.shape.gap ?? 4;
    if (name === 'line.arm_count') return layer.shape.armCount ?? 4;
    if (name === 'dot.diameter') return layer.shape.diameter ?? 2;
    if (name === 'rect.width') return layer.shape.width ?? 8;
    if (name === 'rect.height') return layer.shape.height ?? 8;
    if (name === 'ring.radius') return layer.shape.radius ?? 10;
    if (name === 'polygon.points') return layer.shape.points ?? 6;
    if (name === 'layer.offset_x') return layer.offsetX;
    if (name === 'layer.offset_y') return layer.offsetY;
    if (name === 'layer.rotation') return layer.rotation;
    if (name === 'layer.opacity') return layer.opacity;
    if (name === 'effect.glow_radius') return layer.glowRadius ?? 0;
    if (name === 'effect.shadow_blur') return layer.shadowBlur ?? 0;
    return 0;
  }

  function handleSavePreset() {
    if (!newPresetName.trim()) return;
    editorState.saveStylePreset(newPresetName.trim());
    newPresetName = '';
  }
</script>

<aside class="dock-system" class:maximized={editorState.isDockMaximized}>
  <!-- Top Dock Tab Bar -->
  <div class="dock-header">
    <div class="dock-tabs-scroller">
      {#each editorState.dockTabs as tab}
        <button
          class="dock-tab-btn"
          class:active={editorState.activeDockTab === tab}
          onclick={() => editorState.setActiveDockTab(tab)}
        >
          {tab}
        </button>
      {/each}
    </div>

    <!-- Layout Presets & Maximize Controls -->
    <div class="dock-controls">
      <div class="layout-dropdown-wrapper">
        <button
          class="btn btn-ghost btn-icon"
          title="Layout Presets (Compact, Pixel Art, Designer, Agent Watch)"
          onclick={() => (layoutMenuOpen = !layoutMenuOpen)}
        >
          <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
            <path d="M1 3.5A1.5 1.5 0 0 1 2.5 2h11A1.5 1.5 0 0 1 15 3.5v9a1.5 1.5 0 0 1-1.5 1.5h-11A1.5 1.5 0 0 1 1 12.5v-9zM2.5 3a.5.5 0 0 0-.5.5v9a.5.5 0 0 0 .5.5h11a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5h-11zM10 3.5v9H14v-9H10z"/>
          </svg>
        </button>

        {#if layoutMenuOpen}
          <div class="layout-menu" onclick={() => (layoutMenuOpen = false)} role="presentation">
            <div class="menu-label">Dock Layout Presets</div>
            {#each LAYOUT_PRESETS as preset}
              <button
                class="menu-item"
                class:active={editorState.dockLayoutPreset === preset}
                onclick={() => editorState.applyDockLayout(preset)}
              >
                {preset}
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <button
        class="btn btn-ghost btn-icon"
        title="Maximize Dock (Ctrl+Space)"
        onclick={() => editorState.toggleDockMaximize()}
      >
        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
          {#if editorState.isDockMaximized}
            <path d="M5.5 0a.5.5 0 0 1 .5.5v4A1.5 1.5 0 0 1 4.5 6h-4a.5.5 0 0 1 0-1h4a.5.5 0 0 0 .5-.5v-4a.5.5 0 0 1 .5-.5zm5 0a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 1 0 1h-4A1.5 1.5 0 0 1 10 4.5v-4a.5.5 0 0 1 .5-.5zM0 10.5a.5.5 0 0 1 .5-.5h4A1.5 1.5 0 0 1 6 11.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 0-.5-.5h-4a.5.5 0 0 1-.5-.5zm10 1a1.5 1.5 0 0 1 1.5-1.5h4a.5.5 0 0 1 0 1h-4a.5.5 0 0 0-.5.5v4a.5.5 0 0 1-1 0v-4z"/>
          {:else}
            <path d="M1.5 1a.5.5 0 0 0-.5.5v4a.5.5 0 0 1-1 0v-4A1.5 1.5 0 0 1 1.5 0h4a.5.5 0 0 1 0 1h-4zM10 .5a.5.5 0 0 1 .5-.5h4A1.5 1.5 0 0 1 16 1.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 0-.5-.5h-4a.5.5 0 0 1-.5-.5zM.5 10a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 1 0 1h-4A1.5 1.5 0 0 1 0 14.5v-4a.5.5 0 0 1 .5-.5zm15 0a.5.5 0 0 1 .5.5v4A1.5 1.5 0 0 1 14.5 16h-4a.5.5 0 0 1 0-1h4a.5.5 0 0 0 .5-.5v-4a.5.5 0 0 1 .5-.5z"/>
          {/if}
        </svg>
      </button>
    </div>
  </div>

  <!-- Dock Body -->
  <div class="dock-content">
    {#if editorState.activeDockTab === 'Design'}
      <div class="tab-pane">
        <!-- Position & Nudge Coordinates (§5) -->
        <div class="pane-section">
          <div class="section-title-row">
            <span class="pane-title">Position & Offset (Game Px)</span>
            <div class="segmented-control mini-segmented">
              <button
                class="btn seg-btn {editorState.positionMode === 'relative' ? 'btn-selected' : ''}"
                onclick={() => (editorState.positionMode = 'relative')}
              >
                Rel
              </button>
              <button
                class="btn seg-btn {editorState.positionMode === 'absolute' ? 'btn-selected' : ''}"
                onclick={() => (editorState.positionMode = 'absolute')}
              >
                Abs
              </button>
            </div>
          </div>

          {#if editorState.selectedLayer}
            {@const layer = editorState.selectedLayer}
            <div class="grid-2col">
              <div class="input-block">
                <span class="input-label">X Offset</span>
                <div class="input-with-unit">
                  <input
                    type="number"
                    class="input-text mono"
                    step="0.5"
                    value={layer.offsetX}
                    oninput={(e) => editorState.updateProperty('layer.offset_x', parseFloat((e.target as HTMLInputElement).value) || 0)}
                  />
                  <span class="unit-label">px</span>
                </div>
              </div>
              <div class="input-block">
                <span class="input-label">Y Offset</span>
                <div class="input-with-unit">
                  <input
                    type="number"
                    class="input-text mono"
                    step="0.5"
                    value={layer.offsetY}
                    oninput={(e) => editorState.updateProperty('layer.offset_y', parseFloat((e.target as HTMLInputElement).value) || 0)}
                  />
                  <span class="unit-label">px</span>
                </div>
              </div>
            </div>

            <!-- Recenter Selection / Whole Crosshair (§6) -->
            <div class="recenter-row">
              <button class="btn btn-sm" onclick={() => editorState.recenterSelection()}>
                Recenter Selection (Home)
              </button>
              <button class="btn btn-sm" onclick={() => editorState.recenterCrosshair()}>
                Recenter Crosshair (Ctrl+Home)
              </button>
            </div>

            {#if editorState.isMovedFromCenter}
              <div class="notice-box mono">
                Offset from center: ({editorState.crosshair.offsetX.toFixed(1)}, {editorState.crosshair.offsetY.toFixed(1)})
              </div>
            {/if}
          {/if}
        </div>

        <!-- Transform: Rotation & Scale -->
        <div class="pane-section">
          <span class="pane-title">Transform</span>
          {#if editorState.selectedLayer}
            {@const layer = editorState.selectedLayer}
            <div class="input-block">
              <div class="row">
                <span class="input-label">Rotation</span>
                <span class="mono micro-text">{layer.rotation}°</span>
              </div>
              <input
                type="range"
                min="0"
                max="360"
                step="1"
                class="prop-slider"
                value={layer.rotation}
                oninput={(e) => editorState.updateProperty('layer.rotation', parseFloat((e.target as HTMLInputElement).value) || 0)}
              />
            </div>
            <div class="input-block" style="margin-top: 8px;">
              <div class="row">
                <span class="input-label">Opacity</span>
                <span class="mono micro-text">{Math.round(layer.opacity * 100)}%</span>
              </div>
              <input
                type="range"
                min="0"
                max="1"
                step="0.05"
                class="prop-slider"
                value={layer.opacity}
                oninput={(e) => editorState.updateProperty('layer.opacity', parseFloat((e.target as HTMLInputElement).value) || 1)}
              />
            </div>
          {/if}
        </div>
      </div>

    {:else if editorState.activeDockTab === 'Shape'}
      <div class="tab-pane">
        {#if editorState.selectedLayer}
          {@const layer = editorState.selectedLayer}
          {@const shape = layer.shape}
          <div class="pane-section">
            <span class="pane-title">Shape Properties: {shape.type}</span>

            {#if shape.type === 'LineArms'}
              <div class="prop-control">
                <div class="row">
                  <span class="input-label">Thickness</span>
                  <span class="mono micro-text">{shape.thickness} px</span>
                </div>
                <input
                  type="range"
                  min="0.25"
                  max="16"
                  step="0.25"
                  class="prop-slider"
                  value={shape.thickness}
                  oninput={(e) => editorState.updateProperty('line.thickness', parseFloat((e.target as HTMLInputElement).value) || 1)}
                />
              </div>

              <div class="prop-control">
                <div class="row">
                  <span class="input-label">Length</span>
                  <span class="mono micro-text">{shape.length} px</span>
                </div>
                <input
                  type="range"
                  min="1"
                  max="32"
                  step="0.5"
                  class="prop-slider"
                  value={shape.length}
                  oninput={(e) => editorState.updateProperty('line.length', parseFloat((e.target as HTMLInputElement).value) || 1)}
                />
              </div>

              <div class="prop-control">
                <div class="row">
                  <span class="input-label">Gap</span>
                  <span class="mono micro-text">{shape.gap} px</span>
                </div>
                <input
                  type="range"
                  min="0"
                  max="32"
                  step="0.5"
                  class="prop-slider"
                  value={shape.gap}
                  oninput={(e) => editorState.updateProperty('line.gap', parseFloat((e.target as HTMLInputElement).value) || 0)}
                />
              </div>

            {:else if shape.type === 'Dot'}
              <div class="prop-control">
                <div class="row">
                  <span class="input-label">Diameter</span>
                  <span class="mono micro-text">{shape.diameter} px</span>
                </div>
                <input
                  type="range"
                  min="1"
                  max="16"
                  step="0.5"
                  class="prop-slider"
                  value={shape.diameter}
                  oninput={(e) => editorState.updateProperty('dot.diameter', parseFloat((e.target as HTMLInputElement).value) || 1)}
                />
              </div>
              <div class="checkbox-row" style="margin-top: 10px;">
                <label class="checkbox-label">
                  <input
                    type="checkbox"
                    checked={shape.isSquare}
                    onchange={(e) => (shape.isSquare = (e.target as HTMLInputElement).checked)}
                  />
                  <span>Square pixel dot</span>
                </label>
              </div>

            {:else}
              <div class="notice-box mono">
                Micro properties configured for {shape.type}
              </div>
            {/if}
          </div>
        {:else}
          <div class="empty-state">No layer selected</div>
        {/if}
      </div>

    {:else if editorState.activeDockTab === 'Fill & Stroke'}
      <div class="tab-pane">
        <div class="pane-section">
          <span class="pane-title">Paint & Stroke</span>
          {#if editorState.selectedLayer}
            {@const layer = editorState.selectedLayer}
            <div class="input-block">
              <span class="input-label">Color Hex (#RRGGBBAA)</span>
              <input
                type="text"
                class="input-text mono"
                value={layer.color}
                oninput={(e) => editorState.setLayerColor((e.target as HTMLInputElement).value)}
              />
            </div>

            <div class="swatches-grid" style="margin-top: 10px;">
              {#each editorState.savedSwatches as hex}
                <button
                  class="swatch-btn"
                  style="background: {hex};"
                  onclick={() => editorState.setLayerColor(hex)}
                  aria-label="Color {hex}"
                ></button>
              {/each}
            </div>
          {/if}
        </div>
      </div>

    {:else if editorState.activeDockTab === 'Effects'}
      <div class="tab-pane">
        <div class="pane-section">
          <span class="pane-title">Glow & Shadow Effects</span>
          {#if editorState.selectedLayer}
            {@const layer = editorState.selectedLayer}
            <div class="prop-control">
              <div class="row">
                <span class="input-label">Glow Radius</span>
                <span class="mono micro-text">{layer.glowRadius || 0} px</span>
              </div>
              <input
                type="range"
                min="0"
                max="16"
                step="0.5"
                class="prop-slider"
                value={layer.glowRadius || 0}
                oninput={(e) => editorState.updateProperty('effect.glow_radius', parseFloat((e.target as HTMLInputElement).value) || 0)}
              />
            </div>
            <div class="prop-control" style="margin-top: 10px;">
              <div class="row">
                <span class="input-label">Shadow Blur</span>
                <span class="mono micro-text">{layer.shadowBlur || 0} px</span>
              </div>
              <input
                type="range"
                min="0"
                max="16"
                step="0.5"
                class="prop-slider"
                value={layer.shadowBlur || 0}
                oninput={(e) => editorState.updateProperty('effect.shadow_blur', parseFloat((e.target as HTMLInputElement).value) || 0)}
              />
            </div>
          {/if}
        </div>
      </div>

    {:else if editorState.activeDockTab === 'Color'}
      <div class="tab-pane">
        <ColorPicker />
      </div>

    {:else if editorState.activeDockTab === 'Layers'}
      <div class="tab-pane">
        <LayersPanel />
      </div>

    {:else if editorState.activeDockTab === 'Nudge'}
      <div class="tab-pane">
        <NudgePad />
      </div>

    {:else if editorState.activeDockTab === 'Presets'}
      <div class="tab-pane">
        <div class="pane-section">
          <span class="pane-title">Saved Style Presets</span>
          <div class="presets-list">
            {#each editorState.stylePresets as preset}
              <div class="preset-card">
                <div class="preset-header">
                  <span class="swatch" style="background: {preset.fill};"></span>
                  <span class="preset-name">{preset.name}</span>
                </div>
                <div class="preset-actions">
                  <button class="btn btn-sm btn-ghost" onclick={() => editorState.applyStylePreset(preset.name)}>Apply</button>
                  <button class="btn btn-sm btn-ghost" onclick={() => editorState.deleteStylePreset(preset.name)}>Delete</button>
                </div>
              </div>
            {/each}
          </div>

          <div class="save-preset-row" style="margin-top: 12px;">
            <input
              type="text"
              class="input-text"
              placeholder="Preset Name..."
              bind:value={newPresetName}
            />
            <button class="btn btn-primary" onclick={handleSavePreset}>Save Current</button>
          </div>

          <div style="margin-top: 16px;">
            <button class="btn btn-secondary w-full" onclick={() => editorState.randomizeCrosshair()}>
              🎲 Randomize Variations
            </button>
          </div>
        </div>
      </div>

    {:else if editorState.activeDockTab === 'Properties'}
      <div class="tab-pane">
        <div class="pane-section">
          <!-- Property Search Filter (§9.1) -->
          <div class="search-filter-row">
            <input
              type="text"
              class="input-text search-input"
              placeholder="Search properties (name, unit, tag)..."
              bind:value={searchQuery}
            />
            <label class="checkbox-label" style="margin-top: 6px;">
              <input type="checkbox" bind:checked={modifiedOnly} />
              <span>Show modified only</span>
            </label>
          </div>

          <!-- Catalog List -->
          <div class="descriptors-catalog">
            {#each DESCRIPTORS as desc}
              {#if !searchQuery || desc.name.toLowerCase().includes(searchQuery.toLowerCase()) || desc.label.toLowerCase().includes(searchQuery.toLowerCase())}
                <div class="descriptor-row">
                  <div class="row">
                    <span class="desc-label" title={desc.tip}>{desc.label}</span>
                    <span class="mono desc-val">{getDescriptorValue(desc.name)} {desc.unit || ''}</span>
                  </div>
                  <div class="row desc-meta">
                    <span class="mono micro-text">{desc.name}</span>
                    <button
                      class="reset-btn"
                      title="Reset to default ({desc.default_value})"
                      onclick={() => editorState.updateProperty(desc.name, desc.default_value)}
                    >
                      Reset
                    </button>
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        </div>
      </div>

    {:else if editorState.activeDockTab === 'MCP Activity'}
      <div class="tab-pane">
        <div class="pane-section">
          <div class="section-title-row">
            <span class="pane-title">MCP Protocol Tool Log</span>
            <span class="badge">127.0.0.1:4444</span>
          </div>

          <div class="mcp-log-list">
            {#each editorState.mcpActivityLog as log}
              <div class="mcp-log-card">
                <div class="mcp-log-header">
                  <span class="mono log-tool">{log.tool}</span>
                  <span class="badge {log.status === 'error' ? 'badge-danger' : ''}">{log.status}</span>
                </div>
                <div class="log-summary">{log.summary}</div>
                <div class="log-time mono">{log.timestamp}</div>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  </div>
</aside>

<style>
  .dock-system {
    width: 320px;
    height: 100%;
    background: var(--surface);
    border-left: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    transition: width 150ms ease;
  }

  .dock-system.maximized {
    width: 520px;
  }

  .dock-header {
    height: 38px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px;
    background: var(--surface1);
  }

  .dock-tabs-scroller {
    display: flex;
    overflow-x: auto;
    scrollbar-width: none;
    gap: 2px;
  }

  .dock-tabs-scroller::-webkit-scrollbar {
    display: none;
  }

  .dock-tab-btn {
    padding: 8px 10px;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
    transition: color 100ms ease, border-color 100ms ease;
  }

  .dock-tab-btn:hover {
    color: var(--text);
  }

  .dock-tab-btn.active {
    color: var(--text);
    border-bottom-color: var(--accent);
    font-weight: 600;
  }

  .dock-controls {
    display: flex;
    align-items: center;
    gap: 2px;
    position: relative;
  }

  .layout-dropdown-wrapper {
    position: relative;
  }

  .layout-menu {
    position: absolute;
    top: 32px;
    right: 0;
    width: 160px;
    background: var(--surface);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-dropdown);
    z-index: 100;
    padding: 4px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .menu-label {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    color: var(--text-dim);
    padding: 4px 8px;
  }

  .menu-item {
    padding: 6px 8px;
    font-size: 11px;
    color: var(--text);
    background: transparent;
    border: none;
    border-radius: var(--radius-xs);
    cursor: pointer;
    text-align: left;
  }

  .menu-item:hover,
  .menu-item.active {
    background: var(--hover);
  }

  .dock-content {
    flex: 1;
    overflow-y: auto;
    padding: 12px;
  }

  .tab-pane {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .pane-section {
    background: var(--surface1);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 12px;
  }

  .pane-title {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
    margin-bottom: 10px;
    display: block;
  }

  .section-title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .grid-2col {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .input-block {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .input-label {
    font-size: 11px;
    color: var(--text-muted);
  }

  .input-with-unit {
    position: relative;
    display: flex;
    align-items: center;
  }

  .input-with-unit input {
    width: 100%;
    padding-right: 24px;
  }

  .unit-label {
    position: absolute;
    right: 8px;
    font-size: 10px;
    color: var(--text-dim);
    pointer-events: none;
  }

  .recenter-row {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 10px;
  }

  .notice-box {
    margin-top: 8px;
    padding: 6px 8px;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    font-size: 10px;
    color: var(--warning);
  }

  .prop-control {
    margin-top: 8px;
  }

  .prop-slider {
    width: 100%;
    margin-top: 4px;
  }

  .micro-text {
    font-size: 10px;
    color: var(--text-dim);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: var(--text);
    cursor: pointer;
  }

  .swatches-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .swatch-btn {
    width: 22px;
    height: 22px;
    border-radius: var(--radius-xs);
    border: 1px solid var(--border);
    cursor: pointer;
  }

  .presets-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .preset-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 8px;
    background: var(--raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
  }

  .preset-header {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .preset-name {
    font-size: 11px;
    font-weight: 600;
    color: var(--text);
  }

  .preset-actions {
    display: flex;
    gap: 4px;
  }

  .save-preset-row {
    display: flex;
    gap: 6px;
  }

  .w-full {
    width: 100%;
  }

  .descriptors-catalog {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 8px;
    max-height: 400px;
    overflow-y: auto;
  }

  .descriptor-row {
    padding: 6px 8px;
    background: var(--raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
  }

  .desc-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text);
  }

  .desc-val {
    font-size: 11px;
    color: var(--text-muted);
  }

  .desc-meta {
    margin-top: 2px;
  }

  .reset-btn {
    background: transparent;
    border: none;
    color: var(--text-dim);
    font-size: 10px;
    cursor: pointer;
  }

  .reset-btn:hover {
    color: var(--text);
  }

  .mcp-log-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 8px;
  }

  .mcp-log-card {
    padding: 8px;
    background: var(--raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .mcp-log-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .log-tool {
    font-size: 11px;
    font-weight: 700;
    color: var(--text);
  }

  .log-summary {
    font-size: 11px;
    color: var(--text-muted);
  }

  .log-time {
    font-size: 9px;
    color: var(--text-dim);
  }

  .empty-state {
    padding: 24px 0;
    text-align: center;
    color: var(--text-dim);
    font-size: 12px;
  }

  .search-filter-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
</style>
