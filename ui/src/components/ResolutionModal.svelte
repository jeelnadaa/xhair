<script lang="ts">
  import { editorState, RESOLUTION_PROFILES } from '../lib/state.svelte';
  import type { ResolutionProfile } from '../lib/types';
  import Tooltip from './Tooltip.svelte';

  let customName = $state('Custom Stretch');
  let customW = $state(1440);
  let customH = $state(1080);
  let customStretched = $state(true);

  let derivedAspectRatio = $derived.by(() => {
    const gcd = (a: number, b: number): number => (b === 0 ? a : gcd(b, a % b));
    const d = gcd(customW, customH);
    if (d > 0) {
      return `${Math.round(customW / d)}:${Math.round(customH / d)}`;
    }
    return `${(customW / customH).toFixed(2)}:1`;
  });

  let parityReport = $derived.by(() => {
    const isEvenW = customW % 2 === 0;
    const isEvenH = customH % 2 === 0;
    if (isEvenW && isEvenH) {
      return 'Even W × Even H: True center lands on pixel grid intersection (0.0 game px = edge border)';
    } else if (!isEvenW && !isEvenH) {
      return 'Odd W × Odd H: True center lands on center of central pixel (0.0 game px = pixel middle)';
    } else {
      return `Mixed Parity (${isEvenW ? 'Even W' : 'Odd W'} × ${isEvenH ? 'Even H' : 'Odd H'}): Asymmetric pixel alignment requires 0.5px subpixel offset compensation (§6.2)`;
    }
  });

  function handleCreateProfile() {
    if (customW < 320 || customH < 240) return;
    editorState.addCustomResolutionProfile(customName, customW, customH, customStretched);
    editorState.closeResolutionModal();
  }

  function selectProfile(p: ResolutionProfile) {
    editorState.selectResolutionProfile(p);
  }
</script>

{#if editorState.resolutionModalOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={() => editorState.closeResolutionModal()}>
    <div class="modal-card" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div class="title-group">
          <strong class="title">Resolution Profiles & Alignment</strong>
          <span class="badge">SPEC §6.3 · MULTI-RES</span>
        </div>
        <button
          class="btn-close"
          onclick={() => editorState.closeResolutionModal()}
          aria-label="Close modal"
        >
          ✕
        </button>
      </div>

      <div class="modal-body">
        <div class="section-desc">
          Crosshairs scale and calibrate per render resolution. Match your in-game resolution to guarantee sub-pixel and pixel-edge alignment.
        </div>

        <!-- Existing Profiles List -->
        <div class="profiles-list">
          <span class="subhead">Configured Profiles:</span>
          {#each RESOLUTION_PROFILES as prof}
            <div
              class="profile-item {prof.id === editorState.activeProfile.id ? 'active' : ''}"
              onclick={() => selectProfile(prof)}
              role="button"
              tabindex="0"
              onkeydown={(e) => {
                if (e.key === 'Enter') selectProfile(prof);
              }}
            >
              <div class="profile-info">
                <div class="prof-name-row">
                  <span class="prof-name">{prof.name}</span>
                  {#if prof.id === editorState.activeProfile.id}
                    <span class="badge" style="background: var(--accent-dim); color: var(--accent);">ACTIVE</span>
                  {/if}
                  {#if prof.isStretched}
                    <span class="badge">STRETCHED</span>
                  {/if}
                </div>
                <div class="prof-meta mono">
                  {prof.width} × {prof.height} · {prof.width % 2 === 0 ? 'Even W' : 'Odd W'}
                  {#if prof.calibrationOffset && (prof.calibrationOffset.x !== 0 || prof.calibrationOffset.y !== 0)}
                    · Calibrated ({prof.calibrationOffset.x > 0 ? '+' : ''}{prof.calibrationOffset.x}, {prof.calibrationOffset.y > 0 ? '+' : ''}{prof.calibrationOffset.y})
                  {/if}
                </div>
              </div>
            </div>
          {/each}
        </div>

        <!-- Add Custom Resolution Form -->
        <div class="custom-form-section">
          <span class="subhead">Add Custom Game Resolution:</span>
          <div class="form-grid">
            <div class="form-field">
              <label for="cr-name">Profile Name</label>
              <input
                id="cr-name"
                type="text"
                class="input-text"
                bind:value={customName}
                placeholder="e.g. CS2 4:3 Stretched"
              />
            </div>

            <div class="form-row">
              <div class="form-field">
                <label for="cr-w">Width (px)</label>
                <input
                  id="cr-w"
                  type="number"
                  class="input-text mono"
                  bind:value={customW}
                  min="320"
                  max="7680"
                  step="1"
                />
              </div>

              <div class="form-field">
                <label for="cr-h">Height (px)</label>
                <input
                  id="cr-h"
                  type="number"
                  class="input-text mono"
                  bind:value={customH}
                  min="240"
                  max="4320"
                  step="1"
                />
              </div>

              <div class="form-field" style="justify-content: flex-end;">
                <label class="checkbox-label">
                  <input type="checkbox" bind:checked={customStretched} />
                  <span>Stretched Aspect Mode</span>
                </label>
              </div>
            </div>
          </div>

          <!-- Alignment & Aspect Ratio Analysis -->
          <div class="analysis-box">
            <div class="analysis-row">
              <span class="analysis-label">Derived Aspect Ratio:</span>
              <strong class="mono" style="color: var(--accent);">{derivedAspectRatio}</strong>
            </div>
            <div class="analysis-row">
              <span class="analysis-label">Grid Parity Analysis:</span>
              <span class="analysis-value">{parityReport}</span>
            </div>
          </div>

          <div class="modal-actions">
            <button
              class="btn btn-secondary"
              onclick={() => editorState.closeResolutionModal()}
            >
              Close
            </button>
            <button
              class="btn btn-primary"
              onclick={handleCreateProfile}
            >
              + Add & Activate Profile
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(8, 10, 12, 0.78);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-card {
    background: #12161B;
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    width: 560px;
    max-width: 90vw;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.6);
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    background: rgba(18, 22, 27, 0.6);
  }

  .title-group {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .title {
    font-size: 15px;
    color: var(--text);
  }

  .btn-close {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 16px;
    cursor: pointer;
    padding: 4px;
    border-radius: var(--radius-sm);
  }

  .btn-close:hover {
    color: var(--text);
  }

  .modal-body {
    padding: 20px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .section-desc {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
  }

  .subhead {
    font-size: 12px;
    font-weight: 600;
    color: var(--text);
    margin-bottom: 6px;
    display: block;
  }

  .profiles-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .profile-item {
    background: #090B0E;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 10px 14px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .profile-item:hover {
    border-color: rgba(54, 242, 203, 0.4);
  }

  .profile-item.active {
    border-color: var(--accent);
    background: rgba(54, 242, 203, 0.05);
  }

  .prof-name-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .prof-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
  }

  .prof-meta {
    font-size: 11px;
    color: var(--text-muted);
    margin-top: 3px;
  }

  .custom-form-section {
    border-top: 1px solid var(--border);
    padding-top: 14px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .form-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .form-row {
    display: flex;
    gap: 12px;
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
  }

  .form-field label {
    font-size: 11px;
    color: var(--text-muted);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text);
    cursor: pointer;
    margin-top: 16px;
  }

  .analysis-box {
    background: #090B0E;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 10px 14px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .analysis-row {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    font-size: 11px;
  }

  .analysis-label {
    color: var(--text-muted);
    white-space: nowrap;
  }

  .analysis-value {
    color: var(--text);
    line-height: 1.4;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 10px;
  }
</style>
