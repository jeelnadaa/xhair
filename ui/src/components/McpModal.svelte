<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  function close() {
    editorState.mcpPanelOpen = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      close();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if editorState.mcpPanelOpen}
  <div class="modal-backdrop" onclick={close} role="presentation">
    <div
      class="modal-content"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      tabindex="-1"
      role="dialog"
      aria-modal="true"
      aria-labelledby="mcp-title"
    >
      <header class="modal-header">
        <div class="title-row">
          <span class="mcp-badge">MCP</span>
          <h2 id="mcp-title">Model Context Protocol Activity Log</h2>
          {#if editorState.isAgentEditing}
            <span class="editing-pill pulsing">
              <span class="dot"></span> Agent is editing
            </span>
          {/if}
        </div>
        <button class="close-btn" onclick={close} aria-label="Close modal">✕</button>
      </header>

      <div class="status-banner">
        <div class="stat-item">
          <span class="label">Transport</span>
          <span class="val">stdio + Streamable HTTP</span>
        </div>
        <div class="stat-item">
          <span class="label">Host</span>
          <span class="val">127.0.0.1 (Loopback only)</span>
        </div>
        <div class="stat-item">
          <span class="label">Security</span>
          <span class="val">Bearer Token + DNS Guard</span>
        </div>
        <div class="stat-item">
          <span class="label">Parity Status</span>
          <span class="val green">P4 Verified</span>
        </div>
      </div>

      <div class="log-container">
        <div class="log-header-row">
          <span class="log-title">Recent Agent Invocations ({editorState.mcpActivityLog.length})</span>
          <button class="clear-btn" onclick={() => editorState.clearMcpLog()}>Clear Log</button>
        </div>

        {#if editorState.mcpActivityLog.length === 0}
          <div class="empty-state">No agent tool invocations recorded yet.</div>
        {:else}
          <div class="log-table-wrap">
            <table class="log-table">
              <thead>
                <tr>
                  <th>Time</th>
                  <th>Tool</th>
                  <th>Summary</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {#each editorState.mcpActivityLog as entry (entry.id)}
                  <tr>
                    <td class="time-col">{entry.timestamp}</td>
                    <td class="tool-col">
                      <code>{entry.tool}</code>
                    </td>
                    <td class="summary-col">{entry.summary}</td>
                    <td class="status-col">
                      <span class="status-badge {entry.status}">
                        {entry.status}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

      <footer class="modal-footer">
        <span class="hint">Press <code>Esc</code> to return to editor</span>
        <button class="btn btn-primary" onclick={close}>Done</button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    width: 680px;
    max-width: 90vw;
    max-height: 85vh;
    background: var(--surface1, #14181F);
    border: 1px solid var(--border, #262D38);
    border-radius: 8px;
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
    border-bottom: 1px solid var(--border, #262D38);
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .mcp-badge {
    background: var(--cyan, #36F2CB);
    color: #000;
    font-size: 11px;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 4px;
    letter-spacing: 0.05em;
  }

  h2 {
    font-size: 15px;
    font-weight: 600;
    color: #fff;
    margin: 0;
  }

  .editing-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: rgba(255, 79, 216, 0.15);
    color: #FF4FD8;
    border: 1px solid #FF4FD8;
    padding: 2px 8px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 600;
  }

  .editing-pill .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #FF4FD8;
  }

  .pulsing {
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-dim, #717E93);
    cursor: pointer;
    font-size: 16px;
    padding: 4px 8px;
    border-radius: 4px;
  }

  .close-btn:hover {
    color: #fff;
    background: rgba(255, 255, 255, 0.05);
  }

  .status-banner {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    background: var(--surface2, #1B212B);
    padding: 12px 20px;
    border-bottom: 1px solid var(--border, #262D38);
    gap: 12px;
  }

  .stat-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .stat-item .label {
    font-size: 10px;
    color: var(--text-dim, #717E93);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .stat-item .val {
    font-size: 12px;
    color: #fff;
    font-weight: 500;
  }

  .stat-item .val.green {
    color: #36F2CB;
  }

  .log-container {
    flex: 1;
    overflow-y: auto;
    padding: 16px 20px;
    min-height: 240px;
  }

  .log-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  .log-title {
    font-size: 12px;
    color: var(--text-dim, #717E93);
    font-weight: 600;
  }

  .clear-btn {
    background: transparent;
    border: none;
    color: var(--text-dim, #717E93);
    font-size: 11px;
    cursor: pointer;
    text-decoration: underline;
  }

  .clear-btn:hover {
    color: #fff;
  }

  .empty-state {
    color: var(--text-dim, #717E93);
    font-size: 13px;
    text-align: center;
    padding: 48px 0;
  }

  .log-table-wrap {
    border: 1px solid var(--border, #262D38);
    border-radius: 6px;
    overflow: hidden;
  }

  .log-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 12px;
  }

  .log-table th {
    background: var(--surface2, #1B212B);
    text-align: left;
    padding: 8px 12px;
    font-size: 10px;
    text-transform: uppercase;
    color: var(--text-dim, #717E93);
    border-bottom: 1px solid var(--border, #262D38);
  }

  .log-table td {
    padding: 8px 12px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
    color: #E2E8F0;
  }

  .time-col {
    color: var(--text-dim, #717E93);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
    width: 80px;
  }

  .tool-col code {
    background: rgba(54, 242, 203, 0.1);
    color: #36F2CB;
    padding: 2px 6px;
    border-radius: 3px;
    font-family: monospace;
    font-size: 11px;
  }

  .status-badge {
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status-badge.success {
    background: rgba(54, 242, 203, 0.15);
    color: #36F2CB;
  }

  .status-badge.error {
    background: rgba(255, 75, 75, 0.15);
    color: #FF4B4B;
  }

  .modal-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    background: var(--surface2, #1B212B);
    border-top: 1px solid var(--border, #262D38);
  }

  .hint {
    font-size: 11px;
    color: var(--text-dim, #717E93);
  }

  .hint code {
    background: rgba(255, 255, 255, 0.1);
    padding: 1px 4px;
    border-radius: 3px;
    color: #fff;
  }

  .btn-primary {
    background: var(--cyan, #36F2CB);
    color: #000;
    border: none;
    font-weight: 600;
    font-size: 12px;
    padding: 6px 16px;
    border-radius: 4px;
    cursor: pointer;
  }

  .btn-primary:hover {
    background: #4ef5d3;
  }
</style>
