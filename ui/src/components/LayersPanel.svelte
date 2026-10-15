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
</script>

<div class="panel">
  <div class="panel-header">
    <span>Layers</span>
    <span class="mono" style="color: var(--text-muted); font-size: 11px;">
      {editorState.crosshair.layers.length} layers
    </span>
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
</style>
