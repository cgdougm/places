<script lang="ts">
  import type { PlaceItem, Project } from '$lib/types';
  import { updateItem, deleteItem } from '$lib/stores/projects.svelte';
  import { requestJump } from '$lib/stores/browser.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { basename } from '$lib/utils/paths';

  let { item, project }: { item: PlaceItem; project: Project } = $props();

  let editing = $state(false);
  let editLabel = $state('');
  let editNotes = $state('');

  function startEdit() {
    editLabel = item.label;
    editNotes = item.notes;
    editing = true;
  }

  async function saveEdit() {
    await updateItem(project.id, item.id, { label: editLabel.trim() || item.label, notes: editNotes });
    editing = false;
  }

  async function open() {
    if (item.type === 'file') {
      await invoke('open_path', { path: item.value }).catch(() => {});
    } else if (item.type === 'url') {
      await invoke('open_url', { url: item.value }).catch(() => {});
    }
  }

  async function reveal() {
    if (item.type === 'file') {
      await invoke('reveal_path', { path: item.value }).catch(() => {});
    }
  }

  const typeIcon = $derived(
    item.type === 'file' ? '📄' :
    item.type === 'url'  ? '🔗' :
                           '📝'
  );

  const displayValue = $derived(
    item.type === 'file' ? basename(item.value) :
    item.type === 'url'  ? item.value.replace(/^https?:\/\//, '').split('/')[0] :
                           item.value.slice(0, 60)
  );
</script>

<div class="card" style="--accent:{project.color}">
  <div class="card-header">
    <span class="type-icon">{typeIcon}</span>
    {#if editing}
      <input class="edit-label" bind:value={editLabel} autofocus
        onkeydown={e => e.key === 'Enter' && saveEdit()} />
    {:else}
      <span class="card-label" ondblclick={startEdit}>{item.label}</span>
    {/if}
    <div class="card-actions">
      {#if item.type !== 'text'}
        <button class="icon-btn" onclick={open} title="Open">▶</button>
      {/if}
      {#if item.type === 'file'}
        <button class="icon-btn" onclick={reveal} title="Reveal in explorer">📂</button>
        <button class="icon-btn" onclick={() => requestJump(item.value)} title="Jump to in file browser">🎯</button>
      {/if}
      <button class="icon-btn" onclick={startEdit} title="Edit">✎</button>
      <button class="icon-btn danger" onclick={() => deleteItem(project.id, item.id)} title="Remove">✕</button>
    </div>
  </div>

  <div class="card-value" title={item.value}>{displayValue}</div>

  {#if editing}
    <textarea class="edit-notes" bind:value={editNotes} placeholder="Notes…" rows="2"
      onblur={saveEdit}></textarea>
  {:else if item.notes}
    <div class="card-notes">{item.notes}</div>
  {/if}
</div>

<style>
.card {
  background: var(--card-bg);
  border: 1px solid var(--border);
  border-left: 3px solid var(--accent);
  border-radius: 8px;
  padding: 10px 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  transition: box-shadow 0.15s;
}
.card:hover { box-shadow: 0 2px 8px rgba(0,0,0,0.25); }

.card-header {
  display: flex;
  align-items: center;
  gap: 6px;
}

.type-icon { font-size: 14px; flex-shrink: 0; }

.card-label {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--text);
}

.card-value {
  font-size: 11px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  padding-left: 20px;
}

.card-notes {
  font-size: 11px;
  color: var(--text-secondary);
  padding-left: 20px;
  white-space: pre-wrap;
  word-break: break-word;
}

.card-actions {
  display: none;
  gap: 2px;
  align-items: center;
}
.card:hover .card-actions { display: flex; }

.edit-label {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  background: var(--input-bg);
  border: 1px solid var(--accent);
  border-radius: 4px;
  padding: 2px 6px;
  color: var(--text);
}

.edit-notes {
  font-size: 11px;
  background: var(--input-bg);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 4px 6px;
  color: var(--text);
  resize: none;
  font-family: inherit;
  margin-top: 2px;
}

.icon-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--text-muted);
  padding: 2px 4px;
  border-radius: 4px;
  font-size: 12px;
  line-height: 1;
  transition: color 0.12s;
}
.icon-btn:hover { color: var(--text); }
.icon-btn.danger:hover { color: #e05c4a; }
</style>
