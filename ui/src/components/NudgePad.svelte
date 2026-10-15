<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import type { NudgeTarget, NudgeFrame } from '../lib/types';
  import Tooltip from './Tooltip.svelte';

  const stepOptions = [0.1, 0.25, 0.5, 1, 5, 10];
  const targetOptions: { id: NudgeTarget; label: string }[] = [
    { id: 'arm', label: 'Arm' },
    { id: 'layer', label: 'Layer' },
    { id: 'all', label: 'All' },
  ];
  const frameOptions: { id: NudgeFrame; label: string }[] = [
    { id: 'screen', label: 'Screen axes' },
    { id: 'local', label: 'Layer-local axes' },
  ];

  function handleKeydown(e: KeyboardEvent) {
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
      return;
    }

    let multiplier = 1;
    if (e.shiftKey) multiplier = 10;
    if (e.altKey) multiplier = 0.2; // fine step

    const isCtrl = e.ctrlKey || e.metaKey;
    const oldTarget = editorState.nudgeTarget;
    if (isCtrl) {
      editorState.nudgeTarget = 'all';
    }

    if (e.key === 'ArrowUp') {
      e.preventDefault();
      editorState.nudge(0, -multiplier);
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      editorState.nudge(0, multiplier);
    } else if (e.key === 'ArrowLeft') {
      e.preventDefault();
      editorState.nudge(-multiplier, 0);
    } else if (e.key === 'ArrowRight') {
      e.preventDefault();
      editorState.nudge(multiplier, 0);
    }

    if (isCtrl) {
      editorState.nudgeTarget = oldTarget;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="panel">
  <div class="panel-header">
    <span>Nudge pad</span>
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

    <Tooltip title="Reset Offset" description="Reset current target offset to (0, 0)">
      <button class="btn btn-nudge btn-reset" onclick={() => editorState.resetNudge()} aria-label="Reset offset">
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

  <!-- Step Chips -->
  <div class="step-chips mono">
    {#each stepOptions as s}
      <button
        class="btn btn-step {editorState.nudgeStep === s ? 'btn-selected' : ''}"
        onclick={() => (editorState.nudgeStep = s)}
      >
        {s}
      </button>
    {/each}
  </div>

  <!-- Target Selector (Arm | Layer | All) -->
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

  <!-- Frame Footnote -->
  <p class="frame-hint">
    Frame: Screen axes · step in game px · Hold Shift (10×) / Alt (0.1×)
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

  .step-chips {
    display: flex;
    gap: 4px;
    margin-bottom: 12px;
    overflow-x: auto;
  }

  .btn-step {
    height: 26px;
    padding: 0 8px;
    font-size: 11px;
    flex: 1;
  }

  .segmented-control {
    display: flex;
    gap: 4px;
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
</style>
