<script lang="ts">
  import TopBar from './components/TopBar.svelte';
  import LibrarySidebar from './components/LibrarySidebar.svelte';
  import CanvasGrid from './components/CanvasGrid.svelte';
  import TimelineDock from './components/TimelineDock.svelte';
  import Inspector from './components/Inspector.svelte';
  import LayersPanel from './components/LayersPanel.svelte';
  import NudgePad from './components/NudgePad.svelte';
  import ColorPicker from './components/ColorPicker.svelte';
  import CommandPalette from './components/CommandPalette.svelte';
  import McpModal from './components/McpModal.svelte';
  import CalibrationWizard from './components/CalibrationWizard.svelte';
  import SprayLab from './components/SprayLab.svelte';
  import DynamicOptInModal from './components/DynamicOptInModal.svelte';
  import CompliancePanel from './components/CompliancePanel.svelte';
  import ImportExportModal from './components/ImportExportModal.svelte';
  import { onMount } from 'svelte';
  import { editorState } from './lib/state.svelte';

  onMount(() => {
    editorState.syncWithOverlay();
  });

  function handleKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
      e.preventDefault();
      editorState.saveCurrentProfile();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app-layout">
  <!-- Top Document Bar -->
  <TopBar />

  <!-- Main 3-Column Body -->
  <div class="main-body">
    <!-- Left: Library Sidebar (240px) -->
    <LibrarySidebar />

    <!-- Center: Interactive Pixel Canvas Grid & Animation Timeline -->
    <main class="center-stage">
      <CanvasGrid />
      <TimelineDock />
    </main>

    <!-- Right: Properties, Layers, Nudge Pad, Color (320px) -->
    <aside class="right-inspector">
      <div class="scroll-container">
        <Inspector />
        <LayersPanel />
        <NudgePad />
        <ColorPicker />
      </div>
    </aside>
  </div>

  <!-- Modals & Drawers -->
  <CommandPalette />
  <McpModal />
  <CalibrationWizard />
  <SprayLab />
  <DynamicOptInModal />
  <CompliancePanel />
  <ImportExportModal />
  <ResolutionModal />
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
  }

  .center-stage {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
  }

  .right-inspector {
    width: 320px;
    flex: none;
    border-left: 1px solid var(--border);
    background: var(--surface1);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .scroll-container {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
</style>
