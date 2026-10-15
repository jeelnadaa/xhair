<script lang="ts">
  import { editorState, RESOLUTION_PROFILES } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';
  import type { AlignmentIssue } from '../lib/types';

  let fileInput = $state<HTMLInputElement | null>(null);
  let screenshotPreviewUrl = $state<string | null>(null);
  let isAnalyzing = $state(false);

  function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const file = input.files[0];
    analyzeScreenshotFile(file);
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    if (!e.dataTransfer?.files || e.dataTransfer.files.length === 0) return;
    const file = e.dataTransfer.files[0];
    analyzeScreenshotFile(file);
  }

  function analyzeScreenshotFile(file: File) {
    isAnalyzing = true;
    const reader = new FileReader();
    reader.onload = (e) => {
      const dataUrl = e.target?.result as string;
      screenshotPreviewUrl = dataUrl;

      const img = new Image();
      img.onload = () => {
        // Run offline symmetry analysis on HTML5 canvas
        const canvas = document.createElement('canvas');
        canvas.width = img.width;
        canvas.height = img.height;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
          isAnalyzing = false;
          return;
        }

        ctx.drawImage(img, 0, 0);
        const imgData = ctx.getImageData(0, 0, img.width, img.height);
        const data = imgData.data;

        // Convert to luminance
        const lum = new Uint8Array(img.width * img.height);
        for (let i = 0; i < lum.length; i++) {
          const r = data[i * 4];
          const g = data[i * 4 + 1];
          const b = data[i * 4 + 2];
          lum[i] = Math.round(0.2126 * r + 0.7152 * g + 0.0722 * b);
        }

        // Symmetry center detection around theoretical center
        const tcX = img.width / 2;
        const tcY = img.height / 2;
        const searchR = 32;
        const sampleR = 16;

        let bestScore = -1;
        let bestCx = tcX;
        let bestCy = tcY;
        let maxEnergy = 0;

        for (let sy = -searchR * 2; sy <= searchR * 2; sy++) {
          const cy = tcY + sy * 0.5;
          if (cy < sampleR || cy >= img.height - sampleR) continue;

          for (let sx = -searchR * 2; sx <= searchR * 2; sx++) {
            const cx = tcX + sx * 0.5;
            if (cx < sampleR || cx >= img.width - sampleR) continue;

            let totalDiff = 0;
            let totalEnergy = 0;
            let count = 0;

            for (let dy = 1; dy <= sampleR; dy++) {
              for (let dx = 1; dx <= sampleR; dx++) {
                const ix_pp = Math.min(img.width - 1, Math.max(0, Math.round(cx + dx)));
                const iy_pp = Math.min(img.height - 1, Math.max(0, Math.round(cy + dy)));
                const ix_np = Math.min(img.width - 1, Math.max(0, Math.round(cx - dx)));
                const iy_np = iy_pp;
                const ix_pn = ix_pp;
                const iy_pn = Math.min(img.height - 1, Math.max(0, Math.round(cy - dy)));
                const ix_nn = ix_np;
                const iy_nn = iy_pn;

                const p_pp = lum[iy_pp * img.width + ix_pp];
                const p_np = lum[iy_np * img.width + ix_np];
                const p_pn = lum[iy_pn * img.width + ix_pn];
                const p_nn = lum[iy_nn * img.width + ix_nn];

                totalDiff += Math.abs(p_pp - p_np) + Math.abs(p_pn - p_nn) + Math.abs(p_pp - p_pn) + Math.abs(p_np - p_nn);
                totalEnergy += p_pp + p_np + p_pn + p_nn;
                count += 4;
              }
            }

            if (count > 0) {
              const symFactor = Math.max(0, 1 - totalDiff / (totalEnergy + 0.001));
              const score = totalEnergy * symFactor * symFactor;
              if (score > bestScore) {
                bestScore = score;
                bestCx = cx;
                bestCy = cy;
                maxEnergy = totalEnergy;
              }
            }
          }
        }

        const rawOffsetX = bestCx - tcX;
        const rawOffsetY = bestCy - tcY;
        const qx = Math.round(rawOffsetX * 4) / 4;
        const qy = Math.round(rawOffsetY * 4) / 4;
        const confidence = maxEnergy > 20 ? Math.min(1.0, Math.round((bestScore / (maxEnergy + 1)) * 100) / 100) : 0.2;

        editorState.setScreenshotAlignResult({ x: qx, y: qy }, confidence);
        isAnalyzing = false;
      };
      img.src = dataUrl;
    };
    reader.readAsDataURL(file);
  }
