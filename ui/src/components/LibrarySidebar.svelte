<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  let searchQuery = $state('');
  let filterMode = $state<'all' | 'group' | 'ungrouped'>('all');

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

  function promptNewGroup() {
    const name = prompt('Enter new crosshair group name (e.g. Valorant, Apex, CS2):');
    if (name && name.trim()) {
      editorState.createCrosshairGroup(name.trim());
    }
  }

  function openGroupSettings(groupId: string) {
    editorState.editingGroupId = groupId;
    editorState.groupSettingsModalOpen = true;
  }
</script>

<aside class="library-sidebar">
  <div class="panel-header">
    <div style="display: flex; align-items: center; gap: 6px;">
      <span>Library (§8)</span>
      <span class="mono" style="font-size: 10px; color: var(--text-muted);">
        {editorState.library.length} items
      </span>
    </div>
    <Tooltip title="Create Group" description="Create a new game crosshair group">
      <button class="btn btn-sm btn-icon" onclick={promptNewGroup}>
        + Group
      </button>
    </Tooltip>
  </div>

  <!-- Search Box -->
  <div class="search-box">
    <input
      type="text"
      class="input-text"
      placeholder="Search library…"
      bind:value={searchQuery}
      aria-label="Search crosshair library"
    />
  </div>

  <!-- Groups Tree (§8) -->
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
        <div class="group-pill-wrap">
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
      <button
        class="crosshair-item {item.id === editorState.crosshair.id ? 'active' : ''}"
        onclick={() => editorState.selectCrosshair(item.id)}
      >
        <div class="thumbnail">
          <svg width="40" height="40" viewBox="0 0 40 40" aria-hidden="true">
            <rect width="40" height="40" fill="#090A0D" />
            {#each item.layers as layer}
              {#if layer.visible}
                {#if layer.shape.type === 'LineArms'}
                  {@const len = (layer.shape.length || 6) * 1.5}
                  {@const thick = Math.max(1.5, (layer.shape.thickness || 2))}
                  {@const gap = (layer.shape.gap || 4) * 1.5}
                  <!-- Top Arm -->
                  <rect
                    x={20 - thick / 2}
                    y={20 - gap - len}
                    width={thick}
                    height={len}
                    fill={layer.color}
                  />
                  <!-- Bottom Arm -->
                  <rect
                    x={20 - thick / 2}
                    y={20 + gap}
                    width={thick}
                    height={len}
                    fill={layer.color}
                  />
                  <!-- Left Arm -->
                  <rect
                    x={20 - gap - len}
                    y={20 - thick / 2}
                    width={len}
                    height={thick}
                    fill={layer.color}
                  />
                  <!-- Right Arm -->
                  <rect
                    x={20 + gap}
                    y={20 - thick / 2}
                    width={len}
                    height={thick}
                    fill={layer.color}
                  />
                {:else if layer.shape.type === 'Dot'}
                  {@const d = (layer.shape.diameter || 2) * 1.5}
                  {#if layer.shape.isSquare}
                    <rect x={20 - d / 2} y={20 - d / 2} width={d} height={d} fill={layer.color} />
                  {:else}
                    <circle cx="20" cy="20" r={d / 2} fill={layer.color} />
                  {/if}
                {/if}
              {/if}
            {/each}
          </svg>
        </div>

        <div class="info">
          <div class="name">{item.name}</div>
          <div class="meta mono">
            {item.tags[0] || 'Default'} · {item.layers.length} layers
          </div>
        </div>
      </button>
    {/each}
  </div>

  <!-- Blank-start new crosshair (§2) -->
  <div class="new-buttons-group">
    <Tooltip
      title="Create Blank Crosshair (§2)"
      description="Start with zero layers on an empty grid showing only center marker"
    >
      <button class="btn btn-primary new-btn" onclick={() => editorState.newCrosshair('blank')}>
        + Blank crosshair (§2)
      </button>
    </Tooltip>
  </div>
</aside>

<style>
  .library-sidebar {
    width: 250px;
    flex: none;
    border-right: 1px solid var(--border);
    padding: 16px;
    background: var(--surface1);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    gap: 10px;
  }

  .search-box input {
    width: 100%;
    height: 30px;
    font-size: 12px;
  }

  .groups-tree-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .tree-header {
    font-size: 10px;
    color: var(--text-muted);
    letter-spacing: 0.06em;
  }

  .groups-pills {
    display: flex;
    flex-direction: column;
    gap: 3px;
    max-height: 140px;
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
    background: var(--surface2);
    border: 1px solid transparent;
    border-radius: var(--radius-sm);
    color: var(--text);
    font-size: 12px;
    cursor: pointer;
    text-align: left;
    transition: background 100ms ease;
  }

  .group-pill:hover {
    background: var(--surface3);
  }

  .group-pill.active {
    background: var(--surface3);
    border-color: var(--accent);
  }

  .group-color-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    flex: none;
  }

  .group-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .count-badge {
    font-size: 10px;
    color: var(--text-muted);
    background: var(--surface1);
    padding: 1px 5px;
    border-radius: 99px;
  }

  .btn-group-gear {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 12px;
    padding: 4px;
    border-radius: var(--radius-xs);
  }

  .btn-group-gear:hover {
    color: var(--text);
    background: var(--surface3);
  }

  .crosshairs-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding-right: 2px;
  }

  .crosshair-item {
    background: var(--surface2);
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    padding: 8px;
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    text-align: left;
    color: var(--text);
    transition: border-color 100ms ease, background 100ms ease;
  }

  .crosshair-item:hover {
    background: var(--surface3);
  }

  .crosshair-item.active {
    border-color: var(--accent);
    background: var(--surface3);
  }

  .thumbnail {
    width: 40px;
    height: 40px;
    border-radius: var(--radius-sm);
    overflow: hidden;
    border: 1px solid var(--border);
    flex: none;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow: hidden;
  }

  .name {
    font-size: 13px;
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meta {
    font-size: 11px;
    color: var(--text-muted);
  }

  .new-buttons-group {
    padding-top: 6px;
  }

  .new-btn {
    width: 100%;
    height: 32px;
  }
</style>
