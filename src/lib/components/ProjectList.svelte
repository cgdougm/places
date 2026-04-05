<script lang="ts">
  import {
    getProjects, getActiveId, selectProject,
    addProject, renameProject, deleteProject, setProjectColor
  } from '$lib/stores/projects.svelte';
  import { DEFAULT_COLORS } from '$lib/types';

  let newName = $state('');
  let adding = $state(false);
  let editingId = $state<string | null>(null);
  let editName = $state('');
  let colorPickId = $state<string | null>(null);

  function startAdd() { adding = true; newName = ''; }

  async function confirmAdd() {
    const name = newName.trim();
    if (name) await addProject(name);
    adding = false;
    newName = '';
  }

  function startEdit(id: string, name: string) {
    editingId = id;
    editName = name;
    colorPickId = null;
  }

  async function confirmEdit() {
    if (editingId && editName.trim()) {
      await renameProject(editingId, editName.trim());
    }
    editingId = null;
  }

  async function handleDelete(id: string) {
    await deleteProject(id);
  }
</script>

<aside class="project-list">
  <div class="list-header">
    <span class="section-label">Projects</span>
    <button class="icon-btn" onclick={startAdd} title="New project">+</button>
  </div>

  <ul class="projects">
    {#each getProjects() as project (project.id)}
      <li
        class="project-item"
        class:active={project.id === getActiveId()}
        onclick={() => { selectProject(project.id); colorPickId = null; editingId = null; }}
      >
        <span class="project-dot" style="background:{project.color}"></span>

        {#if editingId === project.id}
          <input
            class="inline-edit"
            bind:value={editName}
            onblur={confirmEdit}
            onkeydown={e => e.key === 'Enter' && confirmEdit()}
            autofocus
          />
        {:else}
          <span class="project-name">{project.name}</span>
        {/if}

        <span class="item-count">{project.items.length}</span>

        <div class="project-actions">
          <button class="icon-btn small" title="Rename"
            onclick={e => { e.stopPropagation(); startEdit(project.id, project.name); }}>✎</button>
          <button class="icon-btn small" title="Color"
            onclick={e => { e.stopPropagation(); colorPickId = colorPickId === project.id ? null : project.id; }}>●</button>
          <button class="icon-btn small danger" title="Delete"
            onclick={e => { e.stopPropagation(); handleDelete(project.id); }}>✕</button>
        </div>
      </li>

      {#if colorPickId === project.id}
        <li class="color-picker">
          {#each DEFAULT_COLORS as c}
            <button
              class="swatch"
              style="background:{c}; outline: {c === project.color ? '2px solid white' : 'none'}"
              onclick={async (e) => { e.stopPropagation(); await setProjectColor(project.id, c); colorPickId = null; }}
            ></button>
          {/each}
        </li>
      {/if}
    {/each}
  </ul>

  {#if adding}
    <div class="add-row">
      <input
        class="new-project-input"
        placeholder="Project name…"
        bind:value={newName}
        onblur={confirmAdd}
        onkeydown={e => e.key === 'Enter' && confirmAdd()}
        autofocus
      />
    </div>
  {/if}
</aside>

<style>
.project-list {
  width: 220px;
  min-width: 160px;
  background: var(--sidebar-bg);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 12px 8px;
  border-bottom: 1px solid var(--border);
}

.section-label {
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--text-muted);
}

.projects {
  list-style: none;
  margin: 0;
  padding: 6px 0;
  overflow-y: auto;
  flex: 1;
}

.project-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 7px 10px 7px 12px;
  cursor: pointer;
  border-radius: 6px;
  margin: 1px 6px;
  position: relative;
  transition: background 0.12s;
}

.project-item:hover { background: var(--hover-bg); }
.project-item.active { background: var(--active-bg); }

.project-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.project-name {
  flex: 1;
  font-size: 13px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--text);
}

.item-count {
  font-size: 11px;
  color: var(--text-muted);
  background: var(--badge-bg);
  padding: 1px 6px;
  border-radius: 10px;
}

.project-actions {
  display: none;
  gap: 2px;
}
.project-item:hover .project-actions,
.project-item.active .project-actions {
  display: flex;
}

.inline-edit {
  flex: 1;
  font-size: 13px;
  background: var(--input-bg);
  border: 1px solid var(--accent);
  border-radius: 4px;
  padding: 2px 5px;
  color: var(--text);
}

.color-picker {
  list-style: none;
  display: flex;
  gap: 6px;
  padding: 6px 14px;
  flex-wrap: wrap;
}

.swatch {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: none;
  cursor: pointer;
  outline-offset: 2px;
}

.add-row {
  padding: 8px 10px;
  border-top: 1px solid var(--border);
}

.new-project-input {
  width: 100%;
  font-size: 13px;
  background: var(--input-bg);
  border: 1px solid var(--accent);
  border-radius: 6px;
  padding: 5px 8px;
  color: var(--text);
  box-sizing: border-box;
}

.icon-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--text-muted);
  padding: 2px 4px;
  border-radius: 4px;
  font-size: 14px;
  line-height: 1;
  transition: color 0.12s, background 0.12s;
}
.icon-btn:hover { color: var(--text); background: var(--hover-bg); }
.icon-btn.small { font-size: 11px; padding: 1px 3px; }
.icon-btn.danger:hover { color: #e05c4a; }
</style>
