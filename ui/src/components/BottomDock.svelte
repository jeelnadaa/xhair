<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';
  import type { AnimKindType, AnimTrack } from '../lib/types';

  type DockTab = 'timeline' | 'spray_lab' | 'calibration' | 'mcp_activity';
  let activeTab = $state<DockTab>('timeline');
  let isExpanded = $state(false);

  // Timeline track inputs
  let selectedProperty = $state<string>('global.rotation');
  let selectedKind = $state<AnimKindType>('rotate');

  function toggleTab(tab: DockTab) {
    if (activeTab === tab) {
      isExpanded = !isExpanded;
    } else {
      activeTab = tab;
      isExpanded = true;
    }
  }

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

  function handleScrubberInput(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    editorState.currentTime = val;
    editorState.syncWithOverlay();
  }
</script>

<div class="bottom-dock {isExpanded ? 'is-expanded' : 'is-collapsed'}">
  <!-- Dock Header Bar (Tabs + Controls) -->
  <div class="dock-header-bar">
    <div class="tabs-group">
      <!-- 1. Timeline Tab -->
      <button
        class="dock-tab-btn {activeTab === 'timeline' && isExpanded ? 'active' : ''}"
        onclick={() => toggleTab('timeline')}
      >
        <span class="tab-icon">⏱</span>
        <span>Timeline</span>
        {#if editorState.isPlaying}
          <span class="play-pulse-dot" title="Playing"></span>
        {/if}
      </button>

      <!-- 2. Spray Lab Tab -->
      <button
        class="dock-tab-btn {activeTab === 'spray_lab' && isExpanded ? 'active' : ''}"
        onclick={() => toggleTab('spray_lab')}
      >
        <span class="tab-icon">🎯</span>
        <span>Spray Lab</span>
      </button>

      <!-- 3. Calibration Tab -->
      <button
        class="dock-tab-btn {activeTab === 'calibration' && isExpanded ? 'active' : ''}"
        onclick={() => toggleTab('calibration')}
      >
        <span class="tab-icon">📐</span>
        <span>Calibration</span>
        {#if editorState.calibrationPatternActive}
          <span class="calib-active-dot" title="Pattern active"></span>
        {/if}
      </button>

      <!-- 4. MCP Agent Log Tab -->
      <button
        class="dock-tab-btn {activeTab === 'mcp_activity' && isExpanded ? 'active' : ''}"
        onclick={() => toggleTab('mcp_activity')}
      >
        <span class="tab-icon">🤖</span>
        <span>MCP Activity</span>
        {#if editorState.mcpActivityLog.length > 0}
          <span class="log-count-badge">{editorState.mcpActivityLog.length}</span>
        {/if}
        {#if editorState.isAgentEditing}
          <span class="agent-pulse-dot" title="Agent is editing"></span>
        {/if}
      </button>
    </div>

    <!-- Quick Status / Expand Controls on the right -->
    <div class="dock-right-controls">
      {#if activeTab === 'timeline'}
        <div class="timeline-quick-controls">
          <button
            class="btn-play-mini {editorState.isPlaying ? 'playing' : ''}"
            onclick={() => editorState.togglePlay()}
            disabled={editorState.staticOnlyLock}
            title={editorState.isPlaying ? 'Pause' : 'Play (Space)'}
          >
            {editorState.isPlaying ? '⏸' : '▶'}
          </button>
          <span class="time-readout mono">
            {editorState.currentTime.toFixed(2)}s / {editorState.timelineDuration.toFixed(2)}s
          </span>
        </div>
      {/if}

      <button
        class="dock-toggle-btn"
        onclick={() => (isExpanded = !isExpanded)}
        title={isExpanded ? 'Collapse Bottom Dock' : 'Expand Bottom Dock'}
      >
        <span class="mono" style="font-size: 11px;">{isExpanded ? '▼ Collapse' : '▲ Expand'}</span>
      </button>
    </div>
  </div>

  <!-- Dock Body Content (Visible when expanded) -->
  {#if isExpanded}
    <div class="dock-body-content">
      <!-- ================= 1. TIMELINE CONTENT ================= -->
      {#if activeTab === 'timeline'}
        <div class="timeline-panel">
          <div class="timeline-sub-bar">
            <!-- Scrubber Slider -->
            <div class="scrubber-row">
              <span class="mono" style="font-size: 11px; color: var(--text-muted);">Scrubber:</span>
              <input
                type="range"
                class="scrubber-range"
                min="0"
                max={editorState.timelineDuration}
                step="0.01"
                value={editorState.currentTime}
                oninput={handleScrubberInput}
              />
            </div>

            <!-- Loop & Compliance Toggles -->
            <div class="timeline-toggles">
              <label class="toggle-label mono">
                <input
                  type="checkbox"
                  checked={editorState.timelineLoop}
                  onchange={(e) => (editorState.timelineLoop = (e.target as HTMLInputElement).checked)}
                />
                Loop
              </label>

              {#if editorState.staticOnlyLock}
                <span class="compliance-badge lock mono">STATIC-ONLY LOCK ACTIVE</span>
              {:else}
                <span class="compliance-badge pass mono">ANTIBOT COMPLIANT</span>
              {/if}
            </div>
          </div>

          <!-- Add Track Row -->
          <div class="add-track-bar">
            <select class="select-dropdown mono" bind:value={selectedProperty}>
              <option value="global.rotation">Rotation (all layers)</option>
              <option value="global.scale">Scale (all layers)</option>
              <option value="global.opacity">Opacity (all layers)</option>
              <option value="layers[0].gap">Primary Arm Gap</option>
              <option value="layers[0].length">Primary Arm Length</option>
            </select>

            <select class="select-dropdown mono" bind:value={selectedKind}>
              <option value="rotate">Continuous Spin</option>
              <option value="pulse">Sine Pulse (Grow/Shrink)</option>
              <option value="breathe">Smooth Breathe</option>
              <option value="blink">Square Blink</option>
            </select>

            <button class="btn btn-sm btn-primary" onclick={addQuickTrack}>
              + Add Anim Track
            </button>
          </div>

          <!-- Tracks List -->
          <div class="tracks-list">
            {#if editorState.animTracks.length === 0}
              <div class="empty-tracks mono">
                No active animation tracks. Add a track above to preview procedural movement (§4.7).
              </div>
            {:else}
              {#each editorState.animTracks as track (track.id)}
                <div class="track-card mono">
                  <span class="track-kind-badge">{track.kind}</span>
                  <span class="track-target">{track.target_path}</span>
                  <span class="track-duration">{track.duration_seconds}s</span>
                  <button
                    class="btn btn-sm btn-delete-track"
                    onclick={() => editorState.removeTrack(track.id)}
                    title="Remove Track"
                  >
                    ✕
                  </button>
                </div>
              {/each}
            {/if}
          </div>
        </div>

      <!-- ================= 2. SPRAY LAB CONTENT ================= -->
      {:else if activeTab === 'spray_lab'}
        <div class="spray-lab-panel">
          <div class="spray-toolbar">
            <span class="sub-title mono">Gun Template:</span>
            <div class="gun-chips">
              {#each editorState.gunProfiles as gun}
                <button
                  class="gun-chip mono {editorState.activeGunProfile.id === gun.id ? 'active' : ''}"
                  onclick={() => (editorState.activeGunProfile = gun)}
                >
                  {gun.name}
                </button>
              {/each}
            </div>

            <button
              class="btn btn-sm btn-primary"
              onclick={() => {
                editorState.simulatedSpread = 4.5;
                setTimeout(() => (editorState.simulatedSpread = 1.2), 600);
              }}
            >
              💥 Fire Burst (Simulation)
            </button>

            <button
              class="btn btn-sm"
              onclick={() => (editorState.sprayLabOpen = true)}
            >
              Open Full Spray Lab Window ↗
            </button>
          </div>

          <div class="spray-preview-row">
            <div class="spray-stat-card mono">
              <span class="stat-name">Base Spread</span>
              <span class="stat-val">{editorState.activeGunProfile.spread.base}°</span>
            </div>
            <div class="spray-stat-card mono">
              <span class="stat-name">Max Spread</span>
              <span class="stat-val">{editorState.activeGunProfile.spread.max}°</span>
            </div>
            <div class="spray-stat-card mono">
              <span class="stat-name">Fire Rate</span>
              <span class="stat-val">{editorState.activeGunProfile.fire_rate_hz} Hz</span>
            </div>
            <div class="spray-stat-card mono">
              <span class="stat-name">Live Recoil Delta</span>
              <span class="stat-val accent">+{((editorState.simulatedSpread - 1.2) * 2).toFixed(1)} px</span>
            </div>
          </div>
        </div>

      <!-- ================= 3. CALIBRATION CONTENT ================= -->
      {:else if activeTab === 'calibration'}
        <div class="calib-panel">
          <div class="calib-toolbar">
            <button
              class="btn btn-sm {editorState.calibrationPatternActive ? 'btn-primary' : ''}"
              onclick={() => editorState.toggleCalibrationPattern()}
            >
              {editorState.calibrationPatternActive ? 'Hide Test Pattern' : 'Show Calibration Pattern'}
            </button>

            <button
              class="btn btn-sm"
              onclick={() => (editorState.calibrationOpen = true)}
            >
              Open 4-Step Calibration Wizard ↗
            </button>

            <span class="mono" style="font-size: 11px; color: var(--text-muted); margin-left: auto;">
              Offset: X: {editorState.calibrationOffset.x}px · Y: {editorState.calibrationOffset.y}px
            </span>

            <button
              class="btn btn-sm"
              onclick={() => {
                editorState.calibrationOffset = { x: 0, y: 0 };
                editorState.syncWithOverlay();
              }}
            >
              Reset Offset (0, 0)
            </button>
          </div>

          <div class="calib-info-box mono">
            <span class="info-icon">💡</span>
            <span>
              The calibration pattern displays pixel-accurate sub-pixel crosshairs, concentric target rings, and edge guides directly over your game window to verify center alignment across native and stretched aspect ratios (§6.7).
            </span>
          </div>
        </div>

      <!-- ================= 4. MCP LIVE ACTIVITY CONTENT ================= -->
      {:else if activeTab === 'mcp_activity'}
        <div class="mcp-panel">
          <div class="mcp-toolbar">
            <div class="mcp-status-pill">
              <span class="status-indicator-dot {editorState.isAgentEditing ? 'active' : ''}"></span>
              <span class="mono" style="font-size: 11px;">
                {editorState.isAgentEditing ? 'AGENT EDITING IN PROGRESS' : 'MCP SERVER CONNECTED (127.0.0.1)'}
              </span>
            </div>

            <button
              class="btn btn-sm"
              onclick={() => editorState.clearMcpLog()}
              disabled={editorState.mcpActivityLog.length === 0}
            >
              Clear Log
            </button>

            <button
              class="btn btn-sm"
              onclick={() => (editorState.mcpPanelOpen = true)}
            >
              Open MCP Inspector ↗
            </button>
          </div>

          <div class="mcp-log-stream mono">
            {#if editorState.mcpActivityLog.length === 0}
              <div class="empty-mcp-log">No recent autonomous agent actions recorded.</div>
            {:else}
              {#each editorState.mcpActivityLog as item (item.id)}
                <div class="mcp-log-item {item.status}">
                  <span class="log-time">{item.timestamp}</span>
                  <span class="log-tool">{item.tool}</span>
                  <span class="log-summary">{item.summary}</span>
                  <span class="log-status-badge {item.status}">{item.status}</span>
                </div>
              {/each}
            {/if}
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .bottom-dock {
    border-top: 1px solid var(--border, #262930);
    background: var(--surface1, #0f1013);
    display: flex;
    flex-direction: column;
    transition: height 160ms cubic-bezier(0.2, 0, 0, 1);
    user-select: none;
    z-index: 50;
    flex-shrink: 0;
  }

  .bottom-dock.is-collapsed {
    height: 32px;
  }

  .bottom-dock.is-expanded {
    height: 220px;
  }

  .dock-header-bar {
    height: 32px;
    background: var(--surface2, #16181d);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px;
    border-bottom: 1px solid var(--border, #262930);
  }

  .tabs-group {
    display: flex;
    height: 100%;
    gap: 2px;
  }

  .dock-tab-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 10px;
    height: 100%;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-muted, #8b949e);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition: all 120ms ease;
  }

  .dock-tab-btn:hover {
    color: var(--text, #f0f6fc);
    background: rgba(255, 255, 255, 0.03);
  }

  .dock-tab-btn.active {
    color: var(--text, #f0f6fc);
    border-bottom-color: var(--accent, #38bdf8);
    background: var(--surface1, #0f1013);
  }

  .tab-icon {
    font-size: 12px;
  }

  .play-pulse-dot,
  .calib-active-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--accent, #38bdf8);
    box-shadow: 0 0 6px var(--accent, #38bdf8);
  }

  .agent-pulse-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #10b981;
    box-shadow: 0 0 8px #10b981;
    animation: pulse 1s infinite alternate;
  }

  @keyframes pulse {
    from { opacity: 0.5; }
    to { opacity: 1; }
  }

  .log-count-badge {
    font-size: 9px;
    background: var(--surface3, #1e2128);
    color: var(--text-muted, #8b949e);
    padding: 1px 4px;
    border-radius: 99px;
  }

  .dock-right-controls {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .timeline-quick-controls {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .btn-play-mini {
    background: var(--surface3, #1e2128);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-xs, 4px);
    color: var(--text, #f0f6fc);
    font-size: 10px;
    width: 22px;
    height: 20px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btn-play-mini.playing {
    border-color: var(--accent, #38bdf8);
    color: var(--accent, #38bdf8);
  }

  .time-readout {
    font-size: 10px;
    color: var(--text-muted, #8b949e);
  }

  .dock-toggle-btn {
    background: transparent;
    border: none;
    color: var(--text-muted, #8b949e);
    cursor: pointer;
    padding: 2px 6px;
    border-radius: var(--radius-xs, 4px);
  }

  .dock-toggle-btn:hover {
    color: var(--text, #f0f6fc);
    background: var(--surface3, #1e2128);
  }

  .dock-body-content {
    flex: 1;
    overflow-y: auto;
    padding: 10px 14px;
    display: flex;
    flex-direction: column;
  }

  /* Timeline Styles */
  .timeline-panel {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .timeline-sub-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .scrubber-row {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
  }

  .scrubber-range {
    flex: 1;
    height: 4px;
    accent-color: var(--accent, #38bdf8);
  }

  .timeline-toggles {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .toggle-label {
    font-size: 11px;
    color: var(--text-muted, #8b949e);
    display: flex;
    align-items: center;
    gap: 4px;
    cursor: pointer;
  }

  .compliance-badge {
    font-size: 9px;
    padding: 2px 6px;
    border-radius: var(--radius-xs, 4px);
  }

  .compliance-badge.pass {
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
    border: 1px solid #10b981;
  }

  .compliance-badge.lock {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
    border: 1px solid #ef4444;
  }

  .add-track-bar {
    display: flex;
    gap: 8px;
  }

  .select-dropdown {
    height: 28px;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    color: var(--text, #f0f6fc);
    font-size: 11px;
    padding: 0 8px;
  }

  .tracks-list {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    max-height: 90px;
    overflow-y: auto;
  }

  .empty-tracks {
    font-size: 11px;
    color: var(--text-muted, #6e7681);
    padding: 12px 0;
  }

  .track-card {
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    padding: 4px 8px;
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
  }

  .track-kind-badge {
    color: var(--accent, #38bdf8);
    font-weight: 600;
  }

  .track-target {
    color: var(--text, #f0f6fc);
  }

  .track-duration {
    color: var(--text-muted, #8b949e);
  }

  .btn-delete-track {
    background: transparent;
    border: none;
    color: var(--text-muted, #8b949e);
    cursor: pointer;
    font-size: 10px;
    padding: 2px 4px;
  }

  .btn-delete-track:hover {
    color: #ef4444;
  }

  /* Spray Lab Styles */
  .spray-lab-panel {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .spray-toolbar {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .sub-title {
    font-size: 11px;
    color: var(--text-muted, #8b949e);
  }

  .gun-chips {
    display: flex;
    gap: 4px;
  }

  .gun-chip {
    font-size: 11px;
    padding: 4px 8px;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    color: var(--text, #f0f6fc);
    cursor: pointer;
  }

  .gun-chip.active {
    border-color: var(--accent, #38bdf8);
    background: var(--surface3, #1e2128);
  }

  .spray-preview-row {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 10px;
  }

  .spray-stat-card {
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    padding: 8px 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .stat-name {
    font-size: 10px;
    color: var(--text-muted, #8b949e);
  }

  .stat-val {
    font-size: 14px;
    font-weight: 600;
    color: var(--text, #f0f6fc);
  }

  .stat-val.accent {
    color: var(--accent, #38bdf8);
  }

  /* Calibration Styles */
  .calib-panel {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .calib-toolbar {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .calib-info-box {
    display: flex;
    gap: 8px;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    padding: 8px 12px;
    font-size: 11px;
    color: var(--text-muted, #8b949e);
    line-height: 1.5;
  }

  .info-icon {
    font-size: 14px;
  }

  /* MCP Styles */
  .mcp-panel {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .mcp-toolbar {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .mcp-status-pill {
    display: flex;
    align-items: center;
    gap: 6px;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    padding: 4px 10px;
    border-radius: var(--radius-sm, 6px);
  }

  .status-indicator-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--text-muted, #8b949e);
  }

  .status-indicator-dot.active {
    background: #10b981;
    box-shadow: 0 0 8px #10b981;
  }

  .mcp-log-stream {
    max-height: 120px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 4px;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    padding: 6px;
  }

  .empty-mcp-log {
    font-size: 11px;
    color: var(--text-muted, #6e7681);
    padding: 8px;
    text-align: center;
  }

  .mcp-log-item {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    padding: 2px 6px;
    border-radius: var(--radius-xs, 4px);
  }

  .mcp-log-item.success {
    background: rgba(16, 185, 129, 0.05);
  }

  .mcp-log-item.error {
    background: rgba(239, 68, 68, 0.05);
  }

  .log-time {
    color: var(--text-muted, #8b949e);
    font-size: 10px;
  }

  .log-tool {
    color: var(--accent, #38bdf8);
    font-weight: 600;
  }

  .log-summary {
    color: var(--text, #f0f6fc);
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .log-status-badge {
    font-size: 9px;
    padding: 1px 4px;
    border-radius: 3px;
    text-transform: uppercase;
  }

  .log-status-badge.success {
    color: #10b981;
    background: rgba(16, 185, 129, 0.15);
  }

  .log-status-badge.error {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.15);
  }
</style>
