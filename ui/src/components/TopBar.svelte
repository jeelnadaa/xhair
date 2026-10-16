<script lang="ts">
  import { editorState, RESOLUTION_PROFILES } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  function toggleOverlay() {
    editorState.toggleOverlay();
  }

  function cycleProfile() {
    const idx = RESOLUTION_PROFILES.findIndex((p) => p.id === editorState.activeProfile.id);
    const nextIdx = (idx + 1) % RESOLUTION_PROFILES.length;
    editorState.activeProfile = RESOLUTION_PROFILES[nextIdx];
  }
</script>

<header class="top-bar">
  <div class="brand-group">
    <strong class="brand">XHAIR</strong>
    <span class="sep">/</span>
    <input
      type="text"
      class="doc-title-input"
      bind:value={editorState.crosshair.name}
      onchange={() => editorState.saveCurrentProfile()}
      aria-label="Crosshair Profile Name"
      title="Click or press F2 to edit profile name"
    />
    {#if editorState.isMovedFromCenter}
      <Tooltip
        title="Moved From Center"
        description="Whole crosshair is offset from screen center. Click to reset to (0, 0)"
        shortcut="Ctrl+Home"
      >
        <button class="badge badge-offset" onclick={() => editorState.recenterCrosshair()}>
          ⚠ Offset ({editorState.crosshair.offsetX > 0 ? `+${editorState.crosshair.offsetX}` : editorState.crosshair.offsetX}, {editorState.crosshair.offsetY > 0 ? `+${editorState.crosshair.offsetY}` : editorState.crosshair.offsetY} px)
        </button>
      </Tooltip>
    {/if}
    <Tooltip
      title="Save Profile"
      description="Save current crosshair changes to library and local storage"
      shortcut="Ctrl+S"
    >
      <button
        class="btn btn-save {editorState.saveStatus === 'saved' ? 'saved' : ''}"
        onclick={() => editorState.saveCurrentProfile()}
        aria-label="Save Profile"
      >
        {#if editorState.saveStatus === 'saved'}
          <span class="save-status-icon">✓</span> Saved
        {:else if editorState.saveStatus === 'saving'}
          <span class="save-status-icon">⌛</span> Saving…
        {:else}
          <span class="save-status-icon">💾</span> Save Profile
        {/if}
      </button>
    </Tooltip>
  </div>

  <div class="actions-group">
    <Tooltip title="Undo" description="Undo the last action" shortcut="Ctrl+Z">
      <button
        class="btn btn-icon"
        onclick={() => editorState.undo()}
        disabled={editorState.undoStack.length === 0}
        aria-label="Undo"
      >
        ↶
      </button>
    </Tooltip>

    <Tooltip title="Redo" description="Redo the previous undone action" shortcut="Ctrl+Y">
      <button
        class="btn btn-icon"
        onclick={() => editorState.redo()}
        disabled={editorState.redoStack.length === 0}
        aria-label="Redo"
      >
        ↷
      </button>
    </Tooltip>

    <Tooltip
      title="Keyboard Shortcuts"
      description="View full shortcuts cheat sheet (§3)"
      shortcut="Ctrl+/"
    >
      <button
        class="btn btn-icon"
        onclick={() => (editorState.shortcutsDialogOpen = true)}
        aria-label="Keyboard Shortcuts"
      >
        ⌨
      </button>
    </Tooltip>

    <Tooltip
      title="Import / Export"
      description="Import or share crosshair codes (.xhair, XH1 share code, Valorant code)"
      shortcut="Ctrl+I"
    >
      <button
        class="btn btn-icon"
        onclick={() => editorState.openImportExport()}
        aria-label="Import or Export Crosshair"
      >
        ⇄
      </button>
    </Tooltip>

    <Tooltip
      title="Command Palette"
      description="Open searchable spotlight command palette"
      shortcut="Ctrl+K"
    >
      <button
        class="btn btn-icon"
        onclick={() => (editorState.commandPaletteOpen = true)}
        aria-label="Command Palette"
      >
        ⌘
      </button>
    </Tooltip>
  </div>

  <div class="spacer"></div>

  <div class="status-group">
    <Tooltip
      title="Desktop Overlay"
      description="Toggle real-time transparent game overlay"
      shortcut="F10"
    >
      <button
        class="pill {editorState.overlayEnabled ? 'active' : ''}"
        onclick={toggleOverlay}
      >
        <span class="dot-indicator"></span>
        Overlay {editorState.overlayEnabled ? 'ON' : 'OFF'}
      </button>
    </Tooltip>

    {#if editorState.isAgentEditing}
      <Tooltip
        title="Agent In Progress"
        description="Autonomous coding agent is executing tool commands"
      >
        <div class="pill agent-editing">
          <span class="dot-indicator pulsing"></span>
          Agent is editing
        </div>
      </Tooltip>
    {/if}

    <Tooltip
      title="MCP Server"
      description="AI agent loopback bridge. Click to view live activity log."
      details="Status: Running on 127.0.0.1"
    >
      <button class="pill active" onclick={() => (editorState.mcpPanelOpen = true)}>
        <span class="dot-indicator"></span>
        MCP ON
      </button>
    </Tooltip>

    <Tooltip
      title="Resolution Profile"
      description="Target game render resolution, stretch mode & parity diagnostic (§6.3)"
      details="Click to manage & add custom resolutions"
    >
      <button class="pill" onclick={() => editorState.openResolutionModal()}>
        📺 {editorState.activeProfile.name}
      </button>
    </Tooltip>

    <Tooltip
      title="Calibration Wizard"
      description="Calibrate screen center and verify pixel alignment (§6.7)"
      details="Click to open 4-step wizard & screenshot align"
    >
      <button class="pill" onclick={() => editorState.openCalibration()}>
        🎯 Calibration
      </button>
    </Tooltip>

    <Tooltip
      title="Spray Lab"
      description="Standalone simulated recoil & spray curve visualizer (§12.6, 100% safe)"
      details="Click to test gun profiles and scrub shot impacts"
    >
      <button class="pill" onclick={() => editorState.openSprayLab()}>
        🔫 Spray Lab
      </button>
    </Tooltip>

    <Tooltip
      title="Dynamic Reticle"
      description={editorState.dynamicReticleEnabled
        ? 'Restricted Raw Input simulation active for this session (§12)'
        : 'Off by default. Click to open session risk opt-in (§2)'}
      details="Reacts to user locomotion/fire inputs"
    >
      <button
        class="pill {editorState.dynamicReticleEnabled ? 'active' : ''}"
        onclick={() => (editorState.dynamicReticleEnabled ? editorState.disableDynamicReticle() : editorState.requestDynamicOptIn())}
      >
        <span class="dot-indicator"></span>
        Dynamics {editorState.dynamicReticleEnabled ? 'ON' : 'OFF'}
      </button>
    </Tooltip>

    <Tooltip
      title="Compliance & Audit"
      description="Exhaustive list of whitelisted OS APIs and strict zero-game-memory rules (§2)"
      details="Click to inspect security & compliance guarantees"
    >
      <button class="pill" onclick={() => editorState.openCompliancePanel()}>
        🛡️ Compliance
      </button>
    </Tooltip>

    <Tooltip
      title="Appearance & Themes"
      description="Select neutral theme, typography pack, and corner curvature (§12, §13)"
    >
      <button class="pill" onclick={() => editorState.openAppearanceModal()}>
        🎨 {editorState.theme}
      </button>
    </Tooltip>

    <Tooltip
      title="Visibility Score"
      description="WCAG-style contrast ratio against background"
      details="Score: {editorState.visibilityScore}:1 (High contrast)"
    >
      <div class="pill">
        Contrast {editorState.visibilityScore}:1
      </div>
    </Tooltip>
  </div>
</header>

<style>
  .top-bar {
    height: 52px;
    flex: none;
    border-bottom: 1px solid var(--border);
    background: var(--surface1);
    display: flex;
    align-items: center;
    padding: 0 16px;
    gap: 16px;
  }

  .brand-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .badge-offset {
    background: #261D09;
    color: #FBBF24;
    border: 1px solid #78350F;
    border-radius: var(--radius-sm);
    padding: 3px 8px;
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .badge-offset:hover {
    background: #38280D;
    border-color: #FBBF24;
  }

  .brand {
    font-weight: 800;
    letter-spacing: 0.14em;
    color: var(--accent);
    font-size: 15px;
  }

  .sep {
    color: var(--text-muted);
  }

  .doc-title-input {
    background: transparent;
    border: 1px solid transparent;
    color: var(--text);
    font-weight: 600;
    font-size: 13px;
    padding: 3px 6px;
    border-radius: var(--radius-xs);
    width: 140px;
    transition: all 0.15s ease;
  }

  .doc-title-input:hover {
    border-color: var(--border);
    background: var(--surface2);
  }

  .doc-title-input:focus {
    outline: none;
    border-color: var(--accent);
    background: var(--surface2);
  }

  .btn-save {
    background: var(--accent);
    color: #0B0D10;
    font-weight: 700;
    font-size: 12px;
    padding: 5px 12px;
    border-radius: var(--radius-sm);
    border: none;
    display: inline-flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-save:hover {
    filter: brightness(1.15);
    transform: translateY(-1px);
  }

  .btn-save.saved {
    background: #10B981;
    color: #FFFFFF;
  }

  .save-status-icon {
    font-size: 13px;
  }

  .actions-group {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .spacer {
    flex: 1;
  }

  .status-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .dot-indicator {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: currentColor;
    display: inline-block;
  }

  .pill.agent-editing {
    background: rgba(255, 79, 216, 0.15);
    color: #FF4FD8;
    border-color: #FF4FD8;
    font-weight: 600;
  }

  .pulsing {
    animation: pulse 1.2s infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.3; transform: scale(0.85); }
  }
</style>
