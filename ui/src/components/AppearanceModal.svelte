<script lang="ts">
  import { editorState } from '../lib/state.svelte';

  const THEMES = [
    { id: 'mono-dark', name: 'Mono Dark', base: 'Graphite', accent: '#FFFFFF', isDefault: true },
    { id: 'mono-light', name: 'Mono Light', base: 'White', accent: '#0A0A0A' },
    { id: 'oled-black', name: 'OLED Black', base: 'Pure Black', accent: '#FFFFFF' },
    { id: 'graphite-mint', name: 'Graphite Mint', base: 'Graphite', accent: '#3DDBB0' },
    { id: 'graphite-violet', name: 'Graphite Violet', base: 'Graphite', accent: '#A78BFA' },
    { id: 'graphite-amber', name: 'Graphite Amber', base: 'Graphite', accent: '#F5B83D' },
    { id: 'graphite-crimson', name: 'Graphite Crimson', base: 'Graphite', accent: '#FF5A5F' },
    { id: 'light-indigo', name: 'Light Indigo', base: 'White', accent: '#4F46E5' },
    { id: 'high-contrast', name: 'High Contrast', base: 'Pure Black (2px)', accent: '#FFE600' },
  ];

  const FONT_PACKS = [
    { id: 'satoshi', name: 'Satoshi (Default)', display: 'Clash Display', ui: 'Satoshi', mono: 'JetBrains Mono' },
    { id: 'technical', name: 'Technical', display: 'Geist', ui: 'Geist', mono: 'Geist Mono' },
    { id: 'editorial', name: 'Editorial', display: 'Cabinet Grotesk', ui: 'General Sans', mono: 'IBM Plex Mono' },
    { id: 'plex', name: 'Plex', display: 'IBM Plex Sans Condensed', ui: 'IBM Plex Sans', mono: 'IBM Plex Mono' },
    { id: 'space', name: 'Space', display: 'Space Grotesk', ui: 'Space Grotesk', mono: 'Space Mono' },
    { id: 'system', name: 'System Native', display: 'Segoe UI Variable', ui: 'Segoe UI Variable', mono: 'Cascadia Mono' },
  ];

  const CORNER_STYLES: Array<{ id: 'sharp' | 'default' | 'soft'; label: string; desc: string }> = [
    { id: 'sharp', label: 'Sharp', desc: '0 / 1 / 2 / 2 / 3 / 4 px' },
    { id: 'default', label: 'Default', desc: '0 / 2 / 3 / 4 / 6 / 8 px' },
    { id: 'soft', label: 'Soft', desc: '0 / 3 / 4 / 5 / 6 / 8 / 10 px' },
  ];

  let previewedTheme = $state<string | null>(null);

  function handleMouseEnterTheme(themeId: string) {
    previewedTheme = themeId;
    if (typeof document !== 'undefined') {
      const doc = document.documentElement;
      doc.classList.forEach((cls) => {
        if (cls.startsWith('theme-')) doc.classList.remove(cls);
      });
      doc.classList.add(`theme-${themeId}`);
    }
  }

  function handleMouseLeaveTheme() {
    previewedTheme = null;
    editorState.setTheme(editorState.theme);
  }

  function selectTheme(themeId: string) {
    editorState.setTheme(themeId);
    previewedTheme = null;
  }

  let resetSuccess = $state(false);

  function handleResetStorage() {
    if (confirm('Are you sure you want to reset all storage to factory defaults? This clears cached data and restores default presets.')) {
      editorState.resetStorage();
      resetSuccess = true;
      setTimeout(() => (resetSuccess = false), 3000);
    }
  }
</script>

