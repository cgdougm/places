<script lang="ts">
  import type { Project, ItemType } from '$lib/types';
  import { addItem } from '$lib/stores/projects.svelte';
  import { open as dialogOpen } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { normalizePath } from '$lib/utils/paths';

  async function pasteClipboard() {
    try {
      const text = await invoke<string>('read_clipboard');
      if (!text) return;
      value = text.trim();
      // Auto-detect type from clipboard content
      if (/^https?:\/\//i.test(value)) {
        type = 'url';
        if (!label) label = new URL(value).hostname;
      } else if (/^[A-Za-z]:[/\\]|^\/[a-zA-Z]\/|^\/home\/|^\/usr\//.test(value)) {
        type = 'file';
        if (!label) label = value.replace(/[/\\]+$/, '').split(/[/\\]/).pop() ?? value;
      } else {
        type = 'text';
      }
    } catch {}
  }

  let { project, onclose }: { project: Project; onclose: () => void } = $props();

  let type = $state<ItemType>('file');
  let label = $state('');
  let value = $state('');
  let notes = $state('');
  let error = $state('');

  async function pickFile() {
    const selected = await dialogOpen({ multiple: false, directory: false });
    if (selected) {
      value = typeof selected === 'string' ? selected : selected[0];
      if (!label) label = value.replace(/[/\\]+$/, '').split(/[/\\]/).pop() ?? value;
    }
  }

  async function pickDir() {
    const selected = await dialogOpen({ multiple: false, directory: true });
    if (selected) {
      value = typeof selected === 'string' ? selected : selected[0];
      if (!label) label = value.replace(/[/\\]+$/, '').split(/[/\\]/).pop() ?? value;
    }
  }

  async function submit() {
    if (!value.trim()) { error = 'Value is required.'; return; }
    const finalLabel = label.trim() || value.trim().split(/[/\\]/).pop() || value.trim();
    await addItem(project.id, {
      type,
      label: finalLabel,
      value: type === 'file' ? normalizePath(value.trim()) : value.trim(),
      notes: notes.trim(),
    });
    onclose();
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Escape') onclose();
    if (e.key === 'Enter' && e.ctrlKey) submit();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onclick={onclose} onkeydown={handleKey}>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal" onclick={e => e.stopPropagation()}>
    <h2 class="modal-title">Add item to <span style="color:{project.color}">{project.name}</span></h2>

    <div class="type-tabs">
      {#each (['file', 'url', 'text'] as ItemType[]) as t}
        <button class="tab" class:active={type === t} onclick={() => { type = t; value = ''; }}>
          {t === 'file' ? '📄 File / Folder' : t === 'url' ? '🔗 URL' : '📝 Text'}
        </button>
      {/each}
    </div>

    <div class="field">
      <label>Label</label>
      <input placeholder="Friendly name (optional)" bind:value={label} />
    </div>

    <div class="field">
      <label>{type === 'file' ? 'Path' : type === 'url' ? 'URL' : 'Content'}</label>
      {#if type === 'text'}
        <textarea bind:value={value} placeholder="Text content…" rows="5"></textarea>
      {:else}
        <div class="value-row">
          <input bind:value={value} placeholder={type === 'file' ? 'C:\\Users\\...' : 'https://...'} />
          {#if type === 'file'}
            <button class="pick-btn" onclick={pickFile}>File…</button>
            <button class="pick-btn" onclick={pickDir}>Folder…</button>
          {/if}
        </div>
      {/if}
    </div>

    <div class="field">
      <label>Notes <span class="optional">(optional)</span></label>
      <textarea bind:value={notes} placeholder="Any notes about this item…" rows="2"></textarea>
    </div>

    {#if error}<p class="error">{error}</p>{/if}

    <div class="modal-footer">
      <button class="btn clipboard" onclick={pasteClipboard} title="Paste from clipboard">📋 From clipboard</button>
      <span style="flex:1"></span>
      <button class="btn secondary" onclick={onclose}>Cancel</button>
      <button class="btn primary" onclick={submit} style="background:{project.color}">Add item</button>
    </div>
  </div>
</div>

<style>
.overlay {
  position: fixed; inset: 0;
  background: rgba(0,0,0,0.55);
  display: flex; align-items: center; justify-content: center;
  z-index: 100;
  backdrop-filter: blur(2px);
}
.modal {
  background: var(--panel-bg);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 24px;
  width: 480px;
  max-width: 95vw;
  display: flex;
  flex-direction: column;
  gap: 14px;
  box-shadow: 0 8px 40px rgba(0,0,0,0.4);
}
.modal-title { margin: 0; font-size: 16px; font-weight: 600; }
.type-tabs { display: flex; gap: 6px; }
.tab {
  flex: 1; padding: 6px; border-radius: 6px; border: 1px solid var(--border);
  background: var(--card-bg); color: var(--text-muted); cursor: pointer;
  font-size: 12px; transition: all 0.12s;
}
.tab.active { background: var(--active-bg); color: var(--text); border-color: var(--accent); }
.field { display: flex; flex-direction: column; gap: 4px; }
.field label { font-size: 12px; color: var(--text-muted); font-weight: 500; }
.optional { font-weight: 400; opacity: 0.6; }
input, textarea {
  background: var(--input-bg); border: 1px solid var(--border);
  border-radius: 6px; padding: 7px 10px; color: var(--text);
  font-size: 13px; font-family: inherit; outline: none;
  transition: border-color 0.12s;
}
input:focus, textarea:focus { border-color: var(--accent); }
textarea { resize: vertical; }
.value-row { display: flex; gap: 6px; align-items: center; }
.value-row input { flex: 1; }
.pick-btn {
  padding: 7px 10px; font-size: 12px; border-radius: 6px;
  border: 1px solid var(--border); background: var(--card-bg);
  color: var(--text); cursor: pointer; white-space: nowrap;
}
.pick-btn:hover { background: var(--hover-bg); }
.error { color: #e05c4a; font-size: 12px; margin: 0; }
.modal-footer { display: flex; justify-content: flex-end; gap: 8px; margin-top: 4px; }
.btn { padding: 8px 18px; border-radius: 8px; border: none; cursor: pointer; font-size: 13px; font-weight: 500; }
.btn.secondary { background: var(--card-bg); border: 1px solid var(--border); color: var(--text); }
.btn.secondary:hover { background: var(--hover-bg); }
.btn.primary { color: #fff; }
.btn.clipboard { background: none; border: 1px solid var(--border); color: var(--text-muted); }
.btn.clipboard:hover { color: var(--text); background: var(--hover-bg); }
</style>
