<script lang="ts">
  let {
    label,
    value = 0,
    min = -9999,
    max = 9999,
    step = 1,
    unit = 'px',
    precision = 1,
    tooltip = '',
    disabled = false,
    onchange,
  }: {
    label: string;
    value: number;
    min?: number;
    max?: number;
    step?: number;
    unit?: string;
    precision?: number;
    tooltip?: string;
    disabled?: boolean;
    onchange?: (val: number) => void;
  } = $props();

  let isScrubbing = $state(false);
  let startX = 0;
  let startVal = 0;
  let localText = $state('');
  let isEditing = $state(false);

  // Sync localText when value changes externally and not editing
  $effect(() => {
    if (!isEditing) {
      localText = Number.isInteger(value) ? String(value) : value.toFixed(precision);
    }
  });

  function clampAndRound(val: number): number {
    const clamped = Math.max(min, Math.min(max, val));
    const factor = Math.pow(10, precision);
    return Math.round(clamped * factor) / factor;
  }

  function handlePointerDown(e: PointerEvent) {
    if (disabled || isEditing) return;
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
    isScrubbing = true;
    startX = e.clientX;
    startVal = value;
  }

  function handlePointerMove(e: PointerEvent) {
    if (!isScrubbing || disabled) return;
    const deltaX = e.clientX - startX;
    const currentStep = e.altKey ? step * 0.1 : e.shiftKey ? step * 10 : step;
    const pixelsPerStep = 3;
    const stepsMoved = deltaX / pixelsPerStep;
    const rawVal = startVal + stepsMoved * currentStep;
    const finalVal = clampAndRound(rawVal);
    if (finalVal !== value) {
      onchange?.(finalVal);
    }
  }

  function handlePointerUp(e: PointerEvent) {
    if (isScrubbing) {
      isScrubbing = false;
      try {
        (e.target as HTMLElement).releasePointerCapture(e.pointerId);
      } catch (_) {}
    }
  }

  function handleInput(e: Event) {
    localText = (e.target as HTMLInputElement).value;
  }

  function commitInput() {
    isEditing = false;
    const parsed = parseFloat(localText);
    if (!isNaN(parsed)) {
      const finalVal = clampAndRound(parsed);
      localText = Number.isInteger(finalVal) ? String(finalVal) : finalVal.toFixed(precision);
      onchange?.(finalVal);
    } else {
      localText = Number.isInteger(value) ? String(value) : value.toFixed(precision);
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (disabled) return;
    const currentStep = e.altKey ? step * 0.1 : e.shiftKey ? step * 10 : step;

    if (e.key === 'ArrowUp') {
      e.preventDefault();
      const finalVal = clampAndRound(value + currentStep);
      onchange?.(finalVal);
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      const finalVal = clampAndRound(value - currentStep);
      onchange?.(finalVal);
    } else if (e.key === 'Enter') {
      commitInput();
      (e.target as HTMLInputElement).blur();
    } else if (e.key === 'Escape') {
      isEditing = false;
      localText = Number.isInteger(value) ? String(value) : value.toFixed(precision);
      (e.target as HTMLInputElement).blur();
    }
  }

  function handleWheel(e: WheelEvent) {
    if (disabled) return;
    e.preventDefault();
    const dir = e.deltaY < 0 ? 1 : -1;
    const currentStep = e.altKey ? step * 0.1 : e.shiftKey ? step * 10 : step;
    const finalVal = clampAndRound(value + dir * currentStep);
    onchange?.(finalVal);
  }
</script>

<div
  class="scrub-field {isScrubbing ? 'is-scrubbing' : ''} {disabled ? 'is-disabled' : ''}"
  title={tooltip || `${label}: Drag label to scrub, or click to type`}
>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <span
    class="scrub-label mono"
    onpointerdown={handlePointerDown}
    onpointermove={handlePointerMove}
    onpointerup={handlePointerUp}
    onpointercancel={handlePointerUp}
  >
    {label}
  </span>
  <div class="scrub-input-wrap">
    <input
      type="text"
      class="scrub-input mono"
      value={localText}
      {disabled}
      onfocus={() => (isEditing = true)}
      onblur={commitInput}
      oninput={handleInput}
      onkeydown={handleKeyDown}
      onwheel={handleWheel}
      aria-label="{label} input"
    />
    {#if unit}
      <span class="scrub-unit mono">{unit}</span>
    {/if}
  </div>
</div>

<style>
  .scrub-field {
    display: inline-flex;
    align-items: center;
    background: var(--surface2, #16181d);
    border: 1px solid var(--border, #262930);
    border-radius: var(--radius-sm, 6px);
    height: 28px;
    padding: 0 4px 0 0;
    transition: border-color 120ms ease, box-shadow 120ms ease, background 120ms ease;
    user-select: none;
    position: relative;
    box-sizing: border-box;
    width: 100%;
    min-width: 0;
  }

  .scrub-field:hover:not(.is-disabled) {
    border-color: var(--border-hover, #3d424e);
    background: var(--surface3, #1e2128);
  }

  .scrub-field:focus-within:not(.is-disabled) {
    border-color: var(--accent, #38bdf8);
    box-shadow: 0 0 0 1px var(--accent, #38bdf8);
  }

  .scrub-field.is-scrubbing {
    border-color: var(--accent, #38bdf8);
    background: var(--surface3, #1e2128);
    box-shadow: 0 0 0 1px rgba(56, 189, 248, 0.3);
  }

  .scrub-field.is-disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
  }

  .scrub-label {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 7px;
    height: 100%;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted, #8b949e);
    cursor: ew-resize;
    user-select: none;
    border-right: 1px solid var(--border, #262930);
    transition: color 100ms ease, background 100ms ease;
    flex-shrink: 0;
  }

  .scrub-label:hover {
    color: var(--accent, #38bdf8);
    background: rgba(56, 189, 248, 0.08);
  }

  .is-scrubbing .scrub-label {
    color: var(--accent, #38bdf8);
    background: rgba(56, 189, 248, 0.15);
  }

  .scrub-input-wrap {
    display: flex;
    align-items: center;
    flex: 1;
    min-width: 0;
    height: 100%;
    padding: 0 4px;
    position: relative;
  }

  .scrub-input {
    width: 100%;
    height: 100%;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text, #f0f6fc);
    font-size: 11px;
    padding: 0 2px;
    text-align: right;
    min-width: 0;
  }

  .scrub-unit {
    font-size: 10px;
    color: var(--text-muted, #6e7681);
    margin-left: 2px;
    pointer-events: none;
    flex-shrink: 0;
  }
</style>
