<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  const presetSwatches = [
    { name: 'Cyan (Default)', hex: '#36F2CB' },
    { name: 'Neon Green', hex: '#00FF66' },
    { name: 'Vibrant Yellow', hex: '#FFE600' },
    { name: 'Pure White', hex: '#FFFFFF' },
    { name: 'Hot Pink / Magenta', hex: '#FF4FD8' },
    { name: 'Crimson Red', hex: '#FF3366' },
    { name: 'Electric Blue', hex: '#00D4FF' },
    { name: 'Stealth Black', hex: '#000000' },
  ];

  function onHexInput(e: Event) {
    const target = e.target as HTMLInputElement;
    const val = target.value;
    if (/^#[0-9A-Fa-f]{6}$/.test(val)) {
      editorState.setLayerColor(val);
    }
  }
</script>

<div class="panel">
  <div class="panel-header">
    <span>Color & Paint</span>
    <span class="mono" style="color: var(--accent); font-size: 11px;">
      {editorState.selectedLayer?.color || '#36F2CB'}
    </span>
  </div>

  <div class="picker-body">
    <!-- Swatches Grid -->
    <div class="swatches-grid">
      {#each presetSwatches as swatch}
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

    <!-- Hex Input & Native Picker Trigger -->
    <div class="row hex-row">
      <span class="mono label">HEX</span>
      <div class="hex-input-group">
        <input
          type="color"
          class="native-picker"
          value={editorState.selectedLayer?.color || '#36F2CB'}
          oninput={(e) => editorState.setLayerColor((e.target as HTMLInputElement).value)}
          aria-label="Native color picker"
        />
        <input
          type="text"
          class="input-text mono hex-text"
          value={editorState.selectedLayer?.color || '#36F2CB'}
          oninput={onHexInput}
          maxlength="7"
          aria-label="Hex color string"
        />
      </div>
    </div>
  </div>
</div>

<style>
  .picker-body {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .swatches-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 8px;
  }

  .swatch-btn {
    height: 28px;
    border-radius: var(--radius-sm);
    border: 1px solid rgba(255, 255, 255, 0.2);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 80ms ease, border-color 80ms ease;
  }

  .swatch-btn:hover {
    transform: scale(1.06);
    border-color: #fff;
  }

  .swatch-btn.active {
    box-shadow: 0 0 0 2px var(--surface1), 0 0 0 4px var(--accent);
  }

  .swatch-check {
    font-size: 14px;
    color: #000;
    text-shadow: 0 0 2px #fff;
    font-weight: 800;
  }

  .hex-row {
    margin-top: 4px;
  }

  .hex-input-group {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .native-picker {
    width: 28px;
    height: 28px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 0;
    cursor: pointer;
    background: transparent;
  }

  .hex-text {
    width: 80px;
    height: 28px;
    text-align: center;
  }

  .label {
    font-size: 11px;
    color: var(--text-muted);
  }
</style>
