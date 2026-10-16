<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  let hue = $state<number>(180);
  let saturation = $state<number>(0.8);
  let value = $state<number>(0.9);
  let alpha = $state<number>(1.0);
  let isDraggingSV = $state<boolean>(false);
  let svBoxEl: HTMLDivElement | null = null;
  let activeFormatTab = $state<'HEX' | 'RGB' | 'HSL'>('HEX');

  const colorblindSafeSwatches = [
    { name: 'Sky Blue (Deuteranopia safe)', hex: '#38BDF8' },
    { name: 'Amber Gold (High visibility)', hex: '#F59E0B' },
    { name: 'Vermilion Red', hex: '#EF4444' },
    { name: 'Pure White', hex: '#FFFFFF' },
    { name: 'Teal Blue', hex: '#0D9488' },
    { name: 'Yellow', hex: '#FBBF24' },
    { name: 'Navy Blue', hex: '#1E3A8A' },
    { name: 'Charcoal', hex: '#1F2937' },
  ];

  const presetSwatches = [
    { name: 'Cyan (Default)', hex: '#36F2CB' },
    { name: 'Neon Green', hex: '#00FF66' },
    { name: 'Vibrant Yellow', hex: '#FFE600' },
    { name: 'Pure White', hex: '#FFFFFF' },
    { name: 'Hot Pink', hex: '#FF4FD8' },
    { name: 'Crimson Red', hex: '#FF3366' },
    { name: 'Electric Blue', hex: '#00D4FF' },
    { name: 'Stealth Black', hex: '#000000' },
  ];

  function hsvToRgb(h: number, s: number, v: number): [number, number, number] {
    const c = v * s;
    const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
    const m = v - c;
    let r = 0, g = 0, b = 0;
    if (h >= 0 && h < 60) { r = c; g = x; b = 0; }
    else if (h >= 60 && h < 120) { r = x; g = c; b = 0; }
    else if (h >= 120 && h < 180) { r = 0; g = c; b = x; }
    else if (h >= 180 && h < 240) { r = 0; g = x; b = c; }
    else if (h >= 240 && h < 300) { r = x; g = 0; b = c; }
    else { r = c; g = 0; b = x; }
    return [Math.round((r + m) * 255), Math.round((g + m) * 255), Math.round((b + m) * 255)];
  }

  function rgbToHex(r: number, g: number, b: number, a: number): string {
    const toHex = (n: number) => n.toString(16).padStart(2, '0').toUpperCase();
    if (a < 0.999) {
      const aHex = Math.round(a * 255).toString(16).padStart(2, '0').toUpperCase();
      return `#${toHex(r)}${toHex(g)}${toHex(b)}${aHex}`;
    }
    return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
  }

  function parseHex(hexStr: string) {
    let raw = hexStr.replace('#', '').trim();
    if (raw.length === 3) {
      raw = raw.split('').map((c) => c + c).join('') + 'FF';
    } else if (raw.length === 6) {
      raw = raw + 'FF';
    } else if (raw.length !== 8) {
      return null;
    }
    const r = parseInt(raw.substring(0, 2), 16);
    const g = parseInt(raw.substring(2, 4), 16);
    const b = parseInt(raw.substring(4, 6), 16);
    const a = parseInt(raw.substring(6, 8), 16) / 255;
    return { r, g, b, a };
  }

  function applyColor(pushUndo: boolean = true) {
    const [r, g, b] = hsvToRgb(hue, saturation, value);
    const hex = rgbToHex(r, g, b, alpha);
    editorState.setLayerColor(hex, pushUndo);
  }

  function handleSvMouseDown(e: MouseEvent) {
    isDraggingSV = true;
    editorState.pushUndo();
    updateSvFromMouse(e, false);
    window.addEventListener('mousemove', handleSvMouseMove);
    window.addEventListener('mouseup', handleSvMouseUp);
  }

  function handleSvMouseMove(e: MouseEvent) {
    if (isDraggingSV) {
      updateSvFromMouse(e, false);
    }
  }

  function handleSvMouseUp() {
    isDraggingSV = false;
    window.removeEventListener('mousemove', handleSvMouseMove);
    window.removeEventListener('mouseup', handleSvMouseUp);
    editorState.syncWithOverlay();
  }

  function updateSvFromMouse(e: MouseEvent, pushUndo: boolean = false) {
    if (!svBoxEl) return;
    const rect = svBoxEl.getBoundingClientRect();
    const x = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
    const y = Math.max(0, Math.min(e.clientY - rect.top, rect.height));
    saturation = Math.round((x / rect.width) * 100) / 100;
    value = Math.round((1 - y / rect.height) * 100) / 100;
    applyColor(pushUndo);
  }

  function onHexInput(e: Event) {
    const val = (e.target as HTMLInputElement).value;
    const parsed = parseHex(val);
    if (parsed) {
      editorState.setLayerColor(val.toUpperCase());
    }
  }

  async function triggerEyedropper() {
    // Canvas Eyedropper support (§7)
    if (typeof window !== 'undefined' && 'EyeDropper' in window) {
      try {
        const eyeDropper = new (window as any).EyeDropper();
        const result = await eyeDropper.open();
        if (result && result.sRGBHex) {
          editorState.setLayerColor(result.sRGBHex.toUpperCase());
        }
      } catch {
        // User cancelled eyedropper
      }
    }
  }

  function addCurrentToSwatches() {
    const currentColor = editorState.selectedLayer?.color || '#36F2CB';
    if (!editorState.savedSwatches.includes(currentColor)) {
      editorState.savedSwatches = [...editorState.savedSwatches, currentColor];
    }
  }

  let pureHueRgb = $derived(hsvToRgb(hue, 1, 1));
  let currentRgb = $derived(hsvToRgb(hue, saturation, value));
