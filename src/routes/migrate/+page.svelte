<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let storeLocation = $state('');
  let log = $state<string[]>([]);
  let running = $state(false);

  type Backend = 'local_json' | 'google_drive' | 'dropbox';

  let source = $state<Backend>('local_json');
  let dest   = $state<Backend>('local_json');

  onMount(async () => {
    try {
      storeLocation = await invoke<string>('get_store_location');
    } catch {}
  });

  function addLog(msg: string) {
    log = [...log, `[${new Date().toLocaleTimeString()}] ${msg}`];
  }

  async function runMigration() {
    if (source === dest) {
      addLog('Source and destination are the same — nothing to do.');
      return;
    }
    running = true;
    addLog(`Starting migration: ${source} → ${dest}`);

    try {
      // Step 1: load from source
      addLog('Loading data from source…');
      const raw = await invoke<string>('load_app_data');
      const data = JSON.parse(raw);
      addLog(`Loaded ${data.projects?.length ?? 0} projects, ${data.projects?.reduce((n: number, p: any) => n + p.items.length, 0)} items.`);

      // Step 2: (future) switch active store to dest, save
      addLog('⚠ Cloud migration not yet implemented. Data verified in source store.');
      addLog('When cloud backends are available, data will be re-saved to the selected destination.');

      // Step 3: verify
      addLog('Verification would confirm data integrity after write.');
      addLog('Migration plan logged. No data was changed.');
    } catch (e) {
      addLog(`Error: ${e}`);
    } finally {
      running = false;
    }
  }

  async function exportJson() {
    running = true;
    addLog('Exporting data as JSON…');
    try {
      const raw = await invoke<string>('load_app_data');
      const blob = new Blob([raw], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `places-export-${new Date().toISOString().slice(0,10)}.json`;
      a.click();
      URL.revokeObjectURL(url);
      addLog('Export complete.');
    } catch (e) {
      addLog(`Export failed: ${e}`);
    } finally {
      running = false;
    }
  }

  async function importJson() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = async () => {
      const file = input.files?.[0];
      if (!file) return;
      running = true;
      addLog(`Importing ${file.name}…`);
      try {
        const text = await file.text();
        JSON.parse(text); // validate
        await invoke('save_app_data', { json: text });
        addLog('Import successful. Reload the app to see changes.');
      } catch (e) {
        addLog(`Import failed: ${e}`);
      } finally {
        running = false;
      }
    };
    input.click();
  }
</script>

<div class="page">
  <h1 class="page-title">Migration Manager</h1>
  <p class="subtitle">Move your data between storage backends or export/import for backup.</p>

  <!-- ── Backend migration ────────────────────────────────────────────────────── -->
  <section class="card">
    <h2>Backend migration</h2>
    <p class="desc">
      Transfer your data from one storage backend to another. Cloud backends require an account
      configured in Settings first.
    </p>

    <div class="migrate-row">
      <div class="backend-select">
        <label>Source</label>
        <select bind:value={source}>
          <option value="local_json">Local JSON ({storeLocation || '…'})</option>
          <option value="google_drive" disabled>Google Drive (not configured)</option>
          <option value="dropbox" disabled>Dropbox (not configured)</option>
        </select>
      </div>

      <div class="arrow">→</div>

      <div class="backend-select">
        <label>Destination</label>
        <select bind:value={dest}>
          <option value="local_json">Local JSON</option>
          <option value="google_drive" disabled>Google Drive (not configured)</option>
          <option value="dropbox" disabled>Dropbox (not configured)</option>
        </select>
      </div>
    </div>

    <button class="btn primary" onclick={runMigration} disabled={running || source === dest}>
      {running ? 'Running…' : 'Run migration'}
    </button>
  </section>

  <!-- ── Export / Import ──────────────────────────────────────────────────────── -->
  <section class="card">
    <h2>Export / Import</h2>
    <p class="desc">
      Export your entire dataset as a portable JSON file, or import a previously exported file.
      Importing will overwrite current data.
    </p>
    <div class="action-row">
      <button class="btn secondary" onclick={exportJson} disabled={running}>Export JSON</button>
      <button class="btn secondary" onclick={importJson} disabled={running}>Import JSON…</button>
    </div>
  </section>

  <!-- ── Migration log ────────────────────────────────────────────────────────── -->
  {#if log.length > 0}
    <section class="card">
      <h2>Log</h2>
      <div class="log">
        {#each log as line}
          <div class="log-line">{line}</div>
        {/each}
      </div>
    </section>
  {/if}
</div>

<style>
.page {
  flex: 1; overflow-y: auto; padding: 32px 40px;
  display: flex; flex-direction: column; gap: 20px;
  max-width: 680px;
}

.page-title { margin: 0; font-size: 22px; font-weight: 600; }
.subtitle { margin: 0; font-size: 13px; color: var(--text-muted); }

.card {
  background: var(--card-bg); border: 1px solid var(--border);
  border-radius: 12px; padding: 20px 24px;
  display: flex; flex-direction: column; gap: 14px;
}
.card h2 { margin: 0; font-size: 15px; font-weight: 600; }
.desc { margin: 0; font-size: 13px; color: var(--text-muted); line-height: 1.5; }

.migrate-row {
  display: flex; align-items: flex-end; gap: 16px; flex-wrap: wrap;
}

.backend-select {
  display: flex; flex-direction: column; gap: 5px; flex: 1; min-width: 160px;
}
.backend-select label { font-size: 11px; color: var(--text-muted); font-weight: 500; }

select {
  padding: 7px 10px; background: var(--input-bg); border: 1px solid var(--border);
  border-radius: 6px; color: var(--text); font-size: 13px; font-family: inherit;
}
select:focus { outline: none; border-color: var(--accent); }

.arrow { font-size: 20px; color: var(--text-muted); padding-bottom: 4px; }

.action-row { display: flex; gap: 10px; }

.btn {
  padding: 8px 18px; border-radius: 8px; border: none; cursor: pointer;
  font-size: 13px; font-weight: 500; font-family: inherit;
  transition: opacity 0.12s; align-self: flex-start;
}
.btn.primary { background: var(--accent); color: #fff; }
.btn.secondary {
  background: var(--card-bg); border: 1px solid var(--border); color: var(--text);
}
.btn.secondary:hover { background: var(--hover-bg); }
.btn:disabled { opacity: 0.4; cursor: not-allowed; }

.log {
  background: var(--code-bg); border-radius: 6px; padding: 10px 12px;
  font-family: 'Cascadia Code', 'Fira Code', monospace; font-size: 11px;
  color: var(--text-secondary); display: flex; flex-direction: column; gap: 2px;
  max-height: 200px; overflow-y: auto;
}
.log-line { white-space: pre-wrap; }
</style>
