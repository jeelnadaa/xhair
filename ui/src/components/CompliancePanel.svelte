<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  function close() {
    editorState.compliancePanelOpen = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      close();
    }
  }

  const ALLOWED_APIS = [
    {
      api: 'UpdateLayeredWindow / WS_EX_LAYERED',
      purpose: 'Renders transparent, click-through, topmost custom overlay window',
      scope: 'Own window handle only',
    },
    {
      api: 'EnumDisplayMonitors / GetMonitorInfo / GetDpiForMonitor',
      purpose: 'Queries monitor physical geometry, DPI scale, and refresh rates',
      scope: 'Display desktop settings',
    },
    {
      api: 'GetForegroundWindow / QueryFullProcessImageName',
      purpose: 'Tracks active window title & process name with limited-query rights (PROCESS_QUERY_LIMITED_INFORMATION)',
      scope: 'Game focus detection only (no memory access)',
    },
    {
      api: 'SetWinEventHook(EVENT_SYSTEM_FOREGROUND)',
      purpose: 'Subscribes to out-of-context foreground window change notifications',
      scope: 'OS window events (zero code injection)',
    },
    {
      api: 'RegisterHotKey / UnregisterHotKey',
      purpose: 'Registers system-wide hotkeys (e.g. F10 overlay toggle, F8 calibration)',
      scope: 'Registered key combos only',
    },
    {
      api: 'RegisterRawInputDevices (Opt-In Only)',
      purpose: 'Receives user-bound locomotion & fire buttons (WASD, Mouse1) for dynamic spread simulation',
      scope: 'User-bound keys only (no logging, no persistence)',
    },
    {
      api: 'AppData File System & Loopback HTTP (127.0.0.1)',
      purpose: 'Saves local crosshair catalog, profiles, and serves MCP tools via localhost Bearer auth',
      scope: 'App directory only; zero telemetry or external network calls',
    },
  ];

  const FORBIDDEN_APIS = [
    {
      api: 'OpenProcess / ReadProcessMemory / WriteProcessMemory',
      rule: 'FORBIDDEN (Spec §2.1)',
      status: 'NEVER CALLED',
      reason: 'Strict zero-game-memory invariant. XHAIR never opens or inspects game address space.',
    },
    {
      api: 'CreateRemoteThread / VirtualAllocEx / DLL Injection',
      rule: 'FORBIDDEN (Spec §2.1)',
      status: 'NEVER CALLED',
      reason: 'No foreign thread creation, DLL injection, or driver-level code.',
    },
    {
      api: 'WH_KEYBOARD_LL / WH_MOUSE_LL Low-Level Hooks',
      rule: 'FORBIDDEN (Spec §2.3)',
      status: 'NEVER CALLED',
      reason: 'Low-level OS hooks intercept arbitrary keystrokes and introduce input lag. Strictly prohibited.',
    },
    {
      api: 'BitBlt / PrintWindow / DXGI Desktop Duplication (Live Screen Capture)',
      rule: 'FORBIDDEN (Spec §2.1)',
      status: 'NEVER CALLED',
      reason: 'No live screen capture of game frames. Alignment calibration uses local user-provided files only.',
    },
    {
      api: 'External Telemetry / Cloud Analytics / Remote Network Sockets',
      rule: 'FORBIDDEN (Spec §2.2)',
      status: 'NEVER CALLED',
      reason: 'Zero outbound telemetry or analytics. Only loopback 127.0.0.1 is permitted for local AI agent MCP tools.',
    },
  ];
</script>

<svelte:window onkeydown={handleKeydown} />

