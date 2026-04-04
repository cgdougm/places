<script lang="ts">
  import type { Project, PlaceItem } from '$lib/types';
  import ItemCard from './ItemCard.svelte';
  import AddItemModal from './AddItemModal.svelte';
  import FilePreview from './FilePreview.svelte';

  let { project }: { project: Project } = $props();

  let showAdd = $state(false);
  let filter = $state<'all' | 'file' | 'url' | 'text'>('all');
  let previewItem = $state<PlaceItem | null>(null);

  const filtered = $derived(
    filter === 'all' ? project.items : project.items.filter(i => i.type === filter)
  );
</script>

<div class="panel">
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
    <div class="preview-strip">
      <div class="preview-header">
        <span>Preview: {previewItem.label}</span>
        <button class="icon-btn" onclick={() => previewItem = null}>✕</button>
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

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  gap: 12px;
  flex-wrap: wrap;
}

.project-title {
  display: flex;
  align-items: center;
  gap: 10px;
}

.dot {
  width: 12px; height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
}

h1 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.count {
  font-size: 12px;
  color: var(--text-muted);
  background: var(--badge-bg);
  padding: 2px 8px;
  border-radius: 10px;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

.filter-tabs {
  display: flex;
  gap: 3px;
  background: var(--sidebar-bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 3px;
}

.filter-tab {
  padding: 4px 10px;
  border-radius: 6px;
  border: none;
  background: none;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.12s;
}
.filter-tab.active {
  background: var(--panel-bg);
  color: var(--text);
  box-shadow: 0 1px 3px rgba(0,0,0,0.2);
}

.add-btn {
  padding: 7px 14px;
  border: none;
  border-radius: 8px;
  color: #fff;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: opacity 0.12s;
}
.add-btn:hover { opacity: 0.85; }

.content {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
}

.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 14px;
  height: 100%;
  color: var(--text-muted);
}

.items-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 10px;
  align-content: start;
}

.preview-strip {
  border-top: 1px solid var(--border);
  height: 280px;
  display: flex;
  flex-direction: column;
  background: var(--sidebar-bg);
}

.preview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 12px;
  border-bottom: 1px solid var(--border);
  font-size: 12px;
  color: var(--text-muted);
}

.icon-btn {
  background: none; border: none; cursor: pointer;
  color: var(--text-muted); padding: 2px 5px; border-radius: 4px; font-size: 13px;
}
.icon-btn:hover { color: var(--text); background: var(--hover-bg); }
</style>