{#if editorState.appearanceModalOpen}
  <!-- Backdrop -->
  <div class="modal-backdrop" onclick={() => editorState.closeAppearanceModal()} role="presentation">
    <!-- Modal Window -->
    <div
      class="modal-window"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby="appearance-title"
    >
      <div class="modal-header">
        <div>
          <h2 id="appearance-title" class="title">Appearance Settings</h2>
          <p class="subtitle">Theme styling, font packs, and corner curvature</p>
        </div>
        <button
          class="btn btn-ghost btn-icon"
          onclick={() => editorState.closeAppearanceModal()}
          aria-label="Close dialog"
        >
          <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
            <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- Section: Themes -->
        <section class="section">
          <div class="section-title-row">
            <h3 class="section-heading">Theme (Neutral Surface Ramps)</h3>
            <span class="badge">Channel Spread ≤ 2</span>
          </div>

          <div class="theme-grid">
            {#each THEMES as t}
              <button
                class="theme-card"
                class:active={editorState.theme === t.id}
                onmouseenter={() => handleMouseEnterTheme(t.id)}
                onmouseleave={handleMouseLeaveTheme}
                onclick={() => selectTheme(t.id)}
              >
                <div class="theme-swatches">
                  <span class="swatch bg" style="background: {t.base === 'White' ? '#FFFFFF' : t.base === 'Pure Black' ? '#000000' : '#0A0A0A'};"></span>
                  <span class="swatch surface" style="background: {t.base === 'White' ? '#F5F5F5' : t.base === 'Pure Black' ? '#0A0A0A' : '#111111'};"></span>
                  <span class="swatch accent" style="background: {t.accent};"></span>
                </div>
                <div class="theme-info">
                  <div class="theme-name">
                    {t.name}
                    {#if t.isDefault}<span class="default-tag">Default</span>{/if}
                  </div>
                  <div class="theme-meta">{t.base} · {t.accent}</div>
                </div>
              </button>
            {/each}
          </div>
        </section>

        <!-- Section: Typography Packs -->
        <section class="section">
          <h3 class="section-heading">Font Pack (Self-Hosted WOFF2)</h3>
          <div class="pack-grid">
            {#each FONT_PACKS as pack}
              <button
                class="pack-card"
                class:active={editorState.fontPack === pack.id}
                onclick={() => editorState.setFontPack(pack.id)}
              >
                <div class="pack-header">
                  <span class="pack-name">{pack.name}</span>
                  <span class="mono micro-text">12.00 px</span>
                </div>
                <div class="specimen-row">
                  <span class="specimen-display">{pack.display}</span>
                  <span class="specimen-mono mono">#38BDF8FF · 1920×1080</span>
                </div>
              </button>
            {/each}
          </div>
        </section>

        <!-- Section: Corner Curvature -->
        <section class="section">
          <h3 class="section-heading">Corner Curvature (Max 10px, Zero Pills)</h3>
          <div class="corner-grid">
            {#each CORNER_STYLES as cs}
              <button
                class="corner-card"
                class:active={editorState.cornerStyle === cs.id}
                onclick={() => editorState.setCornerStyle(cs.id)}
              >
                <div class="corner-preview corner-{cs.id}">
                  <div class="corner-box"></div>
                </div>
                <div class="corner-info">
                  <div class="corner-label">{cs.label}</div>
                  <div class="corner-desc mono">{cs.desc}</div>
                </div>
              </button>
            {/each}
          </div>
        </section>

        <!-- Section: Storage & Diagnostics -->
        <section class="section">
          <h3 class="section-heading">Storage & Diagnostics</h3>
          <div class="storage-box">
            <div class="storage-info">
              <div class="storage-title">Reset Local & Disk Storage</div>
              <div class="storage-desc">Purges corrupt browser cache, resets catalog, and restores clean default presets.</div>
            </div>
            <button class="btn btn-danger" onclick={handleResetStorage}>
              {resetSuccess ? '✓ Cleared' : 'Reset Storage'}
            </button>
          </div>
        </section>
      </div>

      <div class="modal-footer">
        <button class="btn btn-ghost" onclick={() => editorState.closeAppearanceModal()}>Close</button>
        <button class="btn btn-primary" onclick={() => editorState.closeAppearanceModal()}>Apply & Save</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-window {
    width: 680px;
    max-height: 85vh;
    background: var(--surface);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-xl);
    display: flex;
    flex-direction: column;
    box-shadow: var(--shadow-modal);
    overflow: hidden;
  }

  .modal-header {
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .title {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
  }

  .subtitle {
    font-size: 12px;
    color: var(--text-muted);
    margin-top: 2px;
  }

  .modal-body {
    padding: 20px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .section-title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  .section-heading {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
    margin-bottom: 10px;
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }

  .theme-card {
    background: var(--raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 10px;
    cursor: pointer;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 8px;
    transition: border-color 100ms ease, background 100ms ease;
  }

  .theme-card:hover {
    border-color: var(--border-strong);
    background: var(--hover);
  }

  .theme-card.active {
    border-color: var(--accent);
    background: var(--hover);
  }

  .theme-swatches {
    display: flex;
    gap: 4px;
  }

  .swatch {
    width: 18px;
    height: 18px;
    border-radius: var(--radius-xs);
    border: 1px solid var(--border);
  }

  .theme-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--text);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .default-tag {
    font-size: 9px;
    font-weight: 700;
    color: var(--text-dim);
    text-transform: uppercase;
  }

  .theme-meta {
    font-size: 10px;
    color: var(--text-dim);
  }

  .pack-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }

  .pack-card {
    background: var(--raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 12px;
    cursor: pointer;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 6px;
    transition: border-color 100ms ease;
  }

  .pack-card:hover {
    border-color: var(--border-strong);
    background: var(--hover);
  }

  .pack-card.active {
    border-color: var(--accent);
    background: var(--hover);
  }

  .pack-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .pack-name {
    font-size: 12px;
    font-weight: 700;
    color: var(--text);
  }

  .micro-text {
    font-size: 10px;
    color: var(--text-dim);
  }

  .specimen-row {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .specimen-display {
    font-size: 11px;
    color: var(--text-muted);
  }

  .specimen-mono {
    font-size: 10px;
    color: var(--text-dim);
  }

  .corner-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }

  .corner-card {
    background: var(--raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 12px;
    cursor: pointer;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 8px;
    transition: border-color 100ms ease;
  }

  .corner-card:hover {
    border-color: var(--border-strong);
    background: var(--hover);
  }

  .corner-card.active {
    border-color: var(--accent);
    background: var(--hover);
  }

  .corner-preview {
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg);
    border-radius: var(--radius-sm);
  }

  .corner-box {
    width: 48px;
    height: 22px;
    background: var(--surface2);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-md);
  }

  .corner-label {
    font-size: 12px;
    font-weight: 700;
    color: var(--text);
  }

  .corner-desc {
    font-size: 10px;
    color: var(--text-dim);
  }

  .modal-footer {
    padding: 12px 20px;
    border-top: 1px solid var(--border);
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    background: var(--surface1);
  }

  .storage-box {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 12px 16px;
    background: var(--raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
  }

  .storage-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .storage-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
  }

  .storage-desc {
    font-size: 11px;
    color: var(--text-muted);
  }

  .btn-danger {
    background: rgba(255, 59, 48, 0.15);
    color: #ff453a;
    border: 1px solid rgba(255, 59, 48, 0.35);
    padding: 6px 14px;
    font-size: 12px;
    font-weight: 600;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: all 120ms ease;
    white-space: nowrap;
  }

  .btn-danger:hover {
    background: rgba(255, 59, 48, 0.25);
    border-color: rgba(255, 59, 48, 0.6);
  }
</style>
