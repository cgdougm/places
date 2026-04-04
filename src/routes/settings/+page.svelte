<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let storeLocation = $state('');
  let storeKind = $state('local_json');

  // Cloud account inputs (for future use)
  let gdEmail = $state('');
  let dropboxEmail = $state('');

  // Status
  let saving = $state(false);
  let status = $state('');

  onMount(async () => {
    try {
      storeLocation = await invoke<string>('get_store_location');
    } catch {}
  });

  function setStatus(msg: string) {
    status = msg;
    setTimeout(() => status = '', 3000);
  }

  async function saveCloudAccount(provider: string) {
    saving = true;
    // TODO: trigger OAuth flow via Tauri + shell
    setStatus(`${provider} OAuth not yet implemented. Stub saved.`);
    saving = false;
  }
</script>

<div class="page">
  <h1 class="page-title">Settings</h1>

  <!-- ── Storage ──────────────────────────────────────────────────────────────── -->
  <section class="card">
    <h2>Storage</h2>
    <p class="desc">Places currently saves your data to a local JSON file. Future versions will support cloud sync.</p>

    <div class="field-row">
      <label>Active backend</label>
      <div class="backend-pills">
        <span class="pill active">Local JSON</span>
        <span class="pill disabled" title="Not yet implemented">Google Drive</span>
        <span class="pill disabled" title="Not yet implemented">Dropbox</span>
        <span class="pill disabled" title="Not yet implemented">SQLite</span>
      </div>
    </div>

    <div class="field-row">
      <label>Data file</label>
      <code class="path">{storeLocation || '—'}</code>
    </div>
  </section>

  <!-- ── Google Drive ──────────────────────────────────────────────────────────── -->
  <section class="card disabled-section">
    <div class="section-badge">Coming soon</div>
    <h2>Google Drive</h2>
    <p class="desc">
      Places can store <code>places-data.json</code> in your Google Drive hidden AppData folder
      so it's backed up and accessible across devices. Requires a Google account.
    </p>
    <div class="field-row">
      <label>Account email</label>
      <input bind:value={gdEmail} placeholder="you@gmail.com" disabled />
    </div>
    <button class="btn primary" disabled onclick={() => saveCloudAccount('Google Drive')}>
      Connect Google account
    </button>
  </section>

  <!-- ── Dropbox ───────────────────────────────────────────────────────────────── -->
  <section class="card disabled-section">
    <div class="section-badge">Coming soon</div>
    <h2>Dropbox</h2>
    <p class="desc">
      Stores <code>/Apps/Places/data.json</code> in your Dropbox so it syncs automatically
      with Dropbox clients on other machines.
    </p>
    <div class="field-row">
      <label>Account email</label>
      <input bind:value={dropboxEmail} placeholder="you@dropbox.com" disabled />
    </div>
    <button class="btn primary" disabled onclick={() => saveCloudAccount('Dropbox')}>
      Connect Dropbox account
    </button>
  </section>

  {#if status}
    <div class="toast">{status}</div>
  {/if}
</div>

<style>
.page {
  flex: 1; overflow-y: auto; padding: 32px 40px;
  display: flex; flex-direction: column; gap: 20px;
  max-width: 680px;
}

.page-title { margin: 0 0 8px; font-size: 22px; font-weight: 600; }

.card {
  background: var(--card-bg);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 20px 24px;
  display: flex; flex-direction: column; gap: 12px;
  position: relative;
}

.card h2 { margin: 0; font-size: 15px; font-weight: 600; }
.desc { margin: 0; font-size: 13px; color: var(--text-muted); line-height: 1.5; }

.disabled-section { opacity: 0.6; }
.section-badge {
  position: absolute; top: 12px; right: 14px;
  font-size: 10px; font-weight: 600; text-transform: uppercase;
  letter-spacing: 0.06em; color: var(--text-muted);
  background: var(--badge-bg); padding: 2px 8px; border-radius: 8px;
}

.field-row {
  display: flex; align-items: center; gap: 12px; flex-wrap: wrap;
}
.field-row label { font-size: 12px; color: var(--text-muted); width: 100px; flex-shrink: 0; }

.backend-pills { display: flex; gap: 6px; flex-wrap: wrap; }
.pill {
  font-size: 11px; padding: 3px 10px; border-radius: 20px;
  border: 1px solid var(--border); background: var(--card-bg); color: var(--text-muted);
}
.pill.active { background: var(--active-bg); color: var(--text); border-color: var(--accent); }
.pill.disabled { opacity: 0.5; }

.path {
  font-family: 'Cascadia Code', 'Fira Code', monospace;
  font-size: 11px; background: var(--code-bg); padding: 4px 8px;
  border-radius: 5px; color: var(--text-secondary); word-break: break-all;
}

input {
  flex: 1; min-width: 200px; padding: 7px 10px;
  background: var(--input-bg); border: 1px solid var(--border);
  border-radius: 6px; color: var(--text); font-size: 13px;
}
input:disabled { opacity: 0.5; cursor: not-allowed; }

.btn {
  padding: 8px 18px; border-radius: 8px; border: none; cursor: pointer;
  font-size: 13px; font-weight: 500; font-family: inherit; transition: opacity 0.12s;
  align-self: flex-start;
}
.btn.primary { background: var(--accent); color: #fff; }
.btn:disabled { opacity: 0.4; cursor: not-allowed; }

.toast {
  position: fixed; bottom: 20px; left: 50%; transform: translateX(-50%);
  background: var(--card-bg); border: 1px solid var(--border); border-radius: 8px;
  padding: 10px 20px; font-size: 13px; box-shadow: 0 4px 20px rgba(0,0,0,0.3);
}
</style>
