<script lang="ts">
  import { editorState, RESOLUTION_PROFILES } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  function toggleOverlay() {
    editorState.overlayEnabled = !editorState.overlayEnabled;
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
    <span class="doc-title">{editorState.crosshair.name}</span>
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
      description="Target game render resolution and stretch mode"
      details="Click to cycle active profile"
    >
      <button class="pill" onclick={cycleProfile}>
        {editorState.activeProfile.name}
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

  .brand {
    font-weight: 800;
    letter-spacing: 0.14em;
    color: var(--accent);
    font-size: 15px;
  }

  .sep {
    color: var(--text-muted);
  }

  .doc-title {
    font-weight: 600;
    color: var(--text);
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
