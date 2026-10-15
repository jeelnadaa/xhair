<script lang="ts">
  import TopBar from './components/TopBar.svelte';
  import LibrarySidebar from './components/LibrarySidebar.svelte';
  import CanvasGrid from './components/CanvasGrid.svelte';
  import Inspector from './components/Inspector.svelte';
  import LayersPanel from './components/LayersPanel.svelte';
  import NudgePad from './components/NudgePad.svelte';
  import ColorPicker from './components/ColorPicker.svelte';
  import CommandPalette from './components/CommandPalette.svelte';
  import McpModal from './components/McpModal.svelte';
</script>

<div class="app-layout">
  <!-- Top Document Bar -->
  <TopBar />

  <!-- Main 3-Column Body -->
  <div class="main-body">
    <!-- Left: Library Sidebar (240px) -->
    <LibrarySidebar />

    <!-- Center: Interactive Pixel Canvas Grid -->
    <CanvasGrid />

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
