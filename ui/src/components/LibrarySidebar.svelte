<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';
  import type { Layer } from '../lib/types';

  let activeTab = $state<'library' | 'layers'>('library');

  // Search & Filter
  let searchQuery = $state('');
  let filterMode = $state<'all' | 'group' | 'ungrouped'>('all');

  // In-Place Group Creation (eliminating window.prompt)
  let isCreatingGroup = $state(false);
  let newGroupName = $state('');

  // In-Place Rename
  let renamingCrosshairId = $state<string | null>(null);
  let renameCrosshairText = $state('');
  let renamingLayerId = $state<string | null>(null);
  let renameLayerText = $state('');

  // Context Menu State
  let contextMenu = $state<{
    visible: boolean;
    x: number;
    y: number;
    type: 'crosshair' | 'group' | 'layer';
    targetId: string;
  }>({
    visible: false,
    x: 0,
    y: 0,
    type: 'crosshair',
    targetId: '',
  });

  // Share Code Toast
  let shareCodeCopied = $state(false);

  // Add Shape Menu
  let showAddShapeMenu = $state(false);

  const availableShapes: {
    type: import('../lib/types').ShapeType;
    label: string;
    icon: string;
    desc: string;
  }[] = [
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

  let filteredCrosshairs = $derived(
    editorState.library.filter((c) => {
      const matchesSearch =
        c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        c.tags.some((t) => t.toLowerCase().includes(searchQuery.toLowerCase()));
      if (!matchesSearch) return false;

      if (filterMode === 'ungrouped') {
        return !c.groupId;
      }
      if (filterMode === 'group' && editorState.activeGroupId) {
        return c.groupId === editorState.activeGroupId;
      }
      return true;
    })
  );

  let topLevelLayers = $derived(
    editorState.crosshair.layers.filter((l) => !l.groupId)
  );

  function startCreateGroup() {
    isCreatingGroup = true;
    newGroupName = '';
  }

  function commitCreateGroup() {
    const trimmed = newGroupName.trim();
    if (trimmed) {
      editorState.createCrosshairGroup(trimmed);
    }
    isCreatingGroup = false;
    newGroupName = '';
  }

  function openGroupSettings(groupId: string) {
    editorState.editingGroupId = groupId;
    editorState.groupSettingsModalOpen = true;
  }

  function handleContextMenu(e: MouseEvent, type: 'crosshair' | 'group' | 'layer', targetId: string) {
    e.preventDefault();
    contextMenu = {
      visible: true,
      x: Math.min(e.clientX, window.innerWidth - 180),
      y: Math.min(e.clientY, window.innerHeight - 200),
      type,
      targetId,
    };
  }

  function closeContextMenu() {
    contextMenu.visible = false;
  }

  function startRenameCrosshair(id: string) {
    const ch = editorState.library.find((c) => c.id === id);
    if (ch) {
      renamingCrosshairId = id;
      renameCrosshairText = ch.name;
    }
    closeContextMenu();
  }

  function commitRenameCrosshair(id: string) {
    if (renameCrosshairText.trim()) {
      if (editorState.crosshair.id === id) {
        editorState.renameCrosshair(renameCrosshairText.trim());
      } else {
        const item = editorState.library.find((c) => c.id === id);
        if (item) {
          item.name = renameCrosshairText.trim();
          editorState.saveCurrentProfile();
        }
      }
    }
    renamingCrosshairId = null;
  }

  function handleDuplicateCrosshair(id: string) {
    editorState.duplicateCrosshair(id);
    closeContextMenu();
  }

  function handleDeleteCrosshair(id: string) {
    editorState.deleteCrosshair(id);
    closeContextMenu();
  }

  async function handleExportShareCode(id: string) {
    closeContextMenu();
    const ch = editorState.library.find((c) => c.id === id);
    if (!ch) return;
    try {
      const code = `XHAIR-1-${btoa(JSON.stringify({ n: ch.name, l: ch.layers.length })).slice(0, 16)}`;
      if (typeof navigator !== 'undefined' && navigator.clipboard) {
        await navigator.clipboard.writeText(code);
        shareCodeCopied = true;
        setTimeout(() => (shareCodeCopied = false), 2000);
      }
    } catch (_) {}
  }

  function handleAddShape(type: import('../lib/types').ShapeType, label: string) {
    editorState.addLayer(type, label);
    showAddShapeMenu = false;
  }

  function startRenameLayer(layer: Layer) {
    renamingLayerId = layer.id;
    renameLayerText = layer.name;
    closeContextMenu();
  }

  function commitRenameLayer(layer: Layer) {
    if (renameLayerText.trim()) {
      layer.name = renameLayerText.trim();
      editorState.saveCurrentProfile();
    }
    renamingLayerId = null;
  }

  function handleGlobalClick(e: MouseEvent) {
    if (contextMenu.visible) {
      closeContextMenu();
    }
  }
</script>

<svelte:window onclick={handleGlobalClick} />

<aside class="left-rail">
  <!-- Segmented Tab Header (Library vs Layers) with Collapse Toggle -->
  <div class="rail-tabs-header">
    <button
      class="rail-tab-btn {activeTab === 'library' ? 'active' : ''}"
      onclick={() => (activeTab = 'library')}
    >
      <span class="tab-icon">📁</span>
      <span>Library</span>
      <span class="tab-badge">{editorState.library.length}</span>
    </button>
    <button
      class="rail-tab-btn {activeTab === 'layers' ? 'active' : ''}"
      onclick={() => (activeTab = 'layers')}
    >
      <span class="tab-icon">☰</span>
      <span>Layers</span>
      <span class="tab-badge">{editorState.crosshair.layers.length}</span>
    </button>
    <Tooltip title="Collapse Sidebar" shortcut="Ctrl+B">
      <button
        class="btn-rail-collapse"
        onclick={() => editorState.toggleLeftRail()}
        aria-label="Collapse Sidebar"
      >
        ◀
      </button>
    </Tooltip>
  </div>

  {#if activeTab === 'library'}
    <!-- ==================== LIBRARY VIEW ==================== -->
    <div class="rail-content">
      <!-- Search Box & Group Add -->
      <div class="search-toolbar">
        <input
          type="text"
          class="input-text search-input"
          placeholder="Search library…"
          bind:value={searchQuery}
          aria-label="Search crosshair library"
        />
        <Tooltip title="Create Group" description="Create an in-place game profile folder">
          <button class="btn btn-sm btn-icon" onclick={startCreateGroup} aria-label="Add Group">
            + Group
          </button>
        </Tooltip>
      </div>

      <!-- In-Place Group Creation Form -->
      {#if isCreatingGroup}
        <div class="inline-create-box">
            <!-- svelte-ignore a11y_autofocus -->
            <input
              type="text"
              class="input-text inline-group-input mono"
              placeholder="Group name (e.g. Valorant, CS2)"
              bind:value={newGroupName}
              onkeydown={(e) => {
                if (e.key === 'Enter') commitCreateGroup();
                if (e.key === 'Escape') isCreatingGroup = false;
              }}
              autofocus
            />
          <div class="inline-actions">
            <button class="btn btn-sm btn-primary" onclick={commitCreateGroup}>Create</button>
            <button class="btn btn-sm" onclick={() => (isCreatingGroup = false)}>Cancel</button>
          </div>
        </div>
      {/if}

      <!-- Groups Tree Section -->
      <div class="groups-tree-section">
        <div class="tree-header mono">GROUPS</div>
        <div class="groups-pills">
          <button
            class="group-pill {filterMode === 'all' ? 'active' : ''}"
            onclick={() => (filterMode = 'all')}
          >
            <span>All</span>
            <span class="count-badge">{editorState.library.length}</span>
          </button>

          {#each editorState.groups as grp (grp.id)}
            {@const groupCount = editorState.library.filter((c) => c.groupId === grp.id).length}
            {@const isActiveGroup = editorState.activeGroupId === grp.id && filterMode === 'group'}
            <div
              class="group-pill-wrap"
              oncontextmenu={(e) => handleContextMenu(e, 'group', grp.id)}
              role="presentation"
            >
              <button
                class="group-pill {isActiveGroup ? 'active' : ''}"
                onclick={() => {
                  editorState.activateGroup(grp.id);
                  filterMode = 'group';
                }}
              >
                <span class="group-color-dot" style="background-color: {grp.color || '#38BDF8'};"></span>
                <span class="group-name">{grp.name}</span>
                <span class="count-badge">{groupCount}</span>
              </button>
              <Tooltip title="Group Settings" description="Edit process binding & hotkeys">
                <button
                  class="btn-group-gear"
                  onclick={() => openGroupSettings(grp.id)}
                  aria-label="Group Settings"
                >
                  ⚙
                </button>
              </Tooltip>
            </div>
          {/each}

          <button
            class="group-pill {filterMode === 'ungrouped' ? 'active' : ''}"
            onclick={() => (filterMode = 'ungrouped')}
          >
            <span>Ungrouped</span>
            <span class="count-badge">
              {editorState.library.filter((c) => !c.groupId).length}
            </span>
          </button>
        </div>
      </div>

      <!-- Crosshairs List -->
      <div class="crosshairs-list">
        {#each filteredCrosshairs as item (item.id)}
          {@const isActive = item.id === editorState.crosshair.id}
          <div
            class="crosshair-item {isActive ? 'active' : ''}"
            onclick={() => editorState.selectCrosshair(item.id)}
            oncontextmenu={(e) => handleContextMenu(e, 'crosshair', item.id)}
            role="button"
            tabindex="0"
            onkeydown={(e) => {
              if (e.key === 'Enter') editorState.selectCrosshair(item.id);
            }}
          >
            <!-- SVG Thumbnail -->
            <div class="thumbnail">
              <svg width="44" height="44" viewBox="0 0 44 44" aria-hidden="true">
                <rect width="44" height="44" fill="#090A0D" />
                {#each item.layers as layer}
                  {#if layer.visible}
                    {#if layer.shape.type === 'LineArms'}
                      {@const len = (layer.shape.length || 6) * 1.5}
                      {@const thick = Math.max(1.5, layer.shape.thickness || 2)}
                      {@const gap = (layer.shape.gap || 4) * 1.4}
                      <rect x={22 - thick / 2} y={22 - gap - len} width={thick} height={len} fill={layer.color} />
                      <rect x={22 - thick / 2} y={22 + gap} width={thick} height={len} fill={layer.color} />
                      <rect x={22 - gap - len} y={22 - thick / 2} width={len} height={thick} fill={layer.color} />
                      <rect x={22 + gap} y={22 - thick / 2} width={len} height={thick} fill={layer.color} />
                    {:else if layer.shape.type === 'Dot'}
                      {@const d = (layer.shape.diameter || 2) * 1.8}
                      {#if layer.shape.isSquare}
                        <rect x={22 - d / 2} y={22 - d / 2} width={d} height={d} fill={layer.color} />
                      {:else}
                        <circle cx="22" cy="22" r={d / 2} fill={layer.color} />
                      {/if}
                    {:else}
                      <circle cx="22" cy="22" r="5" fill="none" stroke={layer.color} stroke-width="1.5" />
                    {/if}
                  {/if}
                {/each}
              </svg>
            </div>

            <!-- Profile Info & In-place Rename -->
            <div class="info">
              {#if renamingCrosshairId === item.id}
                <!-- svelte-ignore a11y_autofocus -->
                <input
                  type="text"
                  class="input-text inline-rename-input mono"
                  bind:value={renameCrosshairText}
                  onclick={(e) => e.stopPropagation()}
                  onkeydown={(e) => {
                    e.stopPropagation();
                    if (e.key === 'Enter') commitRenameCrosshair(item.id);
                    if (e.key === 'Escape') renamingCrosshairId = null;
                  }}
                  onblur={() => commitRenameCrosshair(item.id)}
                  autofocus
                />
              {:else}
                <div class="name">{item.name}</div>
                <div class="meta mono">
                  <span class="meta-tag">{item.tags[0] || 'Default'}</span>
                  <span class="meta-pill">{item.layers.length} {item.layers.length === 1 ? 'layer' : 'layers'}</span>
                </div>
              {/if}
            </div>

            {#if isActive}
              <span class="active-dot" title="Active in viewport"></span>
            {/if}
          </div>
        {/each}
      </div>

      <!-- New Crosshair Buttons Footer -->
      <div class="rail-footer">
        <Tooltip title="Blank Crosshair" description="Start clean with zero layers on a calibrated grid">
          <button class="btn btn-sm new-btn" onclick={() => editorState.newCrosshair('blank')}>
            + Blank Crosshair
          </button>
        </Tooltip>
        <Tooltip title="Classic Crosshair" description="Standard 4-arm competitive rifle crosshair">
          <button class="btn btn-sm btn-primary new-btn" onclick={() => editorState.newCrosshair('classic')}>
            + Classic
          </button>
        </Tooltip>
      </div>
    </div>
  {:else}
    <!-- ==================== LAYERS VIEW ==================== -->
    <div class="rail-content">
      <!-- Layers Header Actions -->
      <div class="layers-toolbar">
        <button
          class="btn btn-sm btn-add-shape"
          onclick={() => (showAddShapeMenu = !showAddShapeMenu)}
          aria-label="Add Shape"
        >
          <span>+ Add Shape</span>
          <span class="mono" style="font-size: 10px;">{showAddShapeMenu ? '▲' : '▼'}</span>
        </button>

        <div class="group-actions">
          {#if editorState.selectedLayers.some((l) => l.isGroup)}
            <Tooltip title="Ungroup Layers" description="Dissolve group into separate layers" shortcut="Ctrl+Shift+G">
              <button
                class="btn btn-sm"
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
                class="btn btn-sm"
                disabled={editorState.selectedLayerIds.length < 2}
                onclick={() => editorState.groupSelectedLayers()}
              >
                Group
              </button>
            </Tooltip>
          {/if}
        </div>
      </div>

      <!-- Add Shape Dropdown Menu -->
      {#if showAddShapeMenu}
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

      <!-- Layers Tree Hierarchy -->
      <div class="layers-tree">
        {#each topLevelLayers as layer, idx (layer.id)}
          {@const isSelected = editorState.selectedLayerIds.includes(layer.id)}
          {@const issue = editorState.alignmentIssues.find((i) => i.layer_id === layer.id)}

          <div
            class="layer-card {isSelected ? 'active' : ''} {layer.locked ? 'locked' : ''} {!layer.visible ? 'hidden-layer' : ''}"
            onclick={(e) => editorState.selectLayer(layer.id, e.ctrlKey || e.metaKey, e.shiftKey)}
            oncontextmenu={(e) => handleContextMenu(e, 'layer', layer.id)}
            role="button"
            tabindex="0"
            onkeydown={(e) => {
              if (e.key === 'Enter') editorState.selectLayer(layer.id);
            }}
          >
            <!-- Visibility Toggle -->
            <button
              class="layer-icon-btn {layer.visible ? 'active' : ''}"
              onclick={(e) => {
                e.stopPropagation();
                editorState.toggleLayerVisibility(layer.id);
              }}
              aria-label="Toggle Visibility"
            >
              {layer.visible ? '👁' : '🕶'}
            </button>

            <!-- Lock Toggle -->
            <button
              class="layer-icon-btn {layer.locked ? 'locked' : ''}"
              onclick={(e) => {
                e.stopPropagation();
                editorState.toggleLayerLock(layer.id);
              }}
              aria-label="Toggle Lock"
            >
              {layer.locked ? '🔒' : '🔓'}
            </button>

            <!-- Color Swatch Indicator -->
            <span class="layer-swatch" style="background-color: {layer.color || '#38BDF8'};"></span>

            <!-- Layer Title / Inline Rename -->
            <div class="layer-title-area">
              {#if renamingLayerId === layer.id}
                <!-- svelte-ignore a11y_autofocus -->
                <input
                  type="text"
                  class="input-text inline-rename-input mono"
                  bind:value={renameLayerText}
                  onclick={(e) => e.stopPropagation()}
                  onkeydown={(e) => {
                    e.stopPropagation();
                    if (e.key === 'Enter') commitRenameLayer(layer);
                    if (e.key === 'Escape') renamingLayerId = null;
                  }}
                  onblur={() => commitRenameLayer(layer)}
                  autofocus
                />
              {:else}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <span class="layer-name" oncontextmenu={(e) => handleContextMenu(e, 'layer', layer.id)}>
                  {layer.name}
                </span>
                <span class="layer-kind mono">{layer.shape.type}</span>
              {/if}
            </div>

            <!-- Alignment Badge -->
            {#if issue}
              <Tooltip title="Sub-Pixel Offset" description="Layer is off-grid ({issue.severity}). Click to fix automatically.">
                <button
                  class="badge-fix"
                  onclick={(e) => {
                    e.stopPropagation();
                    editorState.fixAlignment(layer.id);
                  }}
                >
                  Fix
                </button>
              </Tooltip>
            {/if}

            <!-- Delete Layer Button -->
            <button
              class="layer-icon-btn delete-btn"
              disabled={editorState.crosshair.layers.length <= 1}
              onclick={(e) => {
                e.stopPropagation();
                editorState.deleteLayer(layer.id);
              }}
              aria-label="Delete Layer"
            >
              ✕
            </button>
          </div>
        {/each}
      </div>

      <!-- Layers Summary Footer -->
      <div class="rail-footer">
        <span class="mono" style="font-size: 11px; color: var(--text-muted);">
          {editorState.crosshair.layers.length} layers · {editorState.alignmentIssues.length === 0 ? '✓ All Aligned' : `${editorState.alignmentIssues.length} issues`}
        </span>
      </div>
    </div>
  {/if}

  <!-- Desktop Context Menu Popover -->
  {#if contextMenu.visible}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="context-menu"
      style="top: {contextMenu.y}px; left: {contextMenu.x}px;"
      onclick={(e) => e.stopPropagation()}
      role="menu"
      tabindex="-1"
    >
      {#if contextMenu.type === 'crosshair'}
        <button
          class="context-menu-item"
          onclick={() => {
            editorState.selectCrosshair(contextMenu.targetId);
            closeContextMenu();
          }}
        >
          Select Crosshair
        </button>
        <button
          class="context-menu-item"
          onclick={() => startRenameCrosshair(contextMenu.targetId)}
        >
          Rename Profile
        </button>
        <button
          class="context-menu-item"
          onclick={() => handleDuplicateCrosshair(contextMenu.targetId)}
        >
          Duplicate Crosshair
        </button>
        <button
          class="context-menu-item"
          onclick={() => handleExportShareCode(contextMenu.targetId)}
        >
          Copy Share Code
        </button>
        <div class="context-divider"></div>
        <button
          class="context-menu-item danger"
          onclick={() => handleDeleteCrosshair(contextMenu.targetId)}
        >
          Delete Crosshair
        </button>
      {:else if contextMenu.type === 'layer'}
        <button
          class="context-menu-item"
          onclick={() => {
            const l = editorState.crosshair.layers.find((x) => x.id === contextMenu.targetId);
            if (l) startRenameLayer(l);
          }}
        >
          Rename Layer
        </button>
        <button
          class="context-menu-item"
          onclick={() => {
            editorState.duplicateLayer(contextMenu.targetId);
            closeContextMenu();
          }}
        >
          Duplicate Layer
        </button>
        <button
          class="context-menu-item"
          onclick={() => {
            editorState.toggleLayerLock(contextMenu.targetId);
            closeContextMenu();
          }}
        >
          Toggle Lock
        </button>
        <button
          class="context-menu-item"
          onclick={() => {
            editorState.toggleLayerVisibility(contextMenu.targetId);
            closeContextMenu();
          }}
        >
          Toggle Visibility
        </button>
        <div class="context-divider"></div>
        <button
          class="context-menu-item danger"
          onclick={() => {
            editorState.deleteLayer(contextMenu.targetId);
            closeContextMenu();
          }}
        >
          Delete Layer
        </button>
      {:else if contextMenu.type === 'group'}
        <button
          class="context-menu-item"
          onclick={() => {
            openGroupSettings(contextMenu.targetId);
            closeContextMenu();
          }}
        >
          Group Settings & Process
        </button>
        <button
          class="context-menu-item danger"
          onclick={() => {
            editorState.deleteCrosshairGroup(contextMenu.targetId);
            closeContextMenu();
          }}
        >
          Delete Group
        </button>
      {/if}
    </div>
  {/if}

  <!-- Share Code Notification -->
  {#if shareCodeCopied}
    <div class="copy-toast mono">Share code copied to clipboard!</div>
  {/if}
</aside>

<style>
  .left-rail {
    width: 100%;
    height: 100%;
    flex: 1;
    background: var(--surface1, #0f1013);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    user-select: none;
  }

  .rail-tabs-header {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border, #262930);
    background: var(--surface2, #16181d);
  }

  .btn-rail-collapse {
    width: 36px;
    height: 36px;
    background: transparent;
    border: none;
    border-left: 1px solid var(--border, #262930);
    color: var(--text-muted, #8b949e);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    transition: all 120ms ease;
    flex-shrink: 0;
  }

  .btn-rail-collapse:hover {
    color: var(--text, #f0f6fc);
    background: var(--surface3, #1e2128);
  }

  .rail-tab-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    height: 36px;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-muted, #8b949e);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 120ms ease;
  }

  .rail-tab-btn:hover {
    color: var(--text, #f0f6fc);
    background: rgba(255, 255, 255, 0.02);
  }

  .rail-tab-btn.active {
    color: var(--text, #f0f6fc);
    border-bottom-color: var(--accent, #38bdf8);
    background: var(--surface1, #0f1013);
  }

  .tab-badge {
    font-size: 10px;
    background: var(--surface3, #1e2128);
    color: var(--text-muted, #8b949e);
    padding: 1px 5px;
    border-radius: 99px;
  }

  .rail-tab-btn.active .tab-badge {
    background: rgba(56, 189, 248, 0.15);
    color: var(--accent, #38bdf8);
  }

  .rail-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    padding: 12px;
    gap: 10px;
  }

  .search-toolbar,
  .layers-toolbar {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .search-input {
    flex: 1;
    height: 28px;
    font-size: 12px;
  }

  .inline-create-box {
    background: var(--surface2, #16181d);
    border: 1px solid var(--accent, #38bdf8);
    border-radius: var(--radius-sm, 6px);
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .inline-group-input,
  .inline-rename-input {
    height: 26px;
    font-size: 12px;
    padding: 0 6px;
    background: var(--surface3, #1e2128);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-xs, 4px);
    color: var(--text, #f0f6fc);
  }

  .inline-actions {
    display: flex;
    gap: 6px;
    justify-content: flex-end;
  }

  .groups-tree-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .tree-header {
    font-size: 10px;
    color: var(--text-muted, #6e7681);
    letter-spacing: 0.05em;
  }

  .groups-pills {
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-height: 120px;
    overflow-y: auto;
  }

  .group-pill-wrap {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .group-pill {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    background: var(--surface2, #16181d);
    border: 1px solid transparent;
    border-radius: var(--radius-sm, 6px);
    color: var(--text, #f0f6fc);
    font-size: 12px;
    cursor: pointer;
    text-align: left;
    transition: background 100ms ease;
  }

  .group-pill:hover {
    background: var(--surface3, #1e2128);
  }

  .group-pill.active {
    background: var(--surface3, #1e2128);
    border-color: var(--accent, #38bdf8);
  }

  .group-color-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .group-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .count-badge {
    font-size: 10px;
    color: var(--text-muted, #8b949e);
    background: var(--surface1, #0f1013);
    padding: 1px 5px;
    border-radius: 99px;
  }

  .btn-group-gear {
    background: transparent;
    border: none;
    color: var(--text-muted, #8b949e);
    cursor: pointer;
    font-size: 12px;
    padding: 4px;
    border-radius: var(--radius-xs, 4px);
  }

  .btn-group-gear:hover {
    color: var(--text, #f0f6fc);
    background: var(--surface3, #1e2128);
  }

  .crosshairs-list,
  .layers-tree {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-right: 2px;
  }

  .crosshair-item {
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-md, 8px);
    padding: 10px 12px;
    display: flex;
    align-items: center;
    gap: 12px;
    cursor: pointer;
    color: var(--text, #f0f6fc);
    position: relative;
    transition: all 120ms ease;
    min-height: 58px;
  }

  .crosshair-item:hover {
    background: var(--surface3, #1e2128);
    border-color: var(--border-strong, #3a3a3a);
  }

  .crosshair-item.active {
    border-color: var(--accent, #38bdf8);
    background: var(--surface3, #1e2128);
    box-shadow: 0 0 0 1px rgba(56, 189, 248, 0.2);
  }

  .thumbnail {
    width: 44px;
    height: 44px;
    border-radius: var(--radius-sm, 6px);
    overflow: hidden;
    border: 1px solid var(--border, #262930);
    background: #090a0d;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 4px;
    overflow: hidden;
    flex: 1;
    min-width: 0;
  }

  .name {
    font-size: 13px;
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--text, #f0f6fc);
  }

  .meta {
    font-size: 11px;
    color: var(--text-muted, #8b949e);
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
  }

  .meta-tag {
    color: var(--accent, #38bdf8);
    font-weight: 500;
    font-size: 10px;
  }

  .meta-pill {
    background: var(--surface1, #0f1013);
    border: 1px solid var(--border, #262930);
    padding: 1px 6px;
    border-radius: 4px;
    font-size: 10px;
    color: var(--text-muted, #8b949e);
  }

  .active-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent, #38bdf8);
    box-shadow: 0 0 6px var(--accent, #38bdf8);
    flex-shrink: 0;
  }

  .rail-footer {
    display: flex;
    gap: 6px;
    padding-top: 6px;
    border-top: 1px solid var(--border, #262930);
  }

  .new-btn {
    flex: 1;
    height: 28px;
    font-size: 11px;
  }

  /* Layers View Specific Styles */
  .btn-add-shape {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 28px;
    font-size: 11px;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    color: var(--text, #f0f6fc);
  }

  .btn-add-shape:hover {
    border-color: var(--accent, #38bdf8);
  }

  .add-shape-menu {
    position: absolute;
    top: 80px;
    left: 12px;
    right: 12px;
    max-height: 260px;
    overflow-y: auto;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-md, 8px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
    z-index: 100;
    display: flex;
    flex-direction: column;
    padding: 4px;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    border: none;
    background: transparent;
    color: var(--text, #f0f6fc);
    border-radius: var(--radius-xs, 4px);
    cursor: pointer;
    text-align: left;
  }

  .menu-item:hover {
    background: var(--surface3, #1e2128);
    color: var(--accent, #38bdf8);
  }

  .item-icon {
    font-size: 14px;
    width: 20px;
    text-align: center;
    flex-shrink: 0;
  }

  .item-info {
    display: flex;
    flex-direction: column;
  }

  .item-title {
    font-size: 11px;
    font-weight: 600;
  }

  .item-desc {
    font-size: 9px;
    color: var(--text-muted, #8b949e);
  }

  .layer-card {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    background: var(--surface2, #16181d);
    border: 1px solid transparent;
    border-radius: var(--radius-sm, 6px);
    color: var(--text, #f0f6fc);
    cursor: pointer;
    transition: all 100ms ease;
    min-height: 38px;
  }

  .layer-card:hover {
    background: var(--surface3, #1e2128);
  }

  .layer-card.active {
    border-color: var(--accent, #38bdf8);
    background: var(--surface3, #1e2128);
  }

  .layer-card.locked {
    opacity: 0.75;
  }

  .layer-card.hidden-layer {
    opacity: 0.45;
  }

  .layer-icon-btn {
    background: transparent;
    border: none;
    color: var(--text-muted, #8b949e);
    cursor: pointer;
    font-size: 13px;
    padding: 4px;
    border-radius: var(--radius-xs, 4px);
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .layer-icon-btn:hover {
    color: var(--text, #f0f6fc);
    background: var(--surface3, #1e2128);
  }

  .layer-icon-btn.active {
    color: var(--accent, #38bdf8);
  }

  .layer-icon-btn.locked {
    color: #eab308;
  }

  .layer-swatch {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
    box-shadow: 0 0 4px rgba(0, 0, 0, 0.4);
  }

  .layer-title-area {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
    min-width: 0;
    gap: 1px;
  }

  .layer-name {
    font-size: 12px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .layer-kind {
    font-size: 10px;
    color: var(--text-muted, #8b949e);
  }

  .badge-fix {
    font-size: 9px;
    padding: 2px 6px;
    background: rgba(234, 179, 8, 0.2);
    border: 1px solid #eab308;
    color: #eab308;
    border-radius: var(--radius-xs, 4px);
    cursor: pointer;
  }

  .delete-btn:hover {
    color: #ef4444;
  }

  /* Desktop Context Menu */
  .context-menu {
    position: fixed;
    z-index: 1000;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-md, 8px);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.6);
    padding: 4px;
    min-width: 170px;
    display: flex;
    flex-direction: column;
  }

  .context-menu-item {
    padding: 6px 10px;
    font-size: 12px;
    background: transparent;
    border: none;
    color: var(--text, #f0f6fc);
    text-align: left;
    cursor: pointer;
    border-radius: var(--radius-xs, 4px);
    transition: background 100ms ease;
  }

  .context-menu-item:hover {
    background: var(--surface3, #1e2128);
    color: var(--accent, #38bdf8);
  }

  .context-menu-item.danger {
    color: #f87171;
  }

  .context-menu-item.danger:hover {
    background: rgba(239, 68, 68, 0.15);
  }

  .context-divider {
    height: 1px;
    background: var(--border, #262930);
    margin: 4px 0;
  }

  .copy-toast {
    position: absolute;
    bottom: 50px;
    left: 50%;
    transform: translateX(-50%);
    background: var(--surface3, #1e2128);
    border: 1px solid var(--accent, #38bdf8);
    color: var(--accent, #38bdf8);
    font-size: 10px;
    padding: 4px 10px;
    border-radius: var(--radius-sm, 6px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    pointer-events: none;
    z-index: 200;
  }
</style>
