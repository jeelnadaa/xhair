<script lang="ts">
  import { editorState } from '../lib/state.svelte';
  import Tooltip from './Tooltip.svelte';

  let inputText = $state('');
  let feedbackMessage = $state<{ type: 'success' | 'error'; text: string } | null>(null);
  let copyCodeSuccess = $state(false);
  let copyJsonSuccess = $state(false);

  let detectedFormat = $derived.by(() => {
    const t = inputText.trim();
    if (!t) return null;
    if (t.startsWith('{') && t.endsWith('}')) return '.xhair JSON Document';
    if (t.startsWith('XH1-')) return 'XHAIR Share Code (XH1)';
    if (t.startsWith('0;') || t.includes(';0l;') || t.includes(';0t;')) return 'Valorant In-Game Code';
    return 'Unknown String';
  });

  function handleImport() {
    feedbackMessage = null;
    const res = editorState.importCrosshair(inputText);
    if (res.success) {
      feedbackMessage = { type: 'success', text: res.message };
      setTimeout(() => {
        editorState.closeImportExport();
        feedbackMessage = null;
        inputText = '';
      }, 1200);
    } else {
      feedbackMessage = { type: 'error', text: res.message };
    }
  }

  function handleFileUpload(e: Event) {
    const input = e.target as HTMLInputElement;
    if (!input.files || input.files.length === 0) return;
    const file = input.files[0];
    const reader = new FileReader();
    reader.onload = (evt) => {
      const content = evt.target?.result as string;
      if (content) {
        inputText = content;
        handleImport();
      }
    };
    reader.readAsText(file);
  }

  async function copyShareCode() {
    const code = editorState.exportCrosshairCode();
    await navigator.clipboard.writeText(code);
    copyCodeSuccess = true;
    setTimeout(() => (copyCodeSuccess = false), 2000);
  }

  async function copyJson() {
    const json = editorState.exportCrosshairJson();
    await navigator.clipboard.writeText(json);
    copyJsonSuccess = true;
    setTimeout(() => (copyJsonSuccess = false), 2000);
  }

  function downloadJsonFile() {
    const json = editorState.exportCrosshairJson();
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${editorState.crosshair.name.toLowerCase().replace(/\s+/g, '-')}.xhair`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

{#if editorState.importExportModalOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={() => editorState.closeImportExport()}>
    <div class="modal-card" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div class="title-group">
          <strong class="title">Crosshair Import & Export</strong>
          <span class="badge">SPEC §11 · UNIVERSAL</span>
        </div>
        <button
          class="btn-close"
          onclick={() => editorState.closeImportExport()}
          aria-label="Close modal"
        >
          ✕
        </button>
      </div>

      <div class="tab-bar">
        <button
          class="tab-btn {editorState.importExportTab === 'import' ? 'active' : ''}"
          onclick={() => (editorState.importExportTab = 'import')}
        >
          📥 Import Profile
        </button>
        <button
          class="tab-btn {editorState.importExportTab === 'export' ? 'active' : ''}"
          onclick={() => (editorState.importExportTab = 'export')}
        >
          📤 Export & Share
        </button>
      </div>

      <div class="modal-content">
        {#if editorState.importExportTab === 'import'}
          <div class="section-desc">
            Paste a <code>.xhair</code> JSON structure, an <code>XH1-...</code> share code, or a Valorant in-game semicolon string (e.g. <code>0;P;c;5;...</code>).
          </div>

          <div class="textarea-wrapper">
            <textarea
              class="code-textarea mono"
              placeholder="Paste code or JSON here..."
              bind:value={inputText}
              rows="6"
            ></textarea>
            {#if detectedFormat}
              <div class="detected-badge">
                Format: <strong>{detectedFormat}</strong>
              </div>
            {/if}
          </div>

          <div class="upload-row">
            <label class="btn btn-secondary file-upload-btn">
              <span>📁 Browse .xhair File</span>
              <input type="file" accept=".xhair,.json" onchange={handleFileUpload} style="display: none;" />
            </label>
            <span class="file-hint">Supports direct drag-and-drop JSON files</span>
          </div>

          {#if feedbackMessage}
            <div class="feedback {feedbackMessage.type}">
              {feedbackMessage.text}
            </div>
          {/if}

          <div class="modal-actions">
            <button
              class="btn btn-secondary"
              onclick={() => editorState.closeImportExport()}
            >
              Cancel
            </button>
            <button
              class="btn btn-primary"
              disabled={!inputText.trim()}
              onclick={handleImport}
            >
              Import to Library
            </button>
          </div>
        {:else}
          <!-- Export Tab -->
          <div class="section-desc">
            Share <strong>"{editorState.crosshair.name}"</strong> across computers or games using a compact code or lossless file.
          </div>

          <div class="export-card">
            <div class="card-header">
              <span class="card-title">XH1 Universal Share Code</span>
              <span class="badge">COMPACT</span>
            </div>
            <div class="code-preview mono">
              {editorState.exportCrosshairCode()}
            </div>
            <div class="card-actions">
              <button class="btn btn-secondary" onclick={copyShareCode}>
                {copyCodeSuccess ? '✓ Copied Code!' : '📋 Copy Share Code'}
              </button>
            </div>
          </div>

          <div class="export-card">
            <div class="card-header">
              <span class="card-title">Lossless .xhair File / JSON</span>
              <span class="badge">SPEC §11.1</span>
            </div>
            <div class="code-preview mono json-preview">
              {editorState.exportCrosshairJson()}
            </div>
            <div class="card-actions">
              <button class="btn btn-secondary" onclick={copyJson}>
                {copyJsonSuccess ? '✓ Copied JSON!' : '📋 Copy JSON'}
              </button>
              <button class="btn btn-primary" onclick={downloadJsonFile}>
                💾 Download .xhair
              </button>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(8, 10, 12, 0.78);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-card {
    background: #12161B;
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    width: 580px;
    max-width: 90vw;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.6);
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    background: rgba(18, 22, 27, 0.6);
  }

  .title-group {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .title {
    font-size: 15px;
    color: var(--text);
  }

  .btn-close {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 16px;
    cursor: pointer;
    padding: 4px;
    border-radius: var(--radius-sm);
  }

  .btn-close:hover {
    color: var(--text);
  }

  .tab-bar {
    display: flex;
    border-bottom: 1px solid var(--border);
    background: rgba(14, 18, 23, 0.8);
    padding: 0 16px;
    gap: 8px;
  }

  .tab-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 13px;
    font-weight: 500;
    padding: 12px 14px;
    cursor: pointer;
    border-bottom: 2px solid transparent;
  }

  .tab-btn:hover {
    color: var(--text);
  }

  .tab-btn.active {
    color: var(--accent);
    border-bottom-color: var(--accent);
  }

  .modal-content {
    padding: 20px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .section-desc {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
  }

  .section-desc code {
    background: rgba(255, 255, 255, 0.08);
    padding: 2px 5px;
    border-radius: 3px;
    color: var(--accent);
  }

  .textarea-wrapper {
    position: relative;
    display: flex;
    flex-direction: column;
  }

  .code-textarea {
    width: 100%;
    background: #090B0E;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 12px;
    color: var(--text);
    font-size: 12px;
    resize: vertical;
    box-sizing: border-box;
  }

  .code-textarea:focus {
    outline: none;
    border-color: var(--accent);
  }

  .detected-badge {
    margin-top: 6px;
    font-size: 11px;
    color: var(--accent);
  }

  .upload-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .file-upload-btn {
    cursor: pointer;
    font-size: 12px;
  }

  .file-hint {
    font-size: 11px;
    color: var(--text-muted);
  }

  .feedback {
    padding: 10px 14px;
    border-radius: var(--radius-md);
    font-size: 12px;
  }

  .feedback.success {
    background: rgba(54, 242, 203, 0.15);
    color: var(--accent);
    border: 1px solid var(--accent);
  }

  .feedback.error {
    background: rgba(255, 56, 100, 0.15);
    color: #FF3864;
    border: 1px solid #FF3864;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 8px;
  }

  .export-card {
    background: #090B0E;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .card-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
  }

  .code-preview {
    background: rgba(18, 22, 27, 0.8);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 8px 12px;
    font-size: 11px;
    color: var(--text-muted);
    word-break: break-all;
    max-height: 80px;
    overflow-y: auto;
  }

  .json-preview {
    max-height: 120px;
    white-space: pre;
    word-break: normal;
  }

  .card-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
</style>
