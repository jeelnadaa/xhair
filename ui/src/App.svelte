<script lang="ts">
  import TopBar from './components/TopBar.svelte';
  import LibrarySidebar from './components/LibrarySidebar.svelte';
  import CanvasGrid from './components/CanvasGrid.svelte';
  import TimelineDock from './components/TimelineDock.svelte';
  import DockSystem from './components/DockSystem.svelte';
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

    <!-- Right: Tabbed Dock System (§10) -->
    <DockSystem />
  </div>

  <!-- Modals & Drawers -->
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
  }

  .center-stage {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
  }
</style>
