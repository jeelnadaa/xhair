<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import type { NudgeTarget, NudgeFrame } from '../lib/types';
  import Tooltip from './Tooltip.svelte';

  const stepOptions = [0.1, 0.25, 0.5, 1, 5, 10];
  const targetOptions: { id: NudgeTarget; label: string }[] = [
    { id: 'layer', label: 'Selection' },
    { id: 'all', label: 'Crosshair' },
    { id: 'arm', label: 'Arm' },
  ];
  const frameOptions: { id: NudgeFrame; label: string }[] = [
    { id: 'screen', label: 'Screen axes' },
    { id: 'local', label: 'Layer-local' },
  ];

  function handleKeydown(e: KeyboardEvent) {
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
      return;
    }

    let multiplier = 1;
    if (e.shiftKey) multiplier = 10;
    if (e.altKey) multiplier = 0.1 / editorState.nudgeStep; // exact 0.1 px

    const isCtrl = e.ctrlKey || e.metaKey;

    if (e.key === 'ArrowUp') {
      e.preventDefault();
      editorState.nudge(0, -1, multiplier, isCtrl);
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      editorState.nudge(0, 1, multiplier, isCtrl);
    } else if (e.key === 'ArrowLeft') {
      e.preventDefault();
      editorState.nudge(-1, 0, multiplier, isCtrl);
    } else if (e.key === 'ArrowRight') {
      e.preventDefault();
      editorState.nudge(1, 0, multiplier, isCtrl);
    } else if (e.key === 'Home') {
      e.preventDefault();
      if (isCtrl) {
        editorState.recenterCrosshair();
      } else {
        editorState.recenterSelection();
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="panel">
  <div class="panel-header">
    <span>Nudge pad (§5)</span>
    <span class="mono" style="color: var(--text-muted); font-size: 11px;">
      {editorState.nudgeStep} px step
    </span>
  </div>

  <!-- 3x3 Directional Grid -->
  <div class="nudge-grid">
    <!-- Row 1: NW, N, NE -->
    <Tooltip title="Nudge ↖ (North-West)" shortcut="Up + Left" details="Step: {editorState.nudgeStep} px">
      <button class="btn btn-nudge" onclick={() => editorState.nudge(-1, -1)} aria-label="Nudge ↖">
        ↖
      </button>
    </Tooltip>

    <Tooltip title="Nudge ↑ (North)" shortcut="ArrowUp" details="Step: {editorState.nudgeStep} px">
      <button class="btn btn-nudge" onclick={() => editorState.nudge(0, -1)} aria-label="Nudge ↑">
        ↑
      </button>
    </Tooltip>

    <Tooltip title="Nudge ↗ (North-East)" shortcut="Up + Right" details="Step: {editorState.nudgeStep} px">
      <button class="btn btn-nudge" onclick={() => editorState.nudge(1, -1)} aria-label="Nudge ↗">
        ↗
      </button>
    </Tooltip>

    <!-- Row 2: W, Center Reset, E -->
    <Tooltip title="Nudge ← (West)" shortcut="ArrowLeft" details="Step: {editorState.nudgeStep} px">
      <button class="btn btn-nudge" onclick={() => editorState.nudge(-1, 0)} aria-label="Nudge ←">
        ←
      </button>
    </Tooltip>

    <Tooltip title="Recenter Selection" description="Reset selected layers to (0, 0)" shortcut="Home">
      <button class="btn btn-nudge btn-reset" onclick={() => editorState.recenterSelection()} aria-label="Reset offset">
        •
      </button>
    </Tooltip>

    <Tooltip title="Nudge → (East)" shortcut="ArrowRight" details="Step: {editorState.nudgeStep} px">
      <button class="btn btn-nudge" onclick={() => editorState.nudge(1, 0)} aria-label="Nudge →">
        →
      </button>
    </Tooltip>

    <!-- Row 3: SW, S, SE -->
    <Tooltip title="Nudge ↙ (South-West)" shortcut="Down + Left" details="Step: {editorState.nudgeStep} px">
      <button class="btn btn-nudge" onclick={() => editorState.nudge(-1, 1)} aria-label="Nudge ↙">
        ↙
      </button>
    </Tooltip>

    <Tooltip title="Nudge ↓ (South)" shortcut="ArrowDown" details="Step: {editorState.nudgeStep} px">
      <button class="btn btn-nudge" onclick={() => editorState.nudge(0, 1)} aria-label="Nudge ↓">
        ↓
      </button>
    </Tooltip>

    <Tooltip title="Nudge ↘ (South-East)" shortcut="Down + Right" details="Step: {editorState.nudgeStep} px">
      <button class="btn btn-nudge" onclick={() => editorState.nudge(1, 1)} aria-label="Nudge ↘">
        ↘
      </button>
    </Tooltip>
  </div>

  <!-- Step Chips & Custom Typed Step -->
  <div class="step-chips-row mono">
    <div class="step-chips">
      {#each stepOptions as s}
        <button
          class="btn btn-step {editorState.nudgeStep === s ? 'btn-selected' : ''}"
          onclick={() => (editorState.nudgeStep = s)}
        >
          {s}
        </button>
      {/each}
    </div>
    <Tooltip title="Custom Step" description="Type custom nudge step (min 0.01 px)">
      <input
        type="number"
        class="input-step mono"
        bind:value={editorState.nudgeStep}
        min="0.01"
        step="0.01"
        aria-label="Custom nudge step"
      />
    </Tooltip>
  </div>

  <!-- Crisp vs Smooth hint (§5) -->
  {#if editorState.nudgeStep < 1}
    <div class="subpixel-hint mono">
      ℹ Sub-pixel nudges require Smooth mode for sub-pixel anti-aliasing.
    </div>
  {/if}

  <!-- Target Selector (Selection | Crosshair | Arm) -->
  <div class="segmented-control">
    {#each targetOptions as t}
      <button
        class="btn seg-btn {editorState.nudgeTarget === t.id ? 'btn-selected' : ''}"
        onclick={() => (editorState.nudgeTarget = t.id)}
      >
        {t.label}
      </button>
    {/each}
  </div>

  <!-- Frame Selector (Screen | Local) -->
  <div class="frame-selector mono">
    {#each frameOptions as f}
      <button
        class="btn btn-frame {editorState.nudgeFrame === f.id ? 'btn-selected' : ''}"
        onclick={() => (editorState.nudgeFrame = f.id)}
      >
        {f.label}
      </button>
    {/each}
  </div>

  <!-- Recenter Action Buttons (§6) -->
  <div class="recenter-actions">
    <button class="btn btn-recenter" onclick={() => editorState.recenterSelection()}>
      Recenter selection
    </button>
    <button class="btn btn-recenter" onclick={() => editorState.recenterCrosshair()}>
      Recenter whole crosshair
    </button>
  </div>

  <!-- Arm Selection Sub-bar when Arm target is active -->
  {#if editorState.nudgeTarget === 'arm'}
    <div class="arm-selector-group">
      <div class="arm-buttons mono">
        {#each [
          { idx: 0, label: 'Top (↑)' },
          { idx: 1, label: 'Right (→)' },
          { idx: 2, label: 'Bottom (↓)' },
          { idx: 3, label: 'Left (←)' }
        ] as a}
          <button
            class="btn btn-arm {editorState.selectedArmIndex === a.idx ? 'btn-selected' : ''}"
            onclick={() => editorState.selectArm(a.idx)}
          >
            {a.label}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Active Target Offset Readout -->
  <div class="offset-status mono">
    {#if editorState.nudgeTarget === 'all'}
      <span class="status-lbl">All:</span>
      <span class="offset-val">
        X {editorState.crosshair.offsetX >= 0 ? `+${editorState.crosshair.offsetX}` : editorState.crosshair.offsetX} · Y {editorState.crosshair.offsetY >= 0 ? `+${editorState.crosshair.offsetY}` : editorState.crosshair.offsetY} px
      </span>
    {:else if editorState.nudgeTarget === 'layer'}
      <span class="status-lbl">{editorState.selectedLayers.length > 1 ? `${editorState.selectedLayers.length} Layers` : (editorState.selectedLayer?.name || 'Layer')}:</span>
      <span class="offset-val">
        X {(editorState.selectedLayer?.offsetX ?? 0) >= 0 ? `+${editorState.selectedLayer?.offsetX ?? 0}` : editorState.selectedLayer?.offsetX} · Y {(editorState.selectedLayer?.offsetY ?? 0) >= 0 ? `+${editorState.selectedLayer?.offsetY ?? 0}` : editorState.selectedLayer?.offsetY} px
      </span>
    {:else}
      {@const armName = ['Top', 'Right', 'Bottom', 'Left'][editorState.selectedArmIndex] || `Arm ${editorState.selectedArmIndex}`}
      {@const armOff = editorState.selectedLayer?.shape.armOffsets?.[editorState.selectedArmIndex] || { x: 0, y: 0 }}
      <span class="status-lbl">{armName} Arm:</span>
      <span class="offset-val">
        X {armOff.x >= 0 ? `+${armOff.x}` : armOff.x} · Y {armOff.y >= 0 ? `+${armOff.y}` : armOff.y} px
      </span>
    {/if}
  </div>

  <!-- Frame Footnote -->
  <p class="frame-hint">
    Arrow keys = nudge · Shift = 10× · Alt = 0.1 px · Ctrl = Whole crosshair
  </p>
</div>

<style>
  .nudge-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 6px;
    margin-bottom: 12px;
  }

  .btn-nudge {
    height: 36px;
    padding: 0;
    width: 100%;
    font-size: 15px;
    font-weight: 600;
  }

  .btn-reset {
    font-size: 20px;
    color: var(--accent);
  }

  .step-chips-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 8px;
  }

  .step-chips {
    display: flex;
    gap: 4px;
    flex: 1;
    overflow-x: auto;
  }

  .input-step {
    width: 52px;
    height: 26px;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text);
    font-size: 11px;
    text-align: center;
  }

  .subpixel-hint {
    font-size: 10px;
    color: var(--warning);
    background: var(--warning-dim);
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    margin-bottom: 8px;
    line-height: 1.3;
  }

  .btn-step {
    height: 26px;
    padding: 0 6px;
    font-size: 11px;
    flex: 1;
  }

  .segmented-control {
    display: flex;
    gap: 4px;
    margin-bottom: 8px;
  }

  .frame-selector {
    display: flex;
    gap: 4px;
    margin-bottom: 8px;
  }

  .btn-frame {
    flex: 1;
    height: 24px;
    font-size: 10px;
    background: var(--surface2);
    border: 1px solid var(--border);
    color: var(--text-muted);
  }

  .recenter-actions {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
    margin-bottom: 8px;
  }

  .btn-recenter {
    height: 26px;
    font-size: 11px;
    background: var(--surface2);
    border: 1px solid var(--border);
    color: var(--text);
  }

  .btn-recenter:hover {
    background: var(--surface3);
    border-color: var(--border-active);
  }

  .seg-btn {
    flex: 1;
    font-size: 12px;
  }

  .frame-hint {
    color: var(--text-muted);
    margin-top: 10px;
    font-size: 11px;
    line-height: 1.3;
  }

  .arm-selector-group {
    margin-top: 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .arm-buttons {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 4px;
  }

  .btn-arm {
    height: 24px;
    padding: 0 4px;
    font-size: 10px;
    border-radius: var(--radius-sm);
  }

  .offset-status {
    margin-top: 10px;
    padding: 6px 8px;
    background: rgba(18, 22, 27, 0.7);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 11px;
  }

  .status-lbl {
    color: var(--text-muted);
  }

  .offset-val {
    color: var(--accent);
    font-weight: 600;
  }
</style>
