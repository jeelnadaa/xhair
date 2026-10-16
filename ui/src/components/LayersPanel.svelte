<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import type { Layer } from '../lib/types';
  import Tooltip from './Tooltip.svelte';

  let showAddMenu = $state(false);
  let editingLayerId = $state<string | null>(null);
  let editingLayerName = $state<string>('');

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

  function startRename(layer: Layer) {
    editingLayerId = layer.id;
    editingLayerName = layer.name;
  }

  function finishRename(layer: Layer) {
    if (editingLayerName.trim()) {
      layer.name = editingLayerName.trim();
      editorState.saveCurrentProfile();
    }
    editingLayerId = null;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
      if (e.key === 'Enter' && editingLayerId) {
        const l = editorState.crosshair.layers.find((x) => x.id === editingLayerId);
        if (l) finishRename(l);
      }
      return;
    }

    if (e.key === 'F2') {
      e.preventDefault();
      const first = editorState.selectedLayers[0];
      if (first) startRename(first);
    } else if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'g') {
      e.preventDefault();
      const grp = editorState.selectedLayers.find((l) => l.isGroup);
      if (grp) editorState.ungroupLayer(grp.id);
    } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'g') {
      e.preventDefault();
      editorState.groupSelectedLayers();
    } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'a') {
      e.preventDefault();
      editorState.selectAllLayers();
    } else if (e.key === 'Escape') {
      editorState.clearSelection();
    } else if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'i') {
      e.preventDefault();
      editorState.invertSelection();
    }
  }

  // Top-level layers (not nested inside a group)
  let topLevelLayers = $derived(
    editorState.crosshair.layers.filter((l) => !l.groupId)
  );
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="panel">
  <div class="panel-header">
    <div style="display: flex; align-items: center; gap: 8px;">
      <span>Layers (§4)</span>
      <span class="mono" style="color: var(--text-muted); font-size: 11px;">
        {editorState.crosshair.layers.length} layers
      </span>
    </div>
    <div class="group-header-actions">
      {#if editorState.selectedLayers.some((l) => l.isGroup)}
        <Tooltip title="Ungroup Layers" description="Dissolve group into separate layers" shortcut="Ctrl+Shift+G">
          <button
            class="btn btn-sm btn-group-action"
            onclick={() => {
              const grp = editorState.selectedLayers.find((l) => l.isGroup);
              if (grp) editorState.ungroupLayer(grp.id);
            }}
          >
            Ungroup
          </button>
        </Tooltip>
      {:else}
        <Tooltip title="Group Layers" description="Group selected layers together" shortcut="Ctrl+G">
          <button
            class="btn btn-sm btn-group-action"
            disabled={editorState.selectedLayerIds.length < 2}
            onclick={() => editorState.groupSelectedLayers()}
          >
            + Group
          </button>
        </Tooltip>
      {/if}
    </div>
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
    {#each topLevelLayers as layer (layer.id)}
      {@const isSelected = editorState.selectedLayerIds.includes(layer.id)}
      {@const issue = editorState.alignmentIssues.find((i) => i.layer_id === layer.id)}

      {#if layer.isGroup}
        <!-- Layer Group Container Row (§4) -->
        <div class="layer-group-card {isSelected ? 'active' : ''}">
          <div
            class="group-header-row"
            onclick={(e) => editorState.selectLayer(layer.id, e.ctrlKey || e.metaKey, e.shiftKey)}
            role="button"
            tabindex="0"
            onkeydown={(e) => { if (e.key === 'Enter') editorState.selectLayer(layer.id); }}
          >
            <div class="layer-main">
              <button
                class="icon-action expand-toggle"
                onclick={(e) => {
                  e.stopPropagation();
                  layer.isExpanded = !layer.isExpanded;
                }}
                aria-label="Expand group"
              >
                {layer.isExpanded !== false ? '▾' : '▸'}
              </button>
              <span class="group-folder-icon">📁</span>
              {#if editingLayerId === layer.id}
                <!-- svelte-ignore a11y_autofocus -->
                <input
                  type="text"
                  class="input-rename mono"
                  bind:value={editingLayerName}
                  onblur={() => finishRename(layer)}
                  onclick={(e) => e.stopPropagation()}
                  autofocus
                />
              {:else}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <span
                  class="layer-name"
                  ondblclick={(e) => { e.stopPropagation(); startRename(layer); }}
                >
                  {layer.name}
                </span>
              {/if}
            </div>

            <div class="layer-actions">
              <span class="badge badge-child-count">
                {layer.children?.length ?? 0}
              </span>
              <Tooltip title="Ungroup" description="Ungroup layers" shortcut="Ctrl+Shift+G">
                <button
                  class="icon-action"
                  onclick={(e) => {
                    e.stopPropagation();
                    editorState.ungroupLayer(layer.id);
                  }}
                  aria-label="Ungroup"
                >
                  ✕
                </button>
              </Tooltip>
            </div>
          </div>

          <!-- Group Children (Indented) -->
          {#if layer.isExpanded !== false && layer.children}
            <div class="group-children-list">
              {#each layer.children as childId (childId)}
                {@const child = editorState.crosshair.layers.find((l) => l.id === childId)}
                {#if child}
                  {@const isChildSelected = editorState.selectedLayerIds.includes(child.id)}
                  <div
                    class="child-layer-row {isChildSelected ? 'active' : ''}"
                    onclick={(e) => editorState.selectLayer(child.id, e.ctrlKey || e.metaKey, e.shiftKey)}
                    role="button"
                    tabindex="0"
                    onkeydown={(e) => { if (e.key === 'Enter') editorState.selectLayer(child.id); }}
                  >
                    <div class="layer-main">
                      <span class="child-indent">↳</span>
                      {#if editingLayerId === child.id}
                        <!-- svelte-ignore a11y_autofocus -->
                        <input
                          type="text"
                          class="input-rename mono"
                          bind:value={editingLayerName}
                          onblur={() => finishRename(child)}
                          onclick={(e) => e.stopPropagation()}
                          autofocus
                        />
                      {:else}
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <span
                          class="layer-name"
                          ondblclick={(e) => { e.stopPropagation(); startRename(child); }}
                        >
                          {child.name}
                        </span>
                      {/if}
                    </div>

                    <div class="layer-actions">
                      <button
                        class="icon-action {child.visible ? '' : 'dimmed'}"
                        onclick={(e) => {
                          e.stopPropagation();
                          editorState.toggleLayerVisibility(child.id);
                        }}
                      >
                        {child.visible ? '👁' : '✕'}
                      </button>
                    </div>
                  </div>
                {/if}
              {/each}
            </div>
          {/if}
        </div>
      {:else}
        <!-- Regular Single Layer Row -->
        <div
          class="layer-row {isSelected ? 'active' : ''}"
          onclick={(e) => editorState.selectLayer(layer.id, e.ctrlKey || e.metaKey, e.shiftKey)}
          role="button"
          tabindex="0"
          onkeydown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              editorState.selectLayer(layer.id);
            }
          }}
        >
          <div class="layer-main">
            <!-- Visibility Toggle -->
            <Tooltip title="Toggle Visibility" description="Hide or show this layer" shortcut="Ctrl+H">
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
            <Tooltip title="Toggle Lock" description="Lock layer from modification" shortcut="Ctrl+L">
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

            {#if editingLayerId === layer.id}
              <!-- svelte-ignore a11y_autofocus -->
              <input
                type="text"
                class="input-rename mono"
                bind:value={editingLayerName}
                onblur={() => finishRename(layer)}
                onclick={(e) => e.stopPropagation()}
                autofocus
              />
            {:else}
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <span
                class="layer-name"
                ondblclick={(e) => { e.stopPropagation(); startRename(layer); }}
              >
                {layer.name}
              </span>
            {/if}
          </div>

          <div class="layer-actions">
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
      {/if}
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

  .group-header-actions {
    display: flex;
    gap: 4px;
  }

  .btn-group-action {
    height: 22px;
    padding: 0 8px;
    font-size: 11px;
    background: var(--surface2);
    border: 1px solid var(--border);
    color: var(--text);
  }

  .btn-group-action:hover:not(:disabled) {
    background: var(--surface3);
    border-color: var(--border-active);
  }

  .layer-group-card {
    background: var(--surface2);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    overflow: hidden;
    transition: border-color 100ms ease;
  }

  .layer-group-card.active {
    border-color: var(--accent);
  }

  .group-header-row {
    padding: 7px 10px;
    background: var(--surface3);
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
  }

  .group-children-list {
    padding: 4px 6px 6px 18px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    background: var(--surface2);
  }

  .child-layer-row {
    padding: 5px 8px;
    border-radius: var(--radius-sm);
    background: var(--surface1);
    border: 1px solid transparent;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
  }

  .child-layer-row:hover {
    background: #202730;
  }

  .child-layer-row.active {
    border-color: var(--accent);
    background: var(--accent-dim);
  }

  .child-indent {
    font-size: 11px;
    color: var(--text-muted);
  }

  .group-folder-icon {
    font-size: 13px;
  }

  .badge-child-count {
    background: var(--surface1);
    color: var(--text-muted);
    font-size: 10px;
  }

  .input-rename {
    background: var(--bg);
    border: 1px solid var(--accent);
    border-radius: var(--radius-xs);
    color: var(--text);
    font-size: 12px;
    padding: 1px 4px;
    width: 130px;
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
