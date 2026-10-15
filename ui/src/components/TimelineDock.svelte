<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';
  import type { AnimKindType, AnimTrack } from '../lib/types';

  let selectedProperty = $state<string>('global.rotation');
  let selectedKind = $state<AnimKindType>('rotate');

  function addQuickTrack() {
    const newTrack: AnimTrack = {
      id: `track-${Date.now()}`,
      target_path: selectedProperty,
      kind: selectedKind,
      duration_seconds: 2.0,
      speed: selectedKind === 'rotate' ? 180 : undefined,
      min: selectedKind === 'pulse' || selectedKind === 'breathe' ? 0.8 : undefined,
      max: selectedKind === 'pulse' || selectedKind === 'breathe' ? 1.2 : undefined,
      frequency_hz: selectedKind === 'pulse' || selectedKind === 'blink' ? 1.0 : undefined,
    };
    editorState.addTrack(newTrack);
  }
</script>

<div class="timeline-dock {editorState.timelineOpen ? 'open' : 'collapsed'}">
  <!-- Dock Header Bar -->
  <div class="timeline-bar">
    <div class="left-controls">
      <Tooltip title="Toggle Timeline" description="Expand or collapse the animation timeline dock">
        <button
          class="collapse-btn"
          onclick={() => (editorState.timelineOpen = !editorState.timelineOpen)}
          aria-label="Toggle Timeline Dock"
        >
          {editorState.timelineOpen ? '▼' : '▲'} Timeline
        </button>
      </Tooltip>

      <Tooltip title="Play / Pause" description="Preview procedural animation tracks live" shortcut="Space">
        <button
          class="btn-play {editorState.isPlaying ? 'playing' : ''}"
          onclick={() => editorState.togglePlay()}
          disabled={editorState.staticOnlyLock}
          aria-label={editorState.isPlaying ? 'Pause' : 'Play'}
        >
          {editorState.isPlaying ? '⏸' : '▶'}
        </button>
      </Tooltip>

      <span class="time-readout">
        {editorState.currentTime.toFixed(2)}s / {editorState.timelineDuration.toFixed(2)}s
      </span>

      <!-- Scrubber slider -->
      <input
        type="range"
        class="scrubber"
        min="0"
        max={editorState.timelineDuration}
        step="0.01"
        value={editorState.currentTime}
        oninput={(e) => editorState.seek(parseFloat((e.target as HTMLInputElement).value))}
        disabled={editorState.staticOnlyLock}
      />
    </div>

    <div class="right-controls">
      <Tooltip title="Loop Animation" description="Restart animation automatically at the end of the timeline">
        <button
          class="btn-icon {editorState.loopAnimation ? 'active' : ''}"
          onclick={() => (editorState.loopAnimation = !editorState.loopAnimation)}
          aria-label="Toggle Loop"
        >
          🔁
        </button>
      </Tooltip>

      <Tooltip
        title="Static-Only Lock"
        description="Global lock disabling all animation and recoil simulation (§4.7, zero CPU)"
      >
        <button
          class="lock-pill {editorState.staticOnlyLock ? 'locked' : ''}"
          onclick={() => editorState.toggleStaticOnlyLock()}
        >
          {editorState.staticOnlyLock ? '🔒 Static Only (Locked)' : '🔓 Static Lock OFF'}
        </button>
      </Tooltip>
    </div>
  </div>

  <!-- Dock Body: Track Inspector & Keyframe Sequence -->
  {#if editorState.timelineOpen}
    <div class="dock-body">
      <div class="tracks-sidebar">
        <div class="sidebar-header">
          <span class="title">Animation Tracks ({editorState.crosshair.animation?.length || 0})</span>
        </div>

        <div class="add-track-form">
          <select bind:value={selectedProperty} class="track-select">
            <option value="global.rotation">global.rotation</option>
            <option value="global.scale">global.scale</option>
            <option value="global.opacity">global.opacity</option>
            {#each editorState.crosshair.layers as layer, i}
              <option value="layers[{i}].opacity">{layer.name} · opacity</option>
              <option value="layers[{i}].transform.rotation">{layer.name} · rotation</option>
              <option value="layers[{i}].transform.scale">{layer.name} · scale</option>
            {/each}
          </select>

          <select bind:value={selectedKind} class="track-select">
            <option value="rotate">Rotate (360° continuous)</option>
            <option value="pulse">Pulse (Sinusoidal)</option>
            <option value="breathe">Breathe (Smooth ease)</option>
            <option value="blink">Blink (Square wave)</option>
            <option value="rainbow_cycle">Rainbow Cycle</option>
          </select>

          <button class="btn-add-track" onclick={addQuickTrack}>+ Add</button>
        </div>
      </div>

      <div class="tracks-list">
        {#if !editorState.crosshair.animation || editorState.crosshair.animation.length === 0}
          <div class="empty-tracks">
            No active animation tracks. Select a property and kind above to add one.
          </div>
        {:else}
          {#each editorState.crosshair.animation as track (track.id)}
            <div class="track-row">
              <div class="track-meta">
                <span class="track-prop"><code>{track.target_path}</code></span>
                <span class="track-kind-badge">{track.kind}</span>
                <span class="track-duration">{track.duration_seconds}s cycle</span>
              </div>
              <div class="track-timeline-bar">
                <div
                  class="playhead-indicator"
                  style="left: {(editorState.currentTime / editorState.timelineDuration) * 100}%"
                ></div>
              </div>
              <button
                class="remove-track-btn"
                onclick={() => editorState.removeTrack(track.id)}
                aria-label="Remove track"
              >
                ✕
              </button>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .timeline-dock {
    border-top: 1px solid var(--border, #262D38);
    background: var(--surface1, #14181F);
    display: flex;
    flex-direction: column;
    z-index: 10;
  }

  .timeline-bar {
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    background: var(--surface2, #1B212B);
  }

  .left-controls,
  .right-controls {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .collapse-btn {
    background: transparent;
    border: none;
    color: var(--text-dim, #717E93);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .collapse-btn:hover {
    color: #fff;
  }

  .btn-play {
    width: 28px;
    height: 28px;
    border-radius: 4px;
    background: var(--surface1, #14181F);
    border: 1px solid var(--border, #262D38);
    color: #36F2CB;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
  }

  .btn-play:hover:not(:disabled) {
    background: rgba(54, 242, 203, 0.15);
    border-color: #36F2CB;
  }

  .btn-play.playing {
    background: #36F2CB;
    color: #000;
  }

  .btn-play:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .time-readout {
    font-family: monospace;
    font-size: 11px;
    color: #E2E8F0;
    min-width: 90px;
  }

  .scrubber {
    width: 220px;
    accent-color: #36F2CB;
    cursor: pointer;
  }

  .btn-icon {
    background: transparent;
    border: none;
    font-size: 14px;
    cursor: pointer;
    opacity: 0.6;
    padding: 4px;
    border-radius: 4px;
  }

  .btn-icon.active {
    opacity: 1;
    background: rgba(54, 242, 203, 0.1);
  }

  .lock-pill {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border, #262D38);
    color: var(--text-dim, #717E93);
    font-size: 11px;
    padding: 4px 10px;
    border-radius: 12px;
    cursor: pointer;
    font-weight: 600;
  }

  .lock-pill.locked {
    background: rgba(255, 75, 75, 0.15);
    border-color: #FF4B4B;
    color: #FF4B4B;
  }

  .dock-body {
    height: 140px;
    display: flex;
    border-top: 1px solid var(--border, #262D38);
  }

  .tracks-sidebar {
    width: 300px;
    border-right: 1px solid var(--border, #262D38);
    padding: 10px 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    background: var(--surface1, #14181F);
  }

  .sidebar-header .title {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-dim, #717E93);
    text-transform: uppercase;
  }

  .add-track-form {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .track-select {
    background: var(--surface2, #1B212B);
    border: 1px solid var(--border, #262D38);
    color: #fff;
    font-size: 11px;
    padding: 4px 6px;
    border-radius: 4px;
  }

  .btn-add-track {
    background: var(--cyan, #36F2CB);
    color: #000;
    border: none;
    font-size: 11px;
    font-weight: 700;
    padding: 4px;
    border-radius: 4px;
    cursor: pointer;
  }

  .tracks-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .empty-tracks {
    color: var(--text-dim, #717E93);
    font-size: 12px;
    padding: 24px;
    text-align: center;
  }

  .track-row {
    display: flex;
    align-items: center;
    gap: 12px;
    background: var(--surface2, #1B212B);
    border: 1px solid var(--border, #262D38);
    border-radius: 4px;
    padding: 6px 10px;
  }

  .track-meta {
    width: 220px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .track-prop code {
    font-size: 11px;
    color: #36F2CB;
  }

  .track-kind-badge {
    background: rgba(255, 255, 255, 0.08);
    font-size: 10px;
    padding: 1px 5px;
    border-radius: 3px;
    text-transform: capitalize;
    color: #CBD5E1;
  }

  .track-duration {
    font-size: 10px;
    color: var(--text-dim, #717E93);
  }

  .track-timeline-bar {
    flex: 1;
    height: 16px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 2px;
    position: relative;
    overflow: hidden;
  }

  .playhead-indicator {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px;
    background: #FF4FD8;
    box-shadow: 0 0 6px #FF4FD8;
  }

  .remove-track-btn {
    background: transparent;
    border: none;
    color: var(--text-dim, #717E93);
    font-size: 12px;
    cursor: pointer;
  }

  .remove-track-btn:hover {
    color: #FF4B4B;
  }
</style>
