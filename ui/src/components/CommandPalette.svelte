<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import { SHORTCUTS_REGISTRY, type CommandShortcut } from '../lib/shortcuts';

  let searchQuery = $state('');
  let selectedIndex = $state(0);
  let inputEl = $state<HTMLInputElement | null>(null);

  let filteredCommands = $derived(
    SHORTCUTS_REGISTRY.filter(
      (cmd) =>
        cmd.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        cmd.category.toLowerCase().includes(searchQuery.toLowerCase()) ||
        cmd.description.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  function executeCommand(cmd: CommandShortcut) {
    editorState.commandPaletteOpen = false;

    if (cmd.action === 'undo') editorState.undo();
    else if (cmd.action === 'redo') editorState.redo();
    else if (cmd.action === 'toggleGrid') editorState.showGrid = !editorState.showGrid;
    else if (cmd.action === 'toggleRulers') editorState.showRulers = !editorState.showRulers;
    else if (cmd.action === 'toggleGuides') editorState.showGuides = !editorState.showGuides;
    else if (cmd.action === 'toggleOverlay') editorState.overlayEnabled = !editorState.overlayEnabled;
    else if (cmd.action.startsWith('selectTool:')) {
      const tool = cmd.action.split(':')[1] as any;
      editorState.activeTool = tool;
    } else if (cmd.action.startsWith('nudge:')) {
      const dir = cmd.action.split(':')[1];
      if (dir === 'up') editorState.nudge(0, -1);
      else if (dir === 'down') editorState.nudge(0, 1);
      else if (dir === 'left') editorState.nudge(-1, 0);
      else if (dir === 'right') editorState.nudge(1, 0);
    } else if (cmd.action.startsWith('nudgeAll:')) {
      const dir = cmd.action.split(':')[1];
      const prev = editorState.nudgeTarget;
      editorState.nudgeTarget = 'all';
      if (dir === 'up') editorState.nudge(0, -1);
      editorState.nudgeTarget = prev;
    } else if (cmd.action === 'zoomIn') {
      editorState.zoom = Math.min(64, editorState.zoom * 2);
    } else if (cmd.action === 'zoomOut') {
      editorState.zoom = Math.max(1, editorState.zoom / 2);
    } else if (cmd.action === 'zoomReset') {
      editorState.zoom = 1;
      editorState.panX = 0;
      editorState.panY = 0;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!editorState.commandPaletteOpen) {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        editorState.commandPaletteOpen = true;
        searchQuery = '';
        selectedIndex = 0;
        setTimeout(() => inputEl?.focus(), 50);
      }
      return;
    }

    if (e.key === 'Escape') {
      e.preventDefault();
      editorState.commandPaletteOpen = false;
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = (selectedIndex + 1) % Math.max(1, filteredCommands.length);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex =
        (selectedIndex - 1 + filteredCommands.length) % Math.max(1, filteredCommands.length);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (filteredCommands[selectedIndex]) {
        executeCommand(filteredCommands[selectedIndex]);
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if editorState.commandPaletteOpen}
  <div
    class="palette-backdrop"
    onclick={() => (editorState.commandPaletteOpen = false)}
    role="presentation"
  >
    <div
      class="palette-modal"
      onclick={(e) => e.stopPropagation()}
      onkeydown={handleKeydown}
      role="dialog"
      tabindex="-1"
      aria-label="Command Palette"
    >
      <div class="palette-search">
        <span class="search-icon">🔍</span>
        <input
          bind:this={inputEl}
          type="text"
          class="palette-input"
          placeholder="Type a command or search action…"
          bind:value={searchQuery}
          aria-label="Command search"
        />
        <kbd class="esc-kbd">ESC</kbd>
      </div>

      <div class="commands-list">
        {#each filteredCommands as cmd, idx (cmd.id)}
          <button
            class="command-item {idx === selectedIndex ? 'selected' : ''}"
            onclick={() => executeCommand(cmd)}
            onmouseenter={() => (selectedIndex = idx)}
          >
            <div class="command-info">
              <div class="command-name">
                <span class="command-cat">{cmd.category}</span>
                <span>{cmd.name}</span>
              </div>
              <div class="command-desc">{cmd.description}</div>
            </div>
            {#if cmd.shortcut}
              <kbd class="command-shortcut">{cmd.shortcut}</kbd>
            {/if}
          </button>
        {:else}
          <div class="no-commands">No commands match "{searchQuery}"</div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .palette-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(4px);
    z-index: 99999;
    display: flex;
    justify-content: center;
    padding-top: 15vh;
  }

  .palette-modal {
    width: 580px;
    max-height: 480px;
    background: var(--surface1);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-modal);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: slideDown 120ms ease;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .palette-search {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border);
    gap: 12px;
  }

  .search-icon {
    font-size: 14px;
    opacity: 0.6;
  }

  .palette-input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text);
    font-size: 14px;
    outline: none;
    font-family: var(--font-sans);
  }

  .esc-kbd {
    font-size: 10px;
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    background: var(--surface2);
    border: 1px solid var(--border);
    color: var(--text-muted);
  }

  .commands-list {
    flex: 1;
    overflow-y: auto;
    padding: 6px;
  }

  .command-item {
    width: 100%;
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    padding: 8px 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    text-align: left;
    color: var(--text);
    transition: background 60ms ease;
  }

  .command-item.selected {
    background: var(--accent-dim);
    border-color: var(--accent);
  }

  .command-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .command-name {
    font-size: 13px;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .command-cat {
    font-size: 10px;
    padding: 1px 5px;
    border-radius: var(--radius-sm);
    background: var(--surface2);
    color: var(--text-muted);
    font-weight: 500;
  }

  .command-desc {
    font-size: 11px;
    color: var(--text-muted);
  }

  .command-shortcut {
    font-family: var(--font-mono);
    font-size: 11px;
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    background: var(--bg);
    border: 1px solid var(--border);
    color: var(--accent);
  }

  .no-commands {
    padding: 24px;
    text-align: center;
    color: var(--text-muted);
    font-size: 13px;
  }
</style>
