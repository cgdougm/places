<script lang="ts">
  import type { Project, PlaceItem } from '$lib/types';
  import ItemCard from './ItemCard.svelte';
  import AddItemModal from './AddItemModal.svelte';
  import FilePreview from './FilePreview.svelte';

  let { project }: { project: Project } = $props();

  let showAdd    = $state(false);
  let filter     = $state<'all' | 'file' | 'url' | 'text'>('all');
  let previewItem = $state<PlaceItem | null>(null);

  const filtered = $derived(
    filter === 'all' ? project.items : project.items.filter(i => i.type === filter)
  );

  // ── Preview strip drag-resize ─────────────────────────────────────────────────
  const MIN_PREVIEW = 80;
  const MAX_PREVIEW = 600;

  let previewH  = $state(280);
  let dragging  = $state(false);
  let dragStartY = 0;
  let dragStartH = 0;

  function startDrag(e: MouseEvent) {
    dragging   = true;
    dragStartY = e.clientY;
    dragStartH = previewH;
    e.preventDefault();
  }

  function handleMouseMove(e: MouseEvent) {
    if (!dragging) return;
    const delta = dragStartY - e.clientY; // dragging up = bigger preview
    previewH = Math.max(MIN_PREVIEW, Math.min(MAX_PREVIEW, dragStartH + delta));
  }

  function stopDrag() { dragging = false; }
</script>

<svelte:window onmousemove={handleMouseMove} onmouseup={stopDrag} />

<div class="panel" class:dragging-v={dragging}>
  <div class="panel-header">
    <div class="project-title">
      <span class="dot" style="background:{project.color}"></span>
      <h1>{project.name}</h1>
      <span class="count">{project.items.length} items</span>
    </div>

    <div class="header-actions">
      <div class="filter-tabs">
        {#each ['all', 'file', 'url', 'text'] as f}
          <button class="filter-tab" class:active={filter === f}
            onclick={() => filter = f as typeof filter}>
            {f === 'all' ? 'All' : f === 'file' ? '📄' : f === 'url' ? '🔗' : '📝'}
          </button>
        {/each}
      </div>
      <button class="add-btn" onclick={() => showAdd = true}
        style="background:{project.color}">+ Add item</button>
    </div>
  </div>

  <div class="content">
    {#if filtered.length === 0}
      <div class="empty">
        <p>No items yet.</p>
        <button class="add-btn" onclick={() => showAdd = true}
          style="background:{project.color}">+ Add your first item</button>
      </div>
    {:else}
      <div class="items-grid">
        {#each filtered as item (item.id)}
          <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
          <div onclick={() => previewItem = previewItem?.id === item.id ? null : item}>
            <ItemCard {item} {project} />
          </div>
        {/each}
      </div>
    {/if}
  </div>

  {#if previewItem && previewItem.type === 'file'}
    <!-- Horizontal splitter -->
    <div class="h-splitter" onmousedown={startDrag} title="Drag to resize preview">
      <div class="h-track"></div>
      <button
        class="collapse-btn"
        onclick={() => previewItem = null}
        title="Close preview"
      >✕</button>
    </div>

    <div class="preview-strip" style="height:{previewH}px">
      <div class="preview-header">
        <span>Preview: {previewItem.label}</span>
      </div>
      <FilePreview path={previewItem.value} />
    </div>
  {/if}
</div>

{#if showAdd}
  <AddItemModal {project} onclose={() => showAdd = false} />
{/if}

<style>
.panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.panel.dragging-v { cursor: row-resize; user-select: none; }

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  gap: 12px;
  flex-wrap: wrap;
  flex-shrink: 0;
}

.project-title { display: flex; align-items: center; gap: 10px; }

.dot { width: 12px; height: 12px; border-radius: 50%; flex-shrink: 0; }

h1 { margin: 0; font-size: 18px; font-weight: 600; }

.count {
  font-size: 12px; color: var(--text-muted);
  background: var(--badge-bg); padding: 2px 8px; border-radius: 10px;
}

.header-actions { display: flex; align-items: center; gap: 10px; }

.filter-tabs {
  display: flex; gap: 3px;
  background: var(--sidebar-bg); border: 1px solid var(--border);
  border-radius: 8px; padding: 3px;
}
.filter-tab {
  padding: 4px 10px; border-radius: 6px; border: none;
  background: none; color: var(--text-muted); cursor: pointer;
  font-size: 12px; transition: all 0.12s;
}
.filter-tab.active {
  background: var(--panel-bg); color: var(--text);
  box-shadow: 0 1px 3px rgba(0,0,0,0.2);
}

.add-btn {
  padding: 7px 14px; border: none; border-radius: 8px;
  color: #fff; cursor: pointer; font-size: 13px; font-weight: 500;
  transition: opacity 0.12s;
}
.add-btn:hover { opacity: 0.85; }

.content { flex: 1; overflow-y: auto; padding: 16px 20px; min-height: 0; }

.empty {
  display: flex; flex-direction: column; align-items: center;
  justify-content: center; gap: 14px; height: 100%; color: var(--text-muted);
}

.items-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 10px; align-content: start;
}

/* ── Horizontal splitter ──────────────────────────────────────────────────── */
.h-splitter {
  height: 8px;
  background: var(--sidebar-bg);
  border-top: 1px solid var(--border);
  border-bottom: 1px solid var(--border);
  cursor: row-resize;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  position: relative;
  transition: background 0.1s;
}
.h-splitter:hover { background: var(--hover-bg); }

.h-track {
  width: 40px; height: 2px;
  background: var(--border); border-radius: 2px;
}
.h-splitter:hover .h-track { background: var(--accent); }

.collapse-btn {
  position: absolute;
  right: 10px;
  top: 50%; transform: translateY(-50%);
  width: 18px; height: 18px;
  background: var(--card-bg); border: 1px solid var(--border);
  border-radius: 4px; color: var(--text-muted);
  cursor: pointer; font-size: 9px;
  display: flex; align-items: center; justify-content: center;
  padding: 0; opacity: 0; transition: opacity 0.15s, color 0.1s;
}
.h-splitter:hover .collapse-btn { opacity: 1; }
.collapse-btn:hover { color: var(--text); background: var(--active-bg); }

/* ── Preview strip ────────────────────────────────────────────────────────── */
.preview-strip {
  display: flex; flex-direction: column;
  background: var(--sidebar-bg);
  flex-shrink: 0;
  min-height: 0;
  overflow: hidden;
}

.preview-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 5px 12px;
  border-bottom: 1px solid var(--border);
  font-size: 12px; color: var(--text-muted);
  flex-shrink: 0;
}

.icon-btn {
  background: none; border: none; cursor: pointer;
  color: var(--text-muted); padding: 2px 5px; border-radius: 4px; font-size: 13px;
}
.icon-btn:hover { color: var(--text); background: var(--hover-bg); }
</style>
