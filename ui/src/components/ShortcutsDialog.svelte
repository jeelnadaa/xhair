<script lang="ts">
  import { editorState } from '../lib/state.svelte';

  const shortcutGroups = [
    {
      title: 'Selection & Manipulation (§3)',
      shortcuts: [
        { key: 'Ctrl+A', desc: 'Select all layers' },
        { key: 'Esc', desc: 'Clear selection' },
        { key: 'Ctrl+Shift+I', desc: 'Invert selection' },
        { key: 'Ctrl+Click', desc: 'Add / remove layer from selection' },
        { key: 'Shift+Click', desc: 'Select layer range' },
        { key: 'Ctrl+D', desc: 'Duplicate selected layers' },
        { key: 'Del', desc: 'Delete selected layers' },
        { key: 'Ctrl+G', desc: 'Group selected layers' },
        { key: 'Ctrl+Shift+G', desc: 'Ungroup selected layer group' },
        { key: 'F2', desc: 'Rename layer or crosshair' },
      ],
    },
    {
      title: 'Navigation & Nudge (§5, §6)',
      shortcuts: [
        { key: 'Arrow Keys', desc: 'Nudge selection by current step' },
        { key: 'Shift+Arrow', desc: 'Nudge selection ×10 step' },
        { key: 'Alt+Arrow', desc: 'Nudge selection by 0.1 px' },
        { key: 'Ctrl+Arrow', desc: 'Nudge whole crosshair (independent of selection)' },
        { key: 'Home', desc: 'Recenter selection to (0, 0)' },
        { key: 'Ctrl+Home', desc: 'Recenter whole crosshair to screen center' },
        { key: 'Ctrl+Z / Ctrl+Y', desc: 'Undo / Redo changes' },
        { key: 'Ctrl+S', desc: 'Save active crosshair profile' },
        { key: 'Ctrl+K', desc: 'Open spotlight command palette' },
        { key: 'Ctrl+Shift+X', desc: 'Global toggle desktop overlay' },
      ],
    },
  ];

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      editorState.shortcutsDialogOpen = false;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if editorState.shortcutsDialogOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={() => (editorState.shortcutsDialogOpen = false)}>
    <div class="modal-window" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div style="display: flex; align-items: center; gap: 8px;">
          <span style="font-size: 16px;">⌨</span>
          <h3>Keyboard Shortcuts Cheat Sheet</h3>
        </div>
        <button class="btn btn-sm btn-icon" onclick={() => (editorState.shortcutsDialogOpen = false)}>
          ✕
        </button>
      </div>

      <div class="modal-body">
        <div class="shortcuts-columns">
          {#each shortcutGroups as grp}
            <div class="shortcut-group">
              <h4 class="group-title">{grp.title}</h4>
              <div class="shortcut-list">
                {#each grp.shortcuts as s}
                  <div class="shortcut-item">
                    <span class="shortcut-desc">{s.desc}</span>
                    <span class="kbd mono">{s.key}</span>
                  </div>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      </div>

      <div class="modal-footer">
        <span class="mono footer-note">Press <kbd class="kbd">Esc</kbd> to close</span>
        <button class="btn btn-primary" onclick={() => (editorState.shortcutsDialogOpen = false)}>
          Done
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(4, 5, 8, 0.82);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal-window {
    width: 640px;
    background: var(--surface1);
    border: 1px solid var(--border-active);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-modal);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .modal-header {
    padding: 14px 18px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .modal-header h3 {
    margin: 0;
    font-size: 14px;
    font-weight: 700;
  }

  .modal-body {
    padding: 18px;
    max-height: 480px;
    overflow-y: auto;
  }

  .shortcuts-columns {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }

  .shortcut-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .group-title {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
    margin: 0 0 4px 0;
  }

  .shortcut-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .shortcut-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 5px 8px;
    background: var(--surface2);
    border-radius: var(--radius-sm);
    font-size: 12px;
  }

  .shortcut-desc {
    color: var(--text);
    font-size: 11px;
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
    padding: 12px 18px;
    border-top: 1px solid var(--border);
    background: var(--surface2);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .footer-note {
    font-size: 11px;
    color: var(--text-muted);
  }
</style>
