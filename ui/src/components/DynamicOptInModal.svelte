<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  let hasConfirmedRisk = $state(false);

  function close() {
    editorState.dynamicOptInOpen = false;
    hasConfirmedRisk = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      close();
    }
  }

  function handleConfirm() {
    if (!hasConfirmedRisk) return;
    editorState.confirmDynamicOptIn();
    close();
  }

  function openSprayLabInstead() {
    close();
    editorState.openSprayLab();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if editorState.dynamicOptInOpen}
  <div class="modal-backdrop" onclick={close} role="presentation">
    <div
      class="modal-content"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      tabindex="-1"
      role="dialog"
      aria-modal="true"
      aria-labelledby="dynamics-optin-title"
    >
      <header class="modal-header">
        <div class="title-row">
          <span class="warning-badge">⚠️ RISK ADVISORY</span>
          <h2 id="dynamics-optin-title">Dynamic Reticle Session Opt-In</h2>
        </div>
        <button class="close-btn" onclick={close} aria-label="Close modal">✕</button>
      </header>

      <div class="modal-body">
        <div class="notice-callout">
          <div class="callout-icon">🛡️</div>
          <div class="callout-text">
            <strong>Architecture & Zero-Memory Safety (§2, §12)</strong>
            <p>
              XHAIR's dynamic reticle is a <em>pure mathematical simulation</em>. It reacts only to your
              own bound keys (WASD, Mouse1) through restricted Raw Input. It
              <strong>never</strong> reads game memory, injects code, hooks game threads, or captures
              game video.
            </p>
          </div>
        </div>

        <div class="risk-section">
          <h3>Publisher & Anti-Cheat Notice</h3>
          <p class="risk-desc">
            Anti-cheat behavior can change without notice. No gameplay aid can be unconditionally
            guaranteed safe across all titles. Competitive leagues and anti-cheat software (e.g.,
            Vanguard, Easy Anti-Cheat, BattlEye) may restrict reactive overlays in ranked queues.
          </p>
          <ul class="bullet-list">
            <li>Check the publisher's current rules before using in ranked or competitive play.</li>
            <li>Gun recoil numbers are user-entered example curves; XHAIR never ships official game data.</li>
            <li>This opt-in is valid for the current session only and will reset upon application restart.</li>
          </ul>
        </div>

        <div class="safe-alt-box">
          <div class="safe-icon">🎯</div>
          <div class="safe-text">
            <strong>Recommended Safe Alternative: Spray Lab</strong>
            <span>Test recoil curves and practice sprays in a standalone window with zero overlay risk.</span>
          </div>
          <button class="btn btn-secondary safe-btn" onclick={openSprayLabInstead}>
            Open Spray Lab
          </button>
        </div>

        <label class="consent-checkbox-row">
          <input type="checkbox" bind:checked={hasConfirmedRisk} />
          <span class="checkbox-label">
            I understand that anti-cheat rules vary by publisher, and I accept full responsibility for enabling dynamic reticle simulation for this session.
          </span>
        </label>
      </div>

      <footer class="modal-footer">
        <button class="btn btn-secondary" onclick={close}>
          Cancel
        </button>
        <Tooltip
          title="Session Confirmation"
          description="Enables restricted Raw Input simulation until application close"
        >
          <button
            class="btn btn-primary"
            disabled={!hasConfirmedRisk}
            onclick={handleConfirm}
          >
            Enable Dynamic Reticle (This Session)
          </button>
        </Tooltip>
      </footer>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.78);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    width: 600px;
    max-width: 92vw;
    background: var(--surface1);
    border: 1px solid var(--border);
    border-radius: 12px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.6);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    background: var(--surface2);
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .warning-badge {
    background: rgba(255, 180, 0, 0.15);
    color: #ffb400;
    border: 1px solid rgba(255, 180, 0, 0.35);
    font-size: 11px;
    font-weight: 700;
    padding: 3px 8px;
    border-radius: 4px;
    letter-spacing: 0.05em;
  }

  h2 {
    font-size: 16px;
    font-weight: 700;
    color: var(--text);
    margin: 0;
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

  .modal-body {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    max-height: 70vh;
    overflow-y: auto;
  }

  .notice-callout {
    display: flex;
    gap: 14px;
    background: rgba(54, 242, 203, 0.08);
    border: 1px solid rgba(54, 242, 203, 0.25);
    border-radius: 8px;
    padding: 14px;
  }

  .callout-icon {
    font-size: 22px;
    flex: none;
  }

  .callout-text strong {
    display: block;
    color: var(--accent);
    font-size: 13px;
    margin-bottom: 4px;
  }

  .callout-text p {
    font-size: 12px;
    line-height: 1.5;
    color: var(--text);
    margin: 0;
  }

  .risk-section h3 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    margin: 0 0 6px 0;
  }

  .risk-desc {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
    margin: 0 0 10px 0;
  }

  .bullet-list {
    margin: 0;
    padding-left: 20px;
    font-size: 12px;
    color: var(--text);
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .safe-alt-box {
    display: flex;
    align-items: center;
    gap: 12px;
    background: rgba(255, 255, 255, 0.03);
    border: 1px dashed var(--border);
    border-radius: 8px;
    padding: 12px 14px;
  }

  .safe-icon {
    font-size: 20px;
    flex: none;
  }

  .safe-text {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .safe-text strong {
    font-size: 12px;
    color: var(--text);
  }

  .safe-text span {
    font-size: 11px;
    color: var(--text-muted);
  }

  .safe-btn {
    flex: none;
    font-size: 11px;
    padding: 6px 12px;
  }

  .consent-checkbox-row {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    cursor: pointer;
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 12px;
    margin-top: 4px;
  }

  .consent-checkbox-row input[type='checkbox'] {
    margin-top: 2px;
    accent-color: var(--accent);
    width: 16px;
    height: 16px;
    cursor: pointer;
  }

  .checkbox-label {
    font-size: 12px;
    line-height: 1.45;
    color: var(--text);
  }

  .modal-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 10px;
    padding: 14px 20px;
    border-top: 1px solid var(--border);
    background: var(--surface2);
  }

  .btn {
    height: 34px;
    padding: 0 16px;
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

  .btn-primary:hover:not(:disabled) {
    filter: brightness(1.1);
  }

  .btn-primary:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
</style>
