<script lang="ts">
  import TopBar from './components/TopBar.svelte';
  import LibrarySidebar from './components/LibrarySidebar.svelte';
  import CanvasGrid from './components/CanvasGrid.svelte';
  import BottomDock from './components/BottomDock.svelte';
  import Inspector from './components/Inspector.svelte';
  import AppearanceModal from './components/AppearanceModal.svelte';
  import CommandPalette from './components/CommandPalette.svelte';
  import McpModal from './components/McpModal.svelte';
  import CalibrationWizard from './components/CalibrationWizard.svelte';
  import SprayLab from './components/SprayLab.svelte';
  import DynamicOptInModal from './components/DynamicOptInModal.svelte';
  import CompliancePanel from './components/CompliancePanel.svelte';
  import ImportExportModal from './components/ImportExportModal.svelte';
  import ResolutionModal from './components/ResolutionModal.svelte';
  import GroupSettingsModal from './components/GroupSettingsModal.svelte';
  import ShortcutsDialog from './components/ShortcutsDialog.svelte';
  import { onMount } from 'svelte';
  import { editorState } from './lib/state.svelte';

  let isResizingLeft = $state(false);
  let isResizingRight = $state(false);

  onMount(() => {
    editorState.syncWithOverlay();
    editorState.loadLibraryFromDisk();

    const interval = setInterval(() => {
      editorState.fetchMcpActivity();
    }, 2000);

    return () => clearInterval(interval);
  });

  function handleKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
      e.preventDefault();
      editorState.saveCurrentProfile();
    } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'b') {
      e.preventDefault();
      editorState.toggleLeftRail();
    } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'i') {
      e.preventDefault();
      editorState.toggleRightRail();
    }
  }

  function startResizeLeft(e: PointerEvent) {
    e.preventDefault();
    isResizingLeft = true;
    const onPointerMove = (ev: PointerEvent) => {
      editorState.setLeftRailWidth(ev.clientX);
    };
    const onPointerUp = () => {
      isResizingLeft = false;
      window.removeEventListener('pointermove', onPointerMove);
      window.removeEventListener('pointerup', onPointerUp);
    };
    window.addEventListener('pointermove', onPointerMove);
    window.addEventListener('pointerup', onPointerUp);
  }

  function startResizeRight(e: PointerEvent) {
    e.preventDefault();
    isResizingRight = true;
    const onPointerMove = (ev: PointerEvent) => {
      editorState.setRightRailWidth(window.innerWidth - ev.clientX);
    };
    const onPointerUp = () => {
      isResizingRight = false;
      window.removeEventListener('pointermove', onPointerMove);
      window.removeEventListener('pointerup', onPointerUp);
    };
    window.addEventListener('pointermove', onPointerMove);
    window.addEventListener('pointerup', onPointerUp);
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app-layout">
  <!-- Top Document Bar -->
  <TopBar />

  <!-- Main 3-Column Studio Body with Resizable Panels -->
  <div class="main-body {isResizingLeft || isResizingRight ? 'is-resizing' : ''}">
    <!-- Left Rail: Explorer & Layers (Collapsible & Expandable) -->
    {#if !editorState.leftRailCollapsed}
      <div class="rail-container left-container" style="width: {editorState.leftRailWidth}px;">
        <LibrarySidebar />
      </div>
      <!-- Left Draggable Splitter -->
      <div
        class="rail-resizer resizer-left {isResizingLeft ? 'active' : ''}"
        onpointerdown={startResizeLeft}
        role="separator"
        aria-orientation="vertical"
        aria-label="Resize left library sidebar"
        title="Drag to resize library sidebar (Min 260px, Max 540px)"
      >
        <div class="resizer-indicator"></div>
      </div>
    {:else}
      <!-- Left Collapsed Floating Strip -->
      <div class="rail-collapsed-strip left-collapsed-strip">
        <button
          class="btn-expand-rail"
          onclick={() => editorState.toggleLeftRail()}
          title="Expand Library & Layers Sidebar (Ctrl+B)"
          aria-label="Expand Library Sidebar"
        >
          <span class="strip-icon">📁</span>
          <span class="strip-arrow">▶</span>
        </button>
      </div>
    {/if}

    <!-- Center Stage: Interactive Pixel Canvas Viewport & Bottom Dock -->
    <main class="center-stage">
      <CanvasGrid />
      <BottomDock />
    </main>

    <!-- Right Rail: Unified Contextual Inspector (Collapsible & Expandable) -->
    {#if !editorState.rightRailCollapsed}
      <!-- Right Draggable Splitter -->
      <div
        class="rail-resizer resizer-right {isResizingRight ? 'active' : ''}"
        onpointerdown={startResizeRight}
        role="separator"
        aria-orientation="vertical"
        aria-label="Resize right inspector panel"
        title="Drag to resize inspector panel (Min 280px, Max 560px)"
      >
        <div class="resizer-indicator"></div>
      </div>
      <div class="rail-container right-container" style="width: {editorState.rightRailWidth}px;">
        <Inspector />
      </div>
    {:else}
      <!-- Right Collapsed Floating Strip -->
      <div class="rail-collapsed-strip right-collapsed-strip">
        <button
          class="btn-expand-rail"
          onclick={() => editorState.toggleRightRail()}
          title="Expand Properties Inspector (Ctrl+I)"
          aria-label="Expand Inspector"
        >
          <span class="strip-arrow">◀</span>
          <span class="strip-icon">⚙</span>
        </button>
      </div>
    {/if}
  </div>

  <!-- Global Modals & Overlays -->
  <CommandPalette />
  <McpModal />
  <AppearanceModal />
  <CalibrationWizard />
  <SprayLab />
  <DynamicOptInModal />
  <CompliancePanel />
  <ImportExportModal />
  <ResolutionModal />
  <GroupSettingsModal />
  <ShortcutsDialog />
</div>

<style>
  .app-layout {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--bg);
  }

  .main-body {
    display: flex;
    flex: 1;
    min-height: 0;
    min-width: 0;
    position: relative;
  }

  .main-body.is-resizing {
    user-select: none;
    -webkit-user-select: none;
    cursor: col-resize;
  }

  .rail-container {
    height: 100%;
    min-height: 0;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    position: relative;
    overflow: hidden;
  }

  .left-container {
    border-right: 1px solid var(--border, #262626);
  }

  .right-container {
    border-left: 1px solid var(--border, #262626);
  }

  /* Draggable Splitters / Resizers */
  .rail-resizer {
    width: 7px;
    height: 100%;
    position: relative;
    cursor: col-resize;
    user-select: none;
    z-index: 40;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    transition: background 150ms ease;
    flex-shrink: 0;
    margin: 0 -3px;
  }

  .rail-resizer:hover,
  .rail-resizer.active {
    background: rgba(56, 189, 248, 0.25);
  }

  .resizer-indicator {
    width: 2px;
    height: 24px;
    background: var(--border, #3a3a3a);
    border-radius: 2px;
    transition: all 150ms ease;
  }

  .rail-resizer:hover .resizer-indicator,
  .rail-resizer.active .resizer-indicator {
    height: 48px;
    background: var(--accent, #38bdf8);
  }

  /* Collapsed Rail Strip with Expand Button */
  .rail-collapsed-strip {
    width: 32px;
    height: 100%;
    background: var(--surface1, #111111);
    border-right: 1px solid var(--border, #262626);
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 10px;
    z-index: 35;
    flex-shrink: 0;
  }

  .right-collapsed-strip {
    border-right: none;
    border-left: 1px solid var(--border, #262626);
  }

  .btn-expand-rail {
    width: 26px;
    height: 52px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    background: var(--surface2, #171717);
    border: 1px solid var(--border, #262626);
    border-radius: var(--radius-xs, 4px);
    color: var(--text-muted, #a0a0a0);
    cursor: pointer;
    font-size: 11px;
    transition: all 120ms ease;
    padding: 0;
  }

  .btn-expand-rail:hover {
    background: var(--surface3, #1f1f1f);
    color: var(--text, #ededed);
    border-color: var(--accent, #38bdf8);
  }

  .strip-icon {
    font-size: 12px;
  }

  .strip-arrow {
    font-size: 9px;
  }

  .center-stage {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    background: var(--bg);
  }
</style>