</script>

<div class="panel color-picker-panel">
  <div class="panel-header">
    <div style="display: flex; align-items: center; gap: 8px;">
      <span>Color & Paint (§7)</span>
      <span class="mono color-readout" style="color: {editorState.selectedLayer?.color || '#38BDF8'};">
        {editorState.selectedLayer?.color || '#36F2CB'}
      </span>
    </div>
    <Tooltip title="Sample Color" description="Eyedropper pixel sampler" shortcut="I">
      <button class="btn btn-sm btn-icon" onclick={triggerEyedropper} aria-label="Eyedropper">
        🖌
      </button>
    </Tooltip>
  </div>

  <div class="picker-body">
    <!-- Saturation / Brightness 2D Square (§7) -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="sv-square"
      bind:this={svBoxEl}
      onmousedown={handleSvMouseDown}
      style="background-color: rgb({pureHueRgb[0]}, {pureHueRgb[1]}, {pureHueRgb[2]});"
    >
      <div class="sv-gradient-white"></div>
      <div class="sv-gradient-black"></div>
      <div
        class="sv-handle"
        style="left: {saturation * 100}%; top: {(1 - value) * 100}%;"
      ></div>
    </div>

    <!-- Hue Slider -->
    <div class="slider-row">
      <span class="mono label">HUE</span>
      <input
        type="range"
        class="hue-slider"
        min="0"
        max="360"
        step="1"
        bind:value={hue}
        onpointerdown={() => editorState.pushUndo()}
        oninput={() => applyColor(false)}
        onchange={() => editorState.syncWithOverlay()}
        aria-label="Hue slider"
      />
    </div>

    <!-- Alpha Slider (Over Checkerboard) -->
    <div class="slider-row">
      <span class="mono label">ALPHA</span>
      <div class="alpha-slider-wrap">
        <input
          type="range"
          class="alpha-slider"
          min="0"
          max="1"
          step="0.01"
          bind:value={alpha}
          onpointerdown={() => editorState.pushUndo()}
          oninput={() => applyColor(false)}
          onchange={() => editorState.syncWithOverlay()}
          style="--thumb-color: rgb({currentRgb[0]}, {currentRgb[1]}, {currentRgb[2]});"
          aria-label="Alpha opacity slider"
        />
      </div>
      <span class="mono alpha-readout">{Math.round(alpha * 100)}%</span>
    </div>

    <!-- Hex and Format Inputs -->
    <div class="format-section">
      <div class="format-tabs mono">
        <button
          class="btn tab-btn {activeFormatTab === 'HEX' ? 'active' : ''}"
          onclick={() => (activeFormatTab = 'HEX')}
        >
          HEX
        </button>
        <button
          class="btn tab-btn {activeFormatTab === 'RGB' ? 'active' : ''}"
          onclick={() => (activeFormatTab = 'RGB')}
        >
          RGB
        </button>
        <button
          class="btn tab-btn {activeFormatTab === 'HSL' ? 'active' : ''}"
          onclick={() => (activeFormatTab = 'HSL')}
        >
          HSL
        </button>
      </div>

      {#if activeFormatTab === 'HEX'}
        <div class="hex-input-group">
          <input
            type="text"
            class="input-text mono hex-text"
            value={editorState.selectedLayer?.color || '#36F2CB'}
            oninput={onHexInput}
            maxlength="9"
            aria-label="Hex color string"
          />
          <Tooltip title="Add Swatch" description="Save current color to swatches palette">
            <button class="btn btn-sm btn-add-swatch" onclick={addCurrentToSwatches}>
              + Swatch
            </button>
          </Tooltip>
        </div>
      {:else if activeFormatTab === 'RGB'}
        <div class="triplet-inputs mono">
          <span>R: {currentRgb[0]}</span>
          <span>G: {currentRgb[1]}</span>
          <span>B: {currentRgb[2]}</span>
        </div>
      {:else}
        <div class="triplet-inputs mono">
          <span>H: {Math.round(hue)}°</span>
          <span>S: {Math.round(saturation * 100)}%</span>
          <span>L: {Math.round(value * 100)}%</span>
        </div>
      {/if}
    </div>

    <!-- Saved Swatches Palette -->
    <div class="swatches-section">
      <div class="swatches-header mono">
        <span>SAVED SWATCHES</span>
      </div>
      <div class="swatches-grid">
        {#each editorState.savedSwatches as hex}
          <Tooltip title={hex} details="Click to apply">
            <button
              class="swatch-btn {editorState.selectedLayer?.color === hex ? 'active' : ''}"
              style="background-color: {hex};"
              onclick={() => editorState.setLayerColor(hex)}
              aria-label="Swatch {hex}"
            >
              {#if editorState.selectedLayer?.color === hex}
                <span class="swatch-check">✓</span>
              {/if}
            </button>
          </Tooltip>
        {/each}
      </div>
    </div>

    <!-- Colorblind-Safe Palette (§7) -->
    <div class="swatches-section">
      <div class="swatches-header mono">
        <span>COLORBLIND-SAFE PALETTE</span>
      </div>
      <div class="swatches-grid">
        {#each colorblindSafeSwatches as swatch}
          <Tooltip title={swatch.name} details="Hex: {swatch.hex}">
            <button
              class="swatch-btn {editorState.selectedLayer?.color === swatch.hex ? 'active' : ''}"
              style="background-color: {swatch.hex};"
              onclick={() => editorState.setLayerColor(swatch.hex)}
              aria-label={swatch.name}
            >
              {#if editorState.selectedLayer?.color === swatch.hex}
                <span class="swatch-check">✓</span>
              {/if}
            </button>
          </Tooltip>
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  .color-picker-panel {
    background: var(--surface1);
  }

  .color-readout {
    font-size: 11px;
    font-weight: 700;
  }

  .picker-body {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .sv-square {
    width: 100%;
    height: 140px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
    position: relative;
    cursor: crosshair;
    overflow: hidden;
  }

  .sv-gradient-white {
    position: absolute;
    inset: 0;
    background: linear-gradient(to right, #FFF, transparent);
  }

  .sv-gradient-black {
    position: absolute;
    inset: 0;
    background: linear-gradient(to top, #000, transparent);
  }

  .sv-handle {
    position: absolute;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: 2px solid #FFF;
    box-shadow: 0 0 3px #000;
    transform: translate(-50%, -50%);
    pointer-events: none;
  }

  .slider-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .label {
    font-size: 10px;
    color: var(--text-muted);
    width: 44px;
    flex: none;
  }

  .hue-slider {
    flex: 1;
    height: 10px;
    border-radius: 5px;
    -webkit-appearance: none;
    appearance: none;
    background: linear-gradient(to right, #F00, #FF0, #0F0, #0FF, #00F, #F0F, #F00);
    outline: none;
    cursor: pointer;
  }

  .hue-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #FFF;
    border: 2px solid #000;
    cursor: pointer;
  }

  .alpha-slider-wrap {
    flex: 1;
    height: 10px;
    border-radius: 5px;
    background: repeating-conic-gradient(#333 0% 25%, #222 0% 50%) 50% / 8px 8px;
    position: relative;
    display: flex;
    align-items: center;
  }

  .alpha-slider {
    width: 100%;
    height: 10px;
    border-radius: 5px;
    -webkit-appearance: none;
    appearance: none;
    background: linear-gradient(to right, transparent, var(--thumb-color, #FFF));
    outline: none;
    cursor: pointer;
    margin: 0;
  }

  .alpha-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #FFF;
    border: 2px solid #000;
    cursor: pointer;
  }

  .alpha-readout {
    font-size: 11px;
    color: var(--text-muted);
    width: 36px;
    text-align: right;
  }

  .format-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding-top: 4px;
    border-top: 1px solid var(--border-subtle);
  }

  .format-tabs {
    display: flex;
    gap: 4px;
  }

  .tab-btn {
    height: 20px;
    padding: 0 6px;
    font-size: 10px;
    background: var(--surface2);
    border: 1px solid var(--border);
    color: var(--text-muted);
  }

  .tab-btn.active {
    background: var(--surface3);
    border-color: var(--border-active);
    color: var(--text);
  }

  .hex-input-group {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .hex-text {
    flex: 1;
    height: 28px;
    font-size: 12px;
    padding: 0 8px;
  }

  .btn-add-swatch {
    height: 28px;
    font-size: 11px;
    background: var(--surface2);
    border: 1px solid var(--border);
    color: var(--text);
  }

  .triplet-inputs {
    display: flex;
    justify-content: space-around;
    padding: 6px;
    background: var(--surface2);
    border-radius: var(--radius-sm);
    font-size: 11px;
    color: var(--text);
  }

  .swatches-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .swatches-header {
    font-size: 10px;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .swatches-grid {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
    gap: 4px;
  }

  .swatch-btn {
    height: 22px;
    border-radius: var(--radius-xs);
    border: 1px solid rgba(255, 255, 255, 0.2);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 80ms ease, border-color 80ms ease;
  }

  .swatch-btn:hover {
    transform: scale(1.1);
    border-color: #fff;
  }

  .swatch-btn.active {
    box-shadow: 0 0 0 2px var(--surface1), 0 0 0 3px var(--accent);
  }

  .swatch-check {
    font-size: 10px;
    color: #000;
    text-shadow: 0 0 2px #fff;
    font-weight: 800;
  }
</style>
