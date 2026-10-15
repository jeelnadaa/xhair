<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    description?: string;
    details?: string;
    shortcut?: string;
    disabled?: boolean;
    children: Snippet;
  }

  let { title, description, details, shortcut, disabled = false, children }: Props = $props();

  let visible = $state(false);
  let timeoutId: number | null = null;
  let tooltipEl = $state<HTMLDivElement | null>(null);
  let coords = $state({ x: 0, y: 0 });

  function show(e: MouseEvent | FocusEvent) {
    if (disabled) return;
    const target = (e.currentTarget || e.target) as HTMLElement;
    if (target && target.getBoundingClientRect) {
      const rect = target.getBoundingClientRect();
      coords = {
        x: rect.left + rect.width / 2,
        y: rect.bottom + 8,
      };
    }
    timeoutId = window.setTimeout(() => {
      visible = true;
    }, 400);
  }

  function hide() {
    if (timeoutId !== null) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
    visible = false;
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      hide();
    }
  }
</script>

<svelte:window onkeydown={onKeyDown} />

<div
  class="tooltip-wrapper"
  onmouseenter={show}
  onmouseleave={hide}
  onfocusin={show}
  onfocusout={hide}
  role="presentation"
>
  {@render children()}

  {#if visible}
    <div
      bind:this={tooltipEl}
      class="tooltip-box"
      style="left: {coords.x}px; top: {coords.y}px;"
      role="tooltip"
    >
      <div class="tooltip-header">
        <span class="tooltip-title">{title}</span>
        {#if shortcut}
          <span class="tooltip-shortcut">{shortcut}</span>
        {/if}
      </div>
      {#if description}
        <div class="tooltip-desc">{description}</div>
      {/if}
      {#if details}
        <div class="tooltip-details">{details}</div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .tooltip-wrapper {
    display: inline-flex;
    position: relative;
    max-width: 100%;
  }

  .tooltip-box {
    position: fixed;
    transform: translateX(-50%);
    background: #1A2027;
    border: 1px solid #262E38;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
    border-radius: 6px;
    padding: 8px 12px;
    z-index: 10000;
    pointer-events: none;
    max-width: 280px;
    font-family: var(--font-sans, system-ui);
    animation: fadeIn 120ms ease;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translate(-50%, -4px);
    }
    to {
      opacity: 1;
      transform: translate(-50%, 0);
    }
  }

  .tooltip-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
    margin-bottom: 4px;
  }

  .tooltip-title {
    font-weight: 700;
    font-size: 12px;
    color: #E9EEF3;
  }

  .tooltip-shortcut {
    font-family: var(--font-mono, monospace);
    font-size: 10px;
    background: #0B0D10;
    border: 1px solid #262E38;
    border-radius: 4px;
    padding: 1px 5px;
    color: #36F2CB;
    font-weight: 600;
  }

  .tooltip-desc {
    font-size: 11px;
    color: #93A0AF;
    line-height: 1.35;
  }

  .tooltip-details {
    font-family: var(--font-mono, monospace);
    font-size: 10px;
    color: #36F2CB;
    margin-top: 4px;
  }
</style>
