<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  function isLayerAligned(layerId: string): boolean {
    const layer = editorState.crosshair.layers.find((l) => l.id === layerId);
    if (!layer) return true;
    // Check if layer offsets or arm geometry align to whole game pixels
    const wholeOffset = Number.isInteger(layer.offsetX) && Number.isInteger(layer.offsetY);
    if (layer.shape.type === 'LineArms') {
      const wholeLength = Number.isInteger(layer.shape.length ?? 0);
      const wholeThickness = Number.isInteger(layer.shape.thickness ?? 0);
      return wholeOffset && wholeLength && wholeThickness;
    }
    if (layer.shape.type === 'Dot') {
      const wholeDiameter = Number.isInteger(layer.shape.diameter ?? 0);
      return wholeOffset && wholeDiameter;
    }
    return wholeOffset;
  }
  let showAddMenu = $state(false);

  const availableShapes: { type: import('../lib/types').ShapeType; label: string; icon: string; desc: string }[] = [
    { type: 'LineArms', label: 'Line Arms', icon: '┼', desc: 'Standard 4-arm competitive crosshair' },
    { type: 'Plus', label: 'Plus Cross', icon: '➕', desc: '4-direction crosshair with center hole support' },
    { type: 'TShape', label: 'T-Shape Reticle', icon: '┬', desc: 'Tactical 3-arm reticle without top arm' },
    { type: 'Dot', label: 'Center Dot', icon: '•', desc: 'Precision center dot (square or circle)' },
    { type: 'Rect', label: 'Rectangle / Box', icon: '▭', desc: 'Filled rectangle or frame box element' },
    { type: 'Circle', label: 'Circle / Ring', icon: '○', desc: 'Circumscribed target ring or outline' },
    { type: 'Arc', label: 'Arc Segment', icon: '⌒', desc: 'Partial angular ring segment' },
    { type: 'Triangle', label: 'Triangle', icon: '▲', desc: 'Isosceles or equilateral aiming triangle' },
    { type: 'Chevron', label: 'Chevron Arrow', icon: '^', desc: 'ACOG / Holo inverted V aiming arrow' },
    { type: 'Polygon', label: 'Polygon / Hexagon', icon: '⬡', desc: 'Regular geometric N-gon reticle' },
    { type: 'Star', label: 'Star Reticle', icon: '★', desc: 'Multi-point star crosshair element' },
    { type: 'SvgPath', label: 'Custom SVG Vector', icon: '✒', desc: 'Arbitrary vector path geometry' },
  ];

  function handleAddShape(type: import('../lib/types').ShapeType, label: string) {
    editorState.addLayer(type, label);
    showAddMenu = false;
  }
</script>

