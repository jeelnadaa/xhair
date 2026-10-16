<script lang="ts">
  import { editorState, RESOLUTION_PROFILES } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  let currentGroup = $derived(
    editorState.groups.find((g) => g.id === (editorState.editingGroupId || editorState.activeGroupId)) || editorState.groups[0]
  );

  let groupName = $state('');
  let gameBinding = $state('');
  let groupColor = $state('#10B981');
  let selectedProfile = $state('4-3-stretched');

  $effect(() => {
    if (currentGroup) {
      groupName = currentGroup.name;
      gameBinding = currentGroup.game_binding || '';
      groupColor = currentGroup.color || '#10B981';
      selectedProfile = currentGroup.default_resolution_profile || '4-3-stretched';
    }
  });

  function saveGroupSettings() {
    if (currentGroup) {
      currentGroup.name = groupName.trim() || currentGroup.name;
      currentGroup.game_binding = gameBinding.trim() || undefined;
      currentGroup.color = groupColor;
      currentGroup.default_resolution_profile = selectedProfile;
      editorState.saveCurrentProfile();
    }
    editorState.groupSettingsModalOpen = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      editorState.groupSettingsModalOpen = false;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if editorState.groupSettingsModalOpen && currentGroup}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={() => (editorState.groupSettingsModalOpen = false)}>
    <div class="modal-window" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 8px;">
          <span style="font-size: 16px;">📁</span>
          <h3>Group Settings (§8)</h3>
        </div>
        <button class="btn btn-sm btn-icon" onclick={() => (editorState.groupSettingsModalOpen = false)}>
          ✕
        </button>
      </div>

      <div class="modal-body">
        <!-- Group Name -->
        <div class="form-group">
          <label for="grp-name" class="label">Group Name</label>
          <input
            id="grp-name"
            type="text"
            class="input-text"
            bind:value={groupName}
            placeholder="e.g. Valorant, CS2, Apex"
          />
        </div>

        <!-- Color Dot -->
        <div class="form-group">
          <span class="label">Color Indicator</span>
          <div class="color-options">
            {#each ['#10B981', '#F59E0B', '#38BDF8', '#FF4FD8', '#EF4444', '#8E98A8'] as col}
              <button
                class="color-dot-btn {groupColor === col ? 'selected' : ''}"
                style="background-color: {col};"
                onclick={() => (groupColor = col)}
                aria-label="Color {col}"
              ></button>
            {/each}
          </div>
        </div>

        <!-- Game Process Name Binding -->
        <div class="form-group">
          <label for="grp-proc" class="label">
            <span>Linked Game Process</span>
            <span class="mono hint">Focused window exe name</span>
          </label>
          <input
            id="grp-proc"
            type="text"
            class="input-text mono"
            bind:value={gameBinding}
            placeholder="e.g. VALORANT-Win64-Shipping.exe, cs2.exe"
          />
          <p class="field-footnote">
            Zero game process memory access. Overlay automatically activates when this process is focused (§2).
          </p>
        </div>

        <!-- Default Resolution Profile -->
        <div class="form-group">
          <label for="grp-profile" class="label">Default Resolution Profile</label>
          <select id="grp-profile" class="input-select" bind:value={selectedProfile}>
            {#each RESOLUTION_PROFILES as prof}
              <option value={prof.id}>{prof.name}</option>
            {/each}
          </select>
        </div>

        <!-- Group Hotkeys Summary (§8) -->
        <div class="hotkeys-box mono">
          <div class="hotkey-row">
            <span>Cycle Crosshairs in Group:</span>
            <span class="kbd">Ctrl+Tab / Ctrl+Shift+Tab</span>
          </div>
          <div class="hotkey-row">
            <span>Jump to Slot 1..9:</span>
            <span class="kbd">Ctrl+1 .. Ctrl+9</span>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn" onclick={() => (editorState.groupSettingsModalOpen = false)}>
          Cancel
        </button>
        <button class="btn btn-primary" onclick={saveGroupSettings}>
          Save Settings
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(4, 5, 8, 0.8);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal-window {
    width: 440px;
    background: var(--surface1);
    border: 1px solid var(--border-active);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-modal);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .modal-header {
    padding: 14px 16px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .modal-header h3 {
    font-size: 14px;
    font-weight: 700;
    margin: 0;
  }

  .modal-body {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    display: flex;
    justify-content: space-between;
  }

  .hint {
    font-size: 10px;
    color: var(--text-dim);
  }

  .field-footnote {
    font-size: 10px;
    color: var(--text-dim);
    margin: 0;
    line-height: 1.3;
  }

  .color-options {
    display: flex;
    gap: 8px;
  }

  .color-dot-btn {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
    transition: transform 100ms ease;
  }

  .color-dot-btn:hover {
    transform: scale(1.15);
  }

  .color-dot-btn.selected {
    border-color: #FFF;
    box-shadow: 0 0 0 2px var(--surface1);
  }

  .input-select {
    background: var(--surface2);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    color: var(--text);
    padding: 6px 8px;
    font-size: 12px;
    outline: none;
  }

  .hotkeys-box {
    background: var(--surface2);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: 11px;
  }

  .hotkey-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: var(--text-muted);
  }

  .kbd {
    background: var(--surface3);
    border: 1px solid var(--border);
    padding: 2px 6px;
    border-radius: var(--radius-xs);
    color: var(--text);
    font-size: 10px;
  }

  .modal-footer {
    padding: 12px 16px;
    border-top: 1px solid var(--border);
    background: var(--surface2);
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
</style>
