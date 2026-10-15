<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import type { GunProfile, SprayShotImpact } from '../lib/types';
  import Tooltip from './Tooltip.svelte';

  let selectedProfile = $state<GunProfile>(editorState.activeGunProfile);
  let scrubbedShotIndex = $state<number>(0);
  let isFiring = $state<boolean>(false);
  let fireInterval: ReturnType<typeof setInterval> | null = null;
  let canvasRef: HTMLCanvasElement | null = null;

  function close() {
    stopFiring();
    editorState.closeSprayLab();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      close();
    }
  }

  function selectProfile(profile: GunProfile) {
    selectedProfile = profile;
    scrubbedShotIndex = 0;
    stopFiring();
  }

  function toggleFiring() {
    if (isFiring) {
      stopFiring();
    } else {
      startFiring();
    }
  }

  function startFiring() {
    isFiring = true;
    scrubbedShotIndex = 0;
    const intervalMs = 1000 / (selectedProfile.fire_rate_hz || 10);

    fireInterval = setInterval(() => {
      if (scrubbedShotIndex + 1 < selectedProfile.magazine) {
        scrubbedShotIndex += 1;
      } else {
        stopFiring();
      }
    }, intervalMs);
  }

  function stopFiring() {
    isFiring = false;
    if (fireInterval) {
      clearInterval(fireInterval);
      fireInterval = null;
    }
  }

  // Calculate current shot metrics
  let currentRecoil = $derived.by<{ x: number; y: number }>(() => {
    if (scrubbedShotIndex === 0 || !selectedProfile.pattern.length) {
      return { x: 0, y: 0 };
    }
    const idx = Math.min(scrubbedShotIndex - 1, selectedProfile.pattern.length - 1);
    return selectedProfile.pattern[idx] || { x: 0, y: 0 };
  });

  let currentSpread = $derived.by<number>(() => {
    const rawSpread = selectedProfile.spread.base + scrubbedShotIndex * selectedProfile.spread.per_shot;
    return Math.min(selectedProfile.spread.max, rawSpread);
  });

  let dynGap = $derived(4 + (currentSpread - 1.2) * 1.5);

  // Calculate all shot impacts for the profile pattern
  let simulatedShots = $derived.by<SprayShotImpact[]>(() => {
    const shots: SprayShotImpact[] = [];
    const interval = 1.0 / (selectedProfile.fire_rate_hz || 10);
    const mag = selectedProfile.magazine;

    for (let i = 0; i < mag; i++) {
      const offset = i < selectedProfile.pattern.length
        ? selectedProfile.pattern[i]
        : selectedProfile.pattern.length > 0
          ? selectedProfile.pattern[selectedProfile.pattern.length - 1]
          : { x: 0, y: i * 2 };

      const spread = Math.min(selectedProfile.spread.max, selectedProfile.spread.base + i * selectedProfile.spread.per_shot);
      shots.push({
        shot_index: i,
        position: offset,
        timestamp_seconds: i * interval,
        spread_radius: spread,
      });
    }
    return shots;
  });

  function exportPatternAsPng() {
    const canvas = document.createElement('canvas');
    canvas.width = 600;
    canvas.height = 600;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Plain dark backdrop
    ctx.fillStyle = '#0e1215';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Grid
    ctx.strokeStyle = '#1d242a';
    ctx.lineWidth = 1;
    for (let x = 0; x <= 600; x += 40) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, 600);
      ctx.stroke();
    }
    for (let y = 0; y <= 600; y += 40) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(600, y);
      ctx.stroke();
    }

    const centerX = 300;
    const centerY = 450;
    const scale = 8.0;

    // Draw trajectory lines
    ctx.strokeStyle = '#36f2cb';
    ctx.lineWidth = 2;
    ctx.beginPath();
    simulatedShots.forEach((shot, idx) => {
      const px = centerX + shot.position.x * scale;
      const py = centerY - shot.position.y * scale;
      if (idx === 0) ctx.moveTo(px, py);
      else ctx.lineTo(px, py);
    });
    ctx.stroke();

    // Draw impact points & spread error discs
    simulatedShots.forEach((shot) => {
      const px = centerX + shot.position.x * scale;
      const py = centerY - shot.position.y * scale;

      // Error disc
      ctx.fillStyle = 'rgba(54, 242, 203, 0.15)';
      ctx.beginPath();
      ctx.arc(px, py, shot.spread_radius * scale * 0.4, 0, Math.PI * 2);
      ctx.fill();

      // Dot
      ctx.fillStyle = '#ffffff';
      ctx.beginPath();
      ctx.arc(px, py, 3, 0, Math.PI * 2);
      ctx.fill();
    });

    // Title text
    ctx.fillStyle = '#ffffff';
    ctx.font = 'bold 16px sans-serif';
    ctx.fillText(`${selectedProfile.name} - Spray Pattern`, 20, 30);
    ctx.fillStyle = '#8e9aa5';
    ctx.font = '12px sans-serif';
    ctx.fillText(`Simulated Pattern · ${simulatedShots.length} shots · ${selectedProfile.fire_rate_hz} Hz`, 20, 50);

    const a = document.createElement('a');
    a.download = `${selectedProfile.id}-spray-pattern.png`;
    a.href = canvas.toDataURL('image/png');
    a.click();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if editorState.sprayLabOpen}
  <div class="spray-lab-overlay" role="dialog" aria-modal="true" aria-labelledby="spray-lab-title">
    <header class="lab-header">
      <div class="header-left">
        <span class="badge-safe">STANDALONE LAB · 100% SAFE</span>
        <h2 id="spray-lab-title">Spray Lab & Recoil Curve Visualizer (§12.6)</h2>
      </div>

      <div class="header-actions">
        <div class="profile-tabs">
          {#each editorState.gunProfiles as profile}
            <button
              class="tab-btn {selectedProfile.id === profile.id ? 'active' : ''}"
              onclick={() => selectProfile(profile)}
            >
              {profile.name}
            </button>
          {/each}
        </div>

        <button class="btn btn-secondary" onclick={exportPatternAsPng}>
          📷 Export PNG
        </button>

        <button class="close-btn" onclick={close} aria-label="Close Spray Lab">✕</button>
      </div>
    </header>

    <div class="lab-body">
      <!-- Left: Simulation Canvas (Center Stage) -->
      <main class="sim-viewport">
        <div class="canvas-container">
          <svg class="spray-svg" viewBox="0 0 600 600">
            <!-- Background Grid -->
            <defs>
              <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
                <path d="M 40 0 L 0 0 0 40" fill="none" stroke="#182026" stroke-width="1" />
              </pattern>
            </defs>
            <rect width="600" height="600" fill="#0b0f12" />
            <rect width="600" height="600" fill="url(#grid)" />

            <!-- Center Origin Crosshair Axis -->
            <line x1="300" y1="0" x2="300" y2="600" stroke="#1d2830" stroke-width="1" stroke-dasharray="4 4" />
            <line x1="0" y1="450" x2="600" y2="450" stroke="#1d2830" stroke-width="1" stroke-dasharray="4 4" />

            <!-- Trajectory Path connecting shots up to scrubbed index -->
            {#if simulatedShots.length > 0}
              {@const pts = simulatedShots
                .slice(0, scrubbedShotIndex + 1)
                .map((s) => `${300 + s.position.x * 8},${450 - s.position.y * 8}`)
                .join(' ')}
              <polyline points={pts} fill="none" stroke="#36f2cb" stroke-width="2" opacity="0.85" />
            {/if}

            <!-- All Planned Shot Guides (Faint) -->
            {#each simulatedShots as shot, idx}
              {@const px = 300 + shot.position.x * 8}
              {@const py = 450 - shot.position.y * 8}
              <circle cx={px} cy={py} r="2" fill="#243440" />
            {/each}

            <!-- Active / Fired Shots -->
            {#each simulatedShots.slice(0, scrubbedShotIndex + 1) as shot, idx}
              {@const px = 300 + shot.position.x * 8}
              {@const py = 450 - shot.position.y * 8}
              {@const isCurrent = idx === scrubbedShotIndex}

              <!-- Spread error radius disc -->
              <circle
                cx={px}
                cy={py}
                r={shot.spread_radius * 3.2}
                fill={isCurrent ? 'rgba(54, 242, 203, 0.25)' : 'rgba(54, 242, 203, 0.08)'}
                stroke={isCurrent ? '#36f2cb' : 'rgba(54, 242, 203, 0.2)'}
                stroke-width="1"
              />

              <!-- Impact point dot -->
              <circle
                cx={px}
                cy={py}
                r={isCurrent ? 5 : 3}
                fill={isCurrent ? '#ffffff' : '#36f2cb'}
                stroke="#0b0f12"
                stroke-width="1.5"
              />

              <!-- Shot sequence number label -->
              {#if idx % 3 === 0 || isCurrent}
                <text
                  x={px + 8}
                  y={py - 6}
                  fill={isCurrent ? '#36f2cb' : '#8e9aa5'}
                  font-size="10"
                  font-family="monospace"
                >
                  #{idx + 1}
                </text>
              {/if}
            {/each}

            <!-- Dynamic Crosshair Response Simulation (Center Preview) -->
            <g transform="translate(300, 450)">
              <!-- Dynamic reticle position reacts to recoil climb and spread gap expansion -->
              <g transform="translate({currentRecoil.x * 8}, {-currentRecoil.y * 8})">
                <!-- Top arm -->
                <line x1="0" y1={-dynGap} x2="0" y2={-dynGap - 8} stroke="#36f2cb" stroke-width="2" />
                <!-- Bottom arm -->
                <line x1="0" y1={dynGap} x2="0" y2={dynGap + 8} stroke="#36f2cb" stroke-width="2" />
                <!-- Left arm -->
                <line x1={-dynGap} y1="0" x2={-dynGap - 8} y2="0" stroke="#36f2cb" stroke-width="2" />
                <!-- Right arm -->
                <line x1={dynGap} y1="0" x2={dynGap + 8} y2="0" stroke="#36f2cb" stroke-width="2" />
                <!-- Center dot -->
                <rect x="-1" y="-1" width="2" height="2" fill="#36f2cb" />
              </g>
            </g>
          </svg>
        </div>

        <!-- Simulation Scrubbing Bar -->
        <div class="playback-controls">
          <button class="btn btn-primary play-btn" onclick={toggleFiring}>
            {isFiring ? '⏹ Stop' : '▶ Fire Burst'}
          </button>

          <div class="scrub-container">
            <span class="scrub-label mono">
              Shot {scrubbedShotIndex + 1} / {selectedProfile.magazine}
            </span>
            <input
              type="range"
              min="0"
              max={selectedProfile.magazine - 1}
              bind:value={scrubbedShotIndex}
              class="scrub-slider"
            />
            <span class="time-label mono">
              {((scrubbedShotIndex / (selectedProfile.fire_rate_hz || 10))).toFixed(2)}s
            </span>
          </div>
        </div>
      </main>

      <!-- Right: Gun Profile Inspector & Custom Spread Parameters -->
      <aside class="lab-sidebar">
        <div class="sidebar-header">
          <h3>Weapon Profile Parameters</h3>
          <span class="profile-badge">{selectedProfile.id}</span>
        </div>

        <div class="metrics-row">
          <div class="metric-card">
            <span class="metric-title">Active Spread</span>
            <span class="metric-val highlight">{currentSpread.toFixed(1)} px</span>
          </div>
          <div class="metric-card">
            <span class="metric-title">Recoil Kick</span>
            <span class="metric-val mono">{currentRecoil.x.toFixed(1)}, {currentRecoil.y.toFixed(1)}</span>
          </div>
          <div class="metric-card">
            <span class="metric-title">Fire Rate</span>
            <span class="metric-val">{selectedProfile.fire_rate_hz} Hz</span>
          </div>
        </div>

        <div class="param-group">
          <div class="group-title">SPREAD DYNAMICS</div>

          <div class="field-row">
            <span class="field-label">Base Standing Spread</span>
            <span class="field-val mono">{selectedProfile.spread.base} px</span>
          </div>
          <div class="field-row">
            <span class="field-label">Per-Shot Increase</span>
            <span class="field-val mono">+{selectedProfile.spread.per_shot} px</span>
          </div>
          <div class="field-row">
            <span class="field-label">Max Spread Ceiling</span>
            <span class="field-val mono">{selectedProfile.spread.max} px</span>
          </div>
          <div class="field-row">
            <span class="field-label">Recovery Rate</span>
            <span class="field-val mono">{selectedProfile.spread.recovery_per_sec} px/s</span>
          </div>
          <div class="field-row">
            <span class="field-label">Walk Penalty</span>
            <span class="field-val mono">+{selectedProfile.spread.move_penalty} px</span>
          </div>
          <div class="field-row">
            <span class="field-label">Shift-Walk Penalty</span>
            <span class="field-val mono">+{selectedProfile.spread.shift_move_penalty} px</span>
          </div>
          <div class="field-row">
            <span class="field-label">Crouch Reduction</span>
            <span class="field-val mono">-{selectedProfile.spread.crouch_bonus} px</span>
          </div>
        </div>

        <div class="param-group">
          <div class="group-title">TEMPORAL SMOOTHING (§12.4)</div>
          <div class="field-row">
            <span class="field-label">Attack Smoothing</span>
            <span class="field-val mono">{selectedProfile.smoothing.attack_ms} ms</span>
          </div>
          <div class="field-row">
            <span class="field-label">Release Smoothing</span>
            <span class="field-val mono">{selectedProfile.smoothing.release_ms} ms</span>
          </div>
        </div>

        <div class="disclaimer-card">
          <div class="disclaimer-title">ℹ️ Example Data Disclaimer</div>
          <p>
            Values above are neutral simulation examples and not official game recoil data.
            Users may enter custom curves per weapon for offline muscle memory training (§12.4).
          </p>
        </div>
      </aside>
    </div>
  </div>
{/if}

<style>
  .spray-lab-overlay {
    position: fixed;
    inset: 0;
    background: var(--bg);
    display: flex;
    flex-direction: column;
    z-index: 1000;
  }

  .lab-header {
    height: 54px;
    background: var(--surface1);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 20px;
    flex: none;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .badge-safe {
    background: rgba(54, 242, 203, 0.15);
    color: var(--accent);
    border: 1px solid rgba(54, 242, 203, 0.35);
    font-size: 10px;
    font-weight: 700;
    padding: 3px 8px;
    border-radius: 4px;
    letter-spacing: 0.06em;
  }

  h2 {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
    margin: 0;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .profile-tabs {
    display: flex;
    background: var(--surface2);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 2px;
    gap: 2px;
  }

  .tab-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 12px;
    padding: 6px 12px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .tab-btn:hover {
    color: var(--text);
  }

  .tab-btn.active {
    background: var(--surface1);
    color: var(--accent);
    font-weight: 600;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.4);
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 16px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
  }

  .close-btn:hover {
    color: var(--text);
    background: rgba(255, 255, 255, 0.05);
  }

  .lab-body {
    flex: 1;
    display: flex;
    min-height: 0;
  }

  .sim-viewport {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: #07090b;
    position: relative;
    border-right: 1px solid var(--border);
  }

  .canvas-container {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
    overflow: hidden;
  }

  .spray-svg {
    width: 600px;
    height: 600px;
    max-width: 100%;
    max-height: 100%;
    border-radius: 8px;
    border: 1px solid var(--border);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.6);
  }

  .playback-controls {
    height: 64px;
    background: var(--surface1);
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    padding: 0 24px;
    gap: 20px;
    flex: none;
  }

  .play-btn {
    min-width: 110px;
  }

  .scrub-container {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .scrub-label {
    font-size: 12px;
    color: var(--text);
    min-width: 110px;
  }

  .time-label {
    font-size: 12px;
    color: var(--text-muted);
    min-width: 50px;
    text-align: right;
  }

  .scrub-slider {
    flex: 1;
    accent-color: var(--accent);
    cursor: pointer;
  }

  .lab-sidebar {
    width: 360px;
    flex: none;
    background: var(--surface1);
    display: flex;
    flex-direction: column;
    padding: 20px;
    gap: 16px;
    overflow-y: auto;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .sidebar-header h3 {
    font-size: 13px;
    font-weight: 700;
    color: var(--text);
    margin: 0;
  }

  .profile-badge {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-muted);
  }

  .metrics-row {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }

  .metric-card {
    background: var(--surface2);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 8px 10px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .metric-title {
    font-size: 10px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .metric-val {
    font-size: 13px;
    font-weight: 700;
    color: var(--text);
  }

  .metric-val.highlight {
    color: var(--accent);
  }

  .param-group {
    background: var(--surface2);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .group-title {
    font-size: 10px;
    font-weight: 700;
    color: var(--text-muted);
    letter-spacing: 0.05em;
  }

  .field-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
  }

  .field-label {
    color: var(--text);
  }

  .field-val {
    color: var(--text-muted);
    font-size: 11px;
  }

  .disclaimer-card {
    background: rgba(255, 255, 255, 0.02);
    border: 1px dashed var(--border);
    border-radius: 8px;
    padding: 12px;
  }

  .disclaimer-title {
    font-size: 11px;
    font-weight: 700;
    color: var(--text);
    margin-bottom: 4px;
  }

  .disclaimer-card p {
    font-size: 11px;
    color: var(--text-muted);
    line-height: 1.45;
    margin: 0;
  }

  .btn {
    height: 32px;
    padding: 0 14px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.15s ease;
  }

  .btn-secondary {
    background: transparent;
    border-color: var(--border);
    color: var(--text);
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .btn-primary {
    background: var(--accent);
    color: #0b1414;
  }

  .btn-primary:hover {
    filter: brightness(1.1);
  }

  .mono {
    font-family: var(--font-mono);
  }
</style>