</script>

{#if editorState.calibrationOpen}
  <div class="modal-backdrop" onclick={(e) => e.target === e.currentTarget && editorState.closeCalibration()} role="presentation">
    <div class="calibration-dialog" role="dialog" aria-modal="true" aria-labelledby="cal-title">
      <!-- Top Bar Header -->
      <header class="dialog-header">
        <div class="header-left">
          <strong id="cal-title" class="brand-title">XHAIR</strong>
          <span class="divider">/</span>
          <span class="section-name">Screen Center Calibration (§6.7)</span>
        </div>

        <div class="header-center">
          <div class="tab-pill-group">
            <button
              class="tab-btn {!editorState.screenshotAlignMode ? 'active' : ''}"
              onclick={() => (editorState.screenshotAlignMode = false)}
            >
              4-Step Wizard
            </button>
            <button
              class="tab-btn {editorState.screenshotAlignMode ? 'active' : ''}"
              onclick={() => (editorState.screenshotAlignMode = true)}
            >
              Screenshot Align (Offline)
            </button>
          </div>
        </div>

        <div class="header-right">
          <span class="status-pill {editorState.calibrationPatternActive ? 'active' : ''}">
            Pattern: {editorState.calibrationPatternActive ? 'ACTIVE (F8)' : 'OFF'}
          </span>
          <span class="status-pill">
            {editorState.activeProfile.name}
          </span>
          <button
            class="close-btn"
            onclick={() => editorState.closeCalibration()}
            aria-label="Close Calibration Dialog"
          >
            ✕
          </button>
        </div>
      </header>

      <!-- Main Body -->
      <div class="dialog-body">
        <!-- Left Sidebar: Stepper or File Upload -->
        <aside class="sidebar-left">
          {#if !editorState.screenshotAlignMode}
            <div class="sidebar-section-title">CALIBRATION WIZARD</div>

            <div class="stepper">
              <button
                class="step-card {editorState.calibrationStep === 1 ? 'selected' : ''}"
                onclick={() => editorState.setCalibrationStep(1)}
              >
                <div class="step-label">1. Pick Profile</div>
                <div class="step-desc">Target game resolution & display mode</div>
              </button>

              <button
                class="step-card {editorState.calibrationStep === 2 ? 'selected' : ''}"
                onclick={() => editorState.setCalibrationStep(2)}
              >
                <div class="step-label">2. Test Pattern</div>
                <div class="step-desc">Show 1-px cross with ±5/10/20 px ticks</div>
              </button>

              <button
                class="step-card {editorState.calibrationStep === 3 ? 'selected' : ''}"
                onclick={() => editorState.setCalibrationStep(3)}
              >
                <div class="step-label">3. Align with Game</div>
                <div class="step-desc">Compare with 1-px dot in practice range</div>
              </button>

              <button
                class="step-card {editorState.calibrationStep === 4 ? 'selected' : ''}"
                onclick={() => editorState.setCalibrationStep(4)}
              >
                <div class="step-label">4. Save & Verify</div>
                <div class="step-desc">Store sub-pixel offset into profile</div>
              </button>
            </div>

            <div class="instruction-box">
              <p>
                <strong>How to align:</strong> Set your in-game reticle to a 1-px dot or line in the practice range. Press <code>F8</code> to toggle test pattern flicker, and use the nudge pad on the right until both crosshairs perfectly overlap.
              </p>
            </div>

            <!-- Profile selector in step 1 -->
            {#if editorState.calibrationStep === 1}
              <div class="profile-select-group">
                <label for="calib-profile-select" class="field-label">Active Profile:</label>
                <select
                  id="calib-profile-select"
                  class="select-input"
                  value={editorState.activeProfile.id}
                  onchange={(e) => {
                    const found = RESOLUTION_PROFILES.find((p) => p.id === (e.target as HTMLSelectElement).value);
                    if (found) editorState.activeProfile = found;
                  }}
                >
                  {#each RESOLUTION_PROFILES as prof}
                    <option value={prof.id}>{prof.name}</option>
                  {/each}
                </select>
              </div>
            {/if}

            <div class="action-btn-row">
              <Tooltip title="Toggle Pattern (F8)" description="Show or hide overlay calibration pattern">
                <button
                  class="btn-pattern {editorState.calibrationPatternActive ? 'active' : ''}"
                  onclick={() => editorState.toggleCalibrationPattern()}
                >
                  {editorState.calibrationPatternActive ? 'Hide Test Pattern' : 'Project Test Pattern'}
                </button>
              </Tooltip>
            </div>
          {:else}
            <!-- Screenshot Align Mode -->
            <div class="sidebar-section-title">SCREENSHOT ALIGN (§6.7)</div>

            <div class="screenshot-info">
              <p>
                Import a screenshot <em>you took yourself</em> of the game's crosshair. The offline symmetry analyzer will detect the reticle center and calculate the offset.
              </p>
              <div class="compliance-badge">
                🔒 Invariant P1: 100% offline local image analysis. Zero screen capture.
              </div>
            </div>

            <input
              type="file"
              accept="image/png,image/jpeg"
              bind:this={fileInput}
              onchange={handleFileSelect}
              style="display: none;"
            />

            <!-- Drag and drop zone -->
            <div
              class="dropzone"
              ondrop={handleDrop}
              ondragover={(e) => e.preventDefault()}
              onclick={() => fileInput?.click()}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === 'Enter' && fileInput?.click()}
            >
              <span class="drop-icon">📷</span>
              <span class="drop-title">Drop Screenshot Here</span>
              <span class="drop-sub">or click to browse local PNG / JPG</span>
            </div>

            {#if isAnalyzing}
              <div class="analyzing-spinner">
                Analyzing bilateral symmetry...
              </div>
            {/if}

            {#if editorState.screenshotDetectedOffset}
              <div class="detection-result-card">
                <div class="result-title">Symmetry Detected</div>
                <div class="result-row">
                  <span>Offset X:</span>
                  <b>{editorState.screenshotDetectedOffset.x >= 0 ? `+${editorState.screenshotDetectedOffset.x.toFixed(2)}` : editorState.screenshotDetectedOffset.x.toFixed(2)} px</b>
                </div>
                <div class="result-row">
                  <span>Offset Y:</span>
                  <b>{editorState.screenshotDetectedOffset.y >= 0 ? `+${editorState.screenshotDetectedOffset.y.toFixed(2)}` : editorState.screenshotDetectedOffset.y.toFixed(2)} px</b>
                </div>
                <div class="result-row">
                  <span>Confidence:</span>
                  <b>{(editorState.screenshotConfidence * 100).toFixed(0)}%</b>
                </div>

                <button class="btn-primary" onclick={() => editorState.applyScreenshotAlignOffset()}>
                  Apply Detected Offset
                </button>
              </div>
            {/if}
          {/if}
        </aside>

        <!-- Center Canvas: Test Pattern or Screenshot View -->
        <main class="center-stage">
          {#if !editorState.screenshotAlignMode || !screenshotPreviewUrl}
            <!-- Test Pattern Canvas -->
            <svg
              class="test-pattern-svg"
              viewBox="0 0 840 680"
              role="img"
              aria-label="Calibration Test Pattern Grid"
            >
              <defs>
                <pattern id="cal-grid" width="24" height="24" patternUnits="userSpaceOnUse" x="12" y="12">
                  <path d="M24 0V24M0 24H24" stroke="#1B232C" fill="none" stroke-width="1" />
                </pattern>
              </defs>

              <rect width="840" height="680" fill="#0E1318" />
              <rect width="840" height="680" fill="url(#cal-grid)" />

              <!-- Crosshair Center Lines -->
              <line x1="0" y1="340" x2="840" y2="340" stroke="#FFFFFF" stroke-width="1.5" />
              <line x1="420" y1="0" x2="420" y2="680" stroke="#FFFFFF" stroke-width="1.5" />

              <!-- Concentric Rings at 10, 20, 40 px (scaled ×8) -->
              <circle cx="420" cy="340" r="40" fill="none" stroke="rgba(54, 242, 203, 0.4)" stroke-width="1" stroke-dasharray="2,2" />
              <circle cx="420" cy="340" r="80" fill="none" stroke="rgba(54, 242, 203, 0.4)" stroke-width="1" stroke-dasharray="3,3" />
              <circle cx="420" cy="340" r="160" fill="none" stroke="rgba(54, 242, 203, 0.25)" stroke-width="1" />

              <!-- Center Edge / Pixel Marker -->
              <circle
                cx={420 + editorState.calibrationOffset.x * 8}
                cy={340 + editorState.calibrationOffset.y * 8}
                r="6"
                fill="none"
                stroke="#FF4FD8"
                stroke-width="2"
              />
              <circle
                cx={420 + editorState.calibrationOffset.x * 8}
                cy={340 + editorState.calibrationOffset.y * 8}
                r="1.5"
                fill="#FF4FD8"
              />

              <!-- Distance Ticks across horizontal axis -->
              {#each [-20, -15, -10, -5, 5, 10, 15, 20] as tick}
                <line
                  x1={420 + tick * 8}
                  y1={334}
                  x2={420 + tick * 8}
                  y2={346}
                  stroke="#8A96A5"
                  stroke-width={tick % 10 === 0 ? "2" : "1"}
                />
                <text
                  x={420 + tick * 8}
                  y="360"
                  font-size="9"
                  font-family="monospace"
                  fill="#717E93"
                  text-anchor="middle"
                >
                  {tick > 0 ? `+${tick}` : tick}
                </text>
              {/each}

              <!-- Distance Ticks across vertical axis -->
              {#each [-20, -10, 10, 20] as tick}
                <line
                  x1={414}
                  y1={340 + tick * 8}
                  x2={426}
                  y2={340 + tick * 8}
                  stroke="#8A96A5"
                  stroke-width="1.5"
                />
              {/each}
            </svg>
          {:else}
            <!-- Uploaded Screenshot Inspection -->
            <div class="screenshot-stage">
              <img src={screenshotPreviewUrl} alt="Imported Game Screenshot" class="screenshot-img" />
              <div
                class="screenshot-detected-marker"
                style="
                  left: calc(50% + {editorState.calibrationOffset.x * 4}px);
                  top: calc(50% + {editorState.calibrationOffset.y * 4}px);
                "
              >
                <div class="reticle-target-cross"></div>
              </div>
            </div>
          {/if}
        </main>

        <!-- Right Sidebar: Nudge Pad & Diagnostic Issues -->
        <aside class="sidebar-right">
          <!-- Fine Nudge Controls -->
          <div class="panel-box">
            <div class="panel-title">NUDGE OFFSET</div>
            <div class="nudge-grid">
              <button class="nudge-btn" onclick={() => editorState.nudgeCalibration(-editorState.nudgeStep, -editorState.nudgeStep)} aria-label="Nudge ↖">↖</button>
              <button class="nudge-btn" onclick={() => editorState.nudgeCalibration(0, -editorState.nudgeStep)} aria-label="Nudge ↑">↑</button>
              <button class="nudge-btn" onclick={() => editorState.nudgeCalibration(editorState.nudgeStep, -editorState.nudgeStep)} aria-label="Nudge ↗">↗</button>
              <button class="nudge-btn" onclick={() => editorState.nudgeCalibration(-editorState.nudgeStep, 0)} aria-label="Nudge ←">←</button>
              <button class="nudge-btn" onclick={() => editorState.resetCalibrationOffset()} aria-label="Reset offset">•</button>
              <button class="nudge-btn" onclick={() => editorState.nudgeCalibration(editorState.nudgeStep, 0)} aria-label="Nudge →">→</button>
              <button class="nudge-btn" onclick={() => editorState.nudgeCalibration(-editorState.nudgeStep, editorState.nudgeStep)} aria-label="Nudge ↙">↙</button>
              <button class="nudge-btn" onclick={() => editorState.nudgeCalibration(0, editorState.nudgeStep)} aria-label="Nudge ↓">↓</button>
              <button class="nudge-btn" onclick={() => editorState.nudgeCalibration(editorState.nudgeStep, editorState.nudgeStep)} aria-label="Nudge ↘">↘</button>
            </div>

            <!-- Granularity Step Chips -->
            <div class="step-chips-row">
              {#each [0.1, 0.25, 0.5, 1.0] as step}
                <button
                  class="step-chip {editorState.nudgeStep === step ? 'active' : ''}"
                  onclick={() => (editorState.nudgeStep = step)}
                >
                  {step}
                </button>
              {/each}
            </div>
            <div class="frame-hint">0.25 px granularity per §6.7</div>
          </div>

          <!-- Alignment & Offset Result -->
          <div class="panel-box">
            <div class="panel-title">RESULT</div>
            <div class="result-readout">
              <div>Offset X: <b>{editorState.calibrationOffset.x >= 0 ? `+${editorState.calibrationOffset.x.toFixed(2)}` : editorState.calibrationOffset.x.toFixed(2)} px</b></div>
              <div>Offset Y: <b>{editorState.calibrationOffset.y >= 0 ? `+${editorState.calibrationOffset.y.toFixed(2)}` : editorState.calibrationOffset.y.toFixed(2)} px</b></div>
              <div class="res-parity">
                W {editorState.activeProfile.width} ({editorState.alignmentDiagnostic.isEvenWidth ? 'even' : 'odd'}) · H {editorState.activeProfile.height} ({editorState.alignmentDiagnostic.isEvenHeight ? 'even' : 'odd'})
              </div>
            </div>

            <div class="badge-row">
              <span class="badge {editorState.alignmentDiagnostic.isEvenWidth ? 'badge-cyan' : 'badge-amber'}">
                {editorState.alignmentDiagnostic.isEvenWidth ? 'EDGE-CENTERED' : 'PIXEL-CENTERED'}
              </span>
            </div>
          </div>

          <!-- Alignment Issues & 1-Click Fixes (§6.2) -->
          <div class="panel-box issues-box">
            <div class="panel-title">
              ALIGNMENT ISSUES ({editorState.alignmentIssues.length})
            </div>

            {#if editorState.alignmentIssues.length === 0}
              <div class="no-issues">
                ✅ All active layers align with screen pixel edges.
              </div>
            {:else}
              <div class="issues-list">
                {#each editorState.alignmentIssues as issue}
                  <div class="issue-item">
                    <div class="issue-meta">
                      <b>{issue.layer_name}</b>: stroke edge misaligned on {issue.axis}
                    </div>
                    <button
                      class="btn-1click-fix"
                      onclick={() => editorState.apply1ClickAlignmentFix(issue)}
                    >
                      Fix to {issue.suggested_fix.new_thickness}px
                    </button>
                  </div>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Save Button -->
          <button class="btn-primary save-btn" onclick={() => { editorState.saveCalibrationOffset(); editorState.closeCalibration(); }}>
            Save to Profile
          </button>
        </aside>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(4, 7, 10, 0.85);
    backdrop-filter: blur(12px);
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .calibration-dialog {
    width: 1240px;
    height: 760px;
    background: var(--bg, #0B0D10);
    border: 1px solid var(--border, #262E38);
    border-radius: var(--radius-lg, 8px);
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.6);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .dialog-header {
    height: 52px;
    background: var(--surface1, #12161B);
    border-bottom: 1px solid var(--border, #262E38);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 18px;
    flex: none;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .brand-title {
    font-weight: 800;
    letter-spacing: 0.14em;
    color: var(--accent, #36F2CB);
  }

  .divider {
    color: var(--text-dim, #717E93);
  }

  .section-name {
    font-size: 13px;
    color: #E9EEF3;
    font-weight: 600;
  }

  .tab-pill-group {
    display: flex;
    background: var(--surface2, #1A2027);
    border: 1px solid var(--border, #262E38);
    border-radius: 99px;
    padding: 3px;
  }

  .tab-btn {
    background: transparent;
    border: none;
    color: var(--text-dim, #717E93);
    font-size: 11px;
    font-weight: 600;
    padding: 4px 14px;
    border-radius: 99px;
    cursor: pointer;
  }

  .tab-btn.active {
    background: var(--accent, #36F2CB);
    color: #04110E;
    font-weight: 700;
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .status-pill {
    border: 1px solid var(--border, #262E38);
    border-radius: 99px;
    padding: 3px 10px;
    font-size: 11px;
    color: var(--text-dim, #717E93);
  }

  .status-pill.active {
    border-color: #1D6B5C;
    color: var(--accent, #36F2CB);
    font-weight: 600;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-dim, #717E93);
    font-size: 16px;
    cursor: pointer;
    padding: 4px;
  }

  .close-btn:hover {
    color: #fff;
  }

  .dialog-body {
    flex: 1;
    display: flex;
    min-height: 0;
  }

  .sidebar-left {
    width: 300px;
    border-right: 1px solid var(--border, #262E38);
    background: var(--surface1, #12161B);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    overflow-y: auto;
  }

  .sidebar-section-title {
    font-size: 11px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-dim, #717E93);
    font-weight: 700;
  }

  .stepper {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .step-card {
    background: var(--surface2, #1A2027);
    border: 1px solid var(--border, #262E38);
    border-radius: 6px;
    padding: 10px 12px;
    text-align: left;
    cursor: pointer;
    color: #E9EEF3;
  }

  .step-card.selected {
    border-color: var(--accent, #36F2CB);
    background: rgba(54, 242, 203, 0.08);
  }

  .step-label {
    font-weight: 700;
    font-size: 12px;
    margin-bottom: 2px;
  }

  .step-desc {
    font-size: 10px;
    color: var(--text-dim, #717E93);
  }

  .instruction-box {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid var(--border, #262E38);
    border-radius: 6px;
    padding: 10px;
    font-size: 11px;
    line-height: 1.5;
    color: #CBD5E1;
  }

  .instruction-box code {
    background: var(--surface2, #1A2027);
    padding: 1px 4px;
    border-radius: 3px;
    color: var(--accent, #36F2CB);
  }

  .profile-select-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-label {
    font-size: 11px;
    color: var(--text-dim, #717E93);
  }

  .select-input {
    background: var(--surface2, #1A2027);
    border: 1px solid var(--border, #262E38);
    border-radius: 4px;
    color: #fff;
    padding: 6px 8px;
    font-size: 12px;
  }

  .btn-pattern {
    width: 100%;
    padding: 8px;
    border-radius: 6px;
    border: 1px solid var(--border, #262E38);
    background: var(--surface2, #1A2027);
    color: #fff;
    cursor: pointer;
    font-weight: 600;
    font-size: 12px;
  }

  .btn-pattern.active {
    border-color: var(--accent, #36F2CB);
    color: var(--accent, #36F2CB);
    background: rgba(54, 242, 203, 0.12);
  }

  .dropzone {
    border: 2px dashed var(--border, #262E38);
    border-radius: 8px;
    padding: 24px 16px;
    text-align: center;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
  }

  .dropzone:hover {
    border-color: var(--accent, #36F2CB);
    background: rgba(54, 242, 203, 0.04);
  }

  .drop-icon {
    font-size: 28px;
  }

  .drop-title {
    font-weight: 700;
    font-size: 12px;
    color: #fff;
  }

  .drop-sub {
    font-size: 10px;
    color: var(--text-dim, #717E93);
  }

  .compliance-badge {
    font-size: 10px;
    color: #36F2CB;
    background: rgba(54, 242, 203, 0.08);
    border-radius: 4px;
    padding: 4px 6px;
    margin-top: 6px;
    line-height: 1.4;
  }

  .detection-result-card {
    background: var(--surface2, #1A2027);
    border: 1px solid var(--border, #262E38);
    border-radius: 6px;
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .result-title {
    font-size: 11px;
    font-weight: 700;
    color: var(--accent, #36F2CB);
  }

  .result-row {
    display: flex;
    justify-content: space-between;
    font-size: 11px;
    color: #CBD5E1;
  }

  .center-stage {
    flex: 1;
    min-width: 0;
    background: #0E1318;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .test-pattern-svg {
    width: 100%;
    height: 100%;
  }

  .screenshot-stage {
    width: 100%;
    height: 100%;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .screenshot-img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
  }

  .screenshot-detected-marker {
    position: absolute;
    width: 24px;
    height: 24px;
    transform: translate(-50%, -50%);
    pointer-events: none;
  }

  .reticle-target-cross {
    width: 100%;
    height: 100%;
    border: 1px solid #FF4FD8;
    border-radius: 50%;
    box-shadow: 0 0 8px #FF4FD8;
  }

  .sidebar-right {
    width: 280px;
    border-left: 1px solid var(--border, #262E38);
    background: var(--surface1, #12161B);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    overflow-y: auto;
  }

  .panel-box {
    background: var(--surface2, #1A2027);
    border: 1px solid var(--border, #262E38);
    border-radius: 6px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .panel-title {
    font-size: 10px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    font-weight: 700;
    color: var(--text-dim, #717E93);
  }

  .nudge-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 4px;
  }

  .nudge-btn {
    height: 32px;
    background: var(--surface1, #12161B);
    border: 1px solid var(--border, #262E38);
    border-radius: 4px;
    color: #fff;
    cursor: pointer;
    font-size: 13px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .nudge-btn:hover {
    border-color: var(--accent, #36F2CB);
    color: var(--accent, #36F2CB);
  }

  .step-chips-row {
    display: flex;
    gap: 4px;
  }

  .step-chip {
    flex: 1;
    height: 24px;
    background: var(--surface1, #12161B);
    border: 1px solid var(--border, #262E38);
    border-radius: 4px;
    color: var(--text-dim, #717E93);
    font-size: 10px;
    cursor: pointer;
    font-family: monospace;
  }

  .step-chip.active {
    border-color: var(--accent, #36F2CB);
    color: var(--accent, #36F2CB);
    font-weight: 700;
  }

  .frame-hint {
    font-size: 10px;
    color: var(--text-dim, #717E93);
    text-align: center;
  }

  .result-readout {
    font-family: monospace;
    font-size: 11px;
    line-height: 1.6;
    color: #E2E8F0;
  }

  .res-parity {
    font-size: 10px;
    color: var(--text-dim, #717E93);
    margin-top: 4px;
  }

  .badge-cyan {
    background: rgba(54, 242, 203, 0.15);
    color: #36F2CB;
    border: 1px solid rgba(54, 242, 203, 0.3);
    font-size: 10px;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 4px;
    display: inline-block;
  }

  .badge-amber {
    background: rgba(255, 180, 84, 0.15);
    color: #FFB454;
    border: 1px solid rgba(255, 180, 84, 0.3);
    font-size: 10px;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 4px;
    display: inline-block;
  }

  .issues-box {
    max-height: 180px;
    overflow-y: auto;
  }

  .no-issues {
    font-size: 11px;
    color: #36F2CB;
    line-height: 1.4;
  }

  .issues-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .issue-item {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 4px;
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .issue-meta {
    font-size: 10px;
    color: #FFB454;
    line-height: 1.3;
  }

  .btn-1click-fix {
    background: var(--accent, #36F2CB);
    color: #04110E;
    border: none;
    font-size: 10px;
    font-weight: 700;
    padding: 3px 6px;
    border-radius: 3px;
    cursor: pointer;
    align-self: flex-start;
  }

  .btn-primary {
    background: var(--accent, #36F2CB);
    color: #04110E;
    border: none;
    font-weight: 800;
    font-size: 12px;
    padding: 10px;
    border-radius: 6px;
    cursor: pointer;
  }

  .save-btn {
    width: 100%;
    margin-top: auto;
  }
</style>
