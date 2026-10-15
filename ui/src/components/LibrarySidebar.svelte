<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  let searchQuery = $state('');

  let filteredCrosshairs = $derived(
    editorState.library.filter(
      (c) =>
        c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        c.tags.some((t) => t.toLowerCase().includes(searchQuery.toLowerCase()))
    )
  );
</script>

<aside class="library-sidebar">
  <div class="panel-header">
    <span>Library</span>
    <span class="mono" style="font-size: 10px; color: var(--text-muted);">
      {editorState.library.length} items
    </span>
  </div>

  <div class="search-box">
    <input
      type="text"
      class="input-text"
      placeholder="Search library…"
      bind:value={searchQuery}
      aria-label="Search crosshair library"
    />
  </div>

  <div class="crosshairs-list">
    {#each filteredCrosshairs as item (item.id)}
      <button
        class="crosshair-item {item.id === editorState.crosshair.id ? 'active' : ''}"
        onclick={() => editorState.selectCrosshair(item.id)}
      >
        <div class="thumbnail">
          <svg width="40" height="40" viewBox="0 0 40 40" aria-hidden="true">
            <rect width="40" height="40" fill="#0B0D10" />
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
          <div class="meta">
            {item.tags[0] || 'Default'} · {item.layers.length} layers
          </div>
        </div>
      </button>
    {/each}
  </div>

  <Tooltip
    title="Create New Crosshair"
    description="Add a new blank crosshair preset to your local library"
  >
    <button class="btn btn-primary new-btn" onclick={() => editorState.newCrosshair()}>
      + New crosshair
    </button>
  </Tooltip>
</aside>

<style>
  .library-sidebar {
    width: 240px;
    flex: none;
    border-right: 1px solid var(--border);
    padding: 16px;
    background: var(--surface1);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .search-box {
    margin-bottom: 12px;
  }

  .search-box input {
    width: 100%;
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
    background: #202730;
  }

  .crosshair-item.active {
    border-color: var(--accent);
    background: var(--accent-dim);
  }

  .thumbnail {
    width: 40px;
    height: 40px;
    flex: none;
    border-radius: var(--radius-sm);
    overflow: hidden;
    border: 1px solid var(--border);
  }

  .info {
    flex: 1;
    min-width: 0;
  }

  .name {
    font-weight: 600;
    font-size: 13px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meta {
    font-size: 11px;
    color: var(--text-muted);
    margin-top: 2px;
  }

  .new-btn {
    width: 100%;
    margin-top: 12px;
  }
</style>
