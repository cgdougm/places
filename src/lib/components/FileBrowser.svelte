<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open as dialogOpen } from '@tauri-apps/plugin-dialog';
  import type { FileEntry, Project } from '$lib/types';
  import { addItem } from '$lib/stores/projects.svelte';
  import { basename, dirname } from '$lib/utils/paths';

  let { project }: { project: Project | null } = $props();

  let currentPath = $state('');
  let entries = $state<FileEntry[]>([]);
  let loading = $state(false);
  let history = $state<string[]>([]);
  let histIdx = $state(-1);

  async function navigate(path: string) {
    if (!path) return;
    loading = true;
    try {
      entries = await invoke<FileEntry[]>('list_dir', { path });
      // Trim forward history when navigating a new path
      history = [...history.slice(0, histIdx + 1), path];
      histIdx = history.length - 1;
      currentPath = path;
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  async function init() {
    const home = await invoke<string>('get_home_dir');
    navigate(home);
  }

  $effect(() => { init(); });

  function goBack() { if (histIdx > 0) navigate(history[histIdx - 1]); }
  function goForward() { if (histIdx < history.length - 1) navigate(history[histIdx + 1]); }
  function goUp() { if (currentPath) navigate(dirname(currentPath)); }

  async function pickFolder() {
    const sel = await dialogOpen({ directory: true, multiple: false });
    if (sel) navigate(typeof sel === 'string' ? sel : sel[0]);
  }

  async function linkToProject(entry: FileEntry) {
    if (!project) return;
    await addItem(project.id, {
      type: 'file',
      label: basename(entry.path),
      value: entry.path,
      notes: '',
    });
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

<div class="browser">
  <div class="toolbar">
    <button class="nav-btn" disabled={histIdx <= 0} onclick={goBack} title="Back">‹</button>
    <button class="nav-btn" disabled={histIdx >= history.length - 1} onclick={goForward} title="Forward">›</button>
    <button class="nav-btn" onclick={goUp} title="Up">↑</button>
    <input
      class="path-input"
      value={currentPath}
      onchange={e => navigate((e.target as HTMLInputElement).value)}
      onkeydown={e => e.key === 'Enter' && navigate((e.target as HTMLInputElement).value)}
    />
    <button class="nav-btn" onclick={pickFolder} title="Pick folder">…</button>
  </div>

  <div class="entry-list">
    {#if loading}
      <div class="loading">Loading…</div>
    {:else}
      {#each entries as entry (entry.path)}
        <div class="entry" ondblclick={() => entry.is_dir && navigate(entry.path)}>
          <span class="entry-icon">{entry.is_dir ? '📁' : '📄'}</span>
          <span class="entry-name">{entry.name}</span>
          {#if !entry.is_dir}
            <span class="entry-size">{formatSize(entry.size)}</span>
          {/if}
          {#if project}
            <button class="link-btn" title="Link to {project.name}"
              onclick={e => { e.stopPropagation(); linkToProject(entry); }}>
              + link
            </button>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
.browser {
  width: 280px;
  min-width: 200px;
  border-left: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  background: var(--sidebar-bg);
}

.toolbar {
  display: flex;
  align-items: center;
  gap: 3px;
  padding: 6px 8px;
  border-bottom: 1px solid var(--border);
}

.nav-btn {
  background: none; border: 1px solid var(--border); border-radius: 5px;
  color: var(--text-muted); cursor: pointer; padding: 3px 7px; font-size: 14px;
  transition: all 0.1s; line-height: 1;
}
.nav-btn:hover:not(:disabled) { color: var(--text); background: var(--hover-bg); }
.nav-btn:disabled { opacity: 0.3; cursor: default; }

.path-input {
  flex: 1; font-size: 11px;
  background: var(--input-bg); border: 1px solid var(--border);
  border-radius: 5px; padding: 4px 7px; color: var(--text);
  min-width: 0;
}
.path-input:focus { border-color: var(--accent); outline: none; }

.entry-list {
  flex: 1; overflow-y: auto;
  padding: 4px 0;
}

.loading {
  color: var(--text-muted); font-size: 12px; padding: 12px;
}

.entry {
  display: flex; align-items: center; gap: 6px;
  padding: 4px 10px; cursor: default;
  transition: background 0.1s; font-size: 12px;
  border-radius: 4px; margin: 1px 4px;
}
.entry:hover { background: var(--hover-bg); }

.entry-icon { font-size: 13px; flex-shrink: 0; }
.entry-name { flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.entry-size { font-size: 10px; color: var(--text-muted); flex-shrink: 0; }

.link-btn {
  display: none; font-size: 10px; padding: 1px 6px;
  border-radius: 4px; border: 1px solid var(--border);
  background: var(--card-bg); color: var(--text-muted); cursor: pointer;
}
.entry:hover .link-btn { display: block; }
.link-btn:hover { color: var(--accent); border-color: var(--accent); }
</style>