<div class="panel">
  <div class="panel-header">
    <span>Layers</span>
    <span class="mono" style="color: var(--text-muted); font-size: 11px;">
      {editorState.crosshair.layers.length} layers
    </span>
  </div>

  <!-- Add Custom Shape Bar -->
  <div class="add-shape-section">
    <button
      class="btn btn-add-shape"
      onclick={() => (showAddMenu = !showAddMenu)}
      aria-label="Add custom shape"
    >
      <span>+ Add Shape</span>
      <span class="mono" style="font-size: 10px;">{showAddMenu ? '▲' : '▼'}</span>
    </button>

    {#if showAddMenu}
      <div class="add-shape-menu">
        {#each availableShapes as shape}
          <button
            class="menu-item"
            onclick={() => handleAddShape(shape.type, shape.label)}
          >
            <span class="item-icon">{shape.icon}</span>
            <div class="item-info">
              <span class="item-title">{shape.label}</span>
              <span class="item-desc">{shape.desc}</span>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <div class="layers-list">
    {#each editorState.crosshair.layers as layer, index (layer.id)}
      {@const issue = editorState.alignmentIssues.find((i) => i.layer_id === layer.id)}
      <div
        class="layer-row {layer.id === editorState.selectedLayerId ? 'active' : ''}"
        onclick={() => (editorState.selectedLayerId = layer.id)}
        role="button"
        tabindex="0"
        onkeydown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            editorState.selectedLayerId = layer.id;
          }
        }}
      >
        <div class="layer-main">
          <!-- Visibility Toggle -->
          <Tooltip title="Toggle Visibility" description="Hide or show this layer on the canvas">
            <button
              class="icon-action {layer.visible ? '' : 'dimmed'}"
              onclick={(e) => {
                e.stopPropagation();
                editorState.toggleLayerVisibility(layer.id);
              }}
              aria-label="Toggle layer visibility"
            >
              {layer.visible ? '👁' : '✕'}
            </button>
          </Tooltip>

          <!-- Lock Toggle -->
          <Tooltip title="Toggle Lock" description="Prevent accidental mutation or dragging">
            <button
              class="icon-action {layer.locked ? 'locked' : 'dimmed'}"
              onclick={(e) => {
                e.stopPropagation();
                editorState.toggleLayerLock(layer.id);
              }}
              aria-label="Toggle layer lock"
            >
              {layer.locked ? '🔒' : '🔓'}
            </button>
          </Tooltip>

          <span class="layer-name">{layer.name}</span>
        </div>

        <div class="layer-actions">
          <!-- Alignment Badge / 1-Click Fix -->
          {#if issue}
            <Tooltip
              title="Off-Grid Edge Detected"
              description="Click to automatically apply 1-click fix to {issue.suggested_fix.new_thickness}px"
            >
              <button
                class="badge badge-warning"
                onclick={(e) => {
                  e.stopPropagation();
                  editorState.apply1ClickAlignmentFix(issue);
                }}
              >
                ⚠ FIX ({issue.suggested_fix.new_thickness}px)
              </button>
            </Tooltip>
          {:else}
            <span class="badge">ALIGNED</span>
          {/if}

          <!-- Duplicate -->
          <Tooltip title="Duplicate Layer" description="Clone this layer" shortcut="Ctrl+D">
            <button
              class="icon-action"
              onclick={(e) => {
                e.stopPropagation();
                editorState.duplicateLayer(layer.id);
              }}
              aria-label="Duplicate layer"
            >
              ⧉
            </button>
          </Tooltip>

          <!-- Delete -->
          <Tooltip title="Delete Layer" description="Delete this layer" shortcut="Del">
            <button
              class="icon-action delete"
              disabled={editorState.crosshair.layers.length <= 1}
              onclick={(e) => {
                e.stopPropagation();
                editorState.deleteLayer(layer.id);
              }}
              aria-label="Delete layer"
            >
              🗑
            </button>
          </Tooltip>
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .layers-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .layer-row {
    background: var(--surface2);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 8px 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    transition: border-color 100ms ease, background 100ms ease;
  }

  .layer-row:hover {
    background: #202730;
  }

  .layer-row.active {
    border-color: var(--accent);
    background: var(--accent-dim);
  }

  .layer-main {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
  }

  .layer-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .icon-action {
    background: transparent;
    border: none;
    color: var(--text);
    font-size: 12px;
    cursor: pointer;
    padding: 2px 4px;
    border-radius: var(--radius-sm);
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .icon-action:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.1);
  }

  .icon-action.dimmed {
    opacity: 0.35;
  }

  .icon-action.locked {
    color: var(--warning);
  }

  .icon-action.delete:hover:not(:disabled) {
    color: var(--danger);
  }

  .icon-action:disabled {
    opacity: 0.2;
    cursor: not-allowed;
  }

  .layer-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .add-shape-section {
    position: relative;
    margin-bottom: 10px;
  }

  .btn-add-shape {
    width: 100%;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    background: rgba(54, 242, 203, 0.08);
    border: 1px dashed var(--accent);
    color: var(--accent);
    font-weight: 600;
    font-size: 12px;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: background 120ms ease, border-color 120ms ease;
  }

  .btn-add-shape:hover {
    background: rgba(54, 242, 203, 0.16);
    border-style: solid;
  }

  .add-shape-menu {
    position: absolute;
    top: 36px;
    left: 0;
    right: 0;
    background: #151A21;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    z-index: 50;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
    max-height: 280px;
    overflow-y: auto;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 8px;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    cursor: pointer;
    text-align: left;
    width: 100%;
    color: var(--text);
    transition: background 100ms ease;
  }

  .menu-item:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .item-icon {
    font-size: 16px;
    color: var(--accent);
    width: 20px;
    text-align: center;
  }

  .item-info {
    display: flex;
    flex-direction: column;
  }

  .item-title {
    font-size: 12px;
    font-weight: 600;
  }

  .item-desc {
    font-size: 10px;
    color: var(--text-muted);
  }
</style>