{#if editorState.compliancePanelOpen}
  <div class="modal-backdrop" onclick={close} role="presentation">
    <div
      class="modal-content"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      tabindex="-1"
      role="dialog"
      aria-modal="true"
      aria-labelledby="compliance-title"
    >
      <header class="modal-header">
        <div class="title-row">
          <span class="shield-badge">🛡️ TRANSPARENCY</span>
          <h2 id="compliance-title">Compliance & Anti-Cheat Transparency Panel (§2)</h2>
        </div>
        <button class="close-btn" onclick={close} aria-label="Close panel">✕</button>
      </header>

      <div class="modal-body">
        <div class="intro-box">
          <p>
            XHAIR operates on a <strong>provable zero-touch architecture</strong>. It runs entirely as an independent
            desktop window on top of Windows DWM. Below is the exhaustive audit of all permitted Windows API
            surfaces and strict architectural prohibitions.
          </p>
        </div>

        <section class="section">
          <div class="section-header">
            <span class="status-indicator allowed"></span>
            <h3>Whitelisted Windows OS Interactions (Exhaustive List)</h3>
          </div>
          <div class="table-container">
            <table>
              <thead>
                <tr>
                  <th>API Surface</th>
                  <th>Functional Purpose</th>
                  <th>Audit Scope</th>
                </tr>
              </thead>
              <tbody>
                {#each ALLOWED_APIS as item}
                  <tr>
                    <td class="mono font-highlight">{item.api}</td>
                    <td>{item.purpose}</td>
                    <td class="muted">{item.scope}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </section>

        <section class="section">
          <div class="section-header">
            <span class="status-indicator forbidden"></span>
            <h3>Prohibited & Forbidden APIs (Code-Level Invariants)</h3>
          </div>
          <div class="table-container">
            <table>
              <thead>
                <tr>
                  <th>Forbidden Pattern</th>
                  <th>Policy Rule</th>
                  <th>Enforcement</th>
                  <th>Rationale</th>
                </tr>
              </thead>
              <tbody>
                {#each FORBIDDEN_APIS as item}
                  <tr>
                    <td class="mono forbidden-text">{item.api}</td>
                    <td><span class="tag-forbidden">{item.rule}</span></td>
                    <td class="status-verified">{item.status}</td>
                    <td class="muted">{item.reason}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </section>
      </div>

      <footer class="modal-footer">
        <div class="footer-note">
          Verified against <code>docs/spec/CROSSHAIR_OVERLAY_SPEC.md §2</code>.
        </div>
        <button class="btn btn-secondary" onclick={close}>Close</button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.8);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    width: 860px;
    max-width: 95vw;
    background: var(--surface1);
    border: 1px solid var(--border);
    border-radius: 12px;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.7);
    display: flex;
    flex-direction: column;
    max-height: 85vh;
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

  .shield-badge {
    background: rgba(54, 242, 203, 0.12);
    color: var(--accent);
    border: 1px solid rgba(54, 242, 203, 0.3);
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
    gap: 20px;
    overflow-y: auto;
  }

  .intro-box p {
    font-size: 13px;
    color: var(--text);
    line-height: 1.5;
    margin: 0;
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }

  .section-header h3 {
    font-size: 13px;
    font-weight: 700;
    color: var(--text);
    margin: 0;
    letter-spacing: 0.02em;
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .status-indicator.allowed {
    background: var(--accent);
    box-shadow: 0 0 8px rgba(54, 242, 203, 0.4);
  }

  .status-indicator.forbidden {
    background: #ff4f4f;
    box-shadow: 0 0 8px rgba(255, 79, 79, 0.4);
  }

  .table-container {
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 12px;
    text-align: left;
  }

  th {
    background: var(--surface2);
    padding: 8px 12px;
    color: var(--text-muted);
    font-weight: 600;
    border-bottom: 1px solid var(--border);
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  td {
    padding: 10px 12px;
    border-bottom: 1px solid var(--border);
    line-height: 1.4;
    color: var(--text);
  }

  tr:last-child td {
    border-bottom: none;
  }

  .mono {
    font-family: var(--font-mono);
    font-size: 11px;
  }

  .font-highlight {
    color: var(--accent);
  }

  .forbidden-text {
    color: #ff7777;
  }

  .muted {
    color: var(--text-muted);
    font-size: 11px;
  }

  .tag-forbidden {
    background: rgba(255, 79, 79, 0.15);
    color: #ff5555;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 700;
  }

  .status-verified {
    color: #36f2cb;
    font-weight: 600;
    font-size: 11px;
  }

  .modal-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    border-top: 1px solid var(--border);
    background: var(--surface2);
  }

  .footer-note {
    font-size: 11px;
    color: var(--text-muted);
  }

  .btn {
    height: 32px;
    padding: 0 16px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid var(--border);
    background: transparent;
    color: var(--text);
    transition: all 0.15s ease;
  }

  .btn:hover {
    background: rgba(255, 255, 255, 0.05);
  }
</style>
