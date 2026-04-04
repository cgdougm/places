<script lang="ts">
  import { getProjects, getActiveId, activeProject } from '$lib/stores/projects.svelte';
  import ProjectList from '$lib/components/ProjectList.svelte';
  import ProjectPanel from '$lib/components/ProjectPanel.svelte';
  import FileBrowser from '$lib/components/FileBrowser.svelte';

  let showBrowser = $state(true);
</script>

<div class="workspace">
  <ProjectList />

  <div class="main-area">
    {#if activeProject}
      <ProjectPanel project={activeProject} />
    {:else if getProjects().length === 0}
      <div class="empty-state">
        <div class="empty-icon">◈</div>
        <h2>No projects yet</h2>
        <p>Create a project in the sidebar to start organising your files, URLs, and notes.</p>
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-icon">◈</div>
        <p>Select a project from the sidebar.</p>
      </div>
    {/if}
  </div>

  {#if showBrowser}
    <FileBrowser project={activeProject} />
  {/if}

  <button
    class="browser-toggle"
    onclick={() => showBrowser = !showBrowser}
    title={showBrowser ? 'Hide file browser' : 'Show file browser'}
  >
    {showBrowser ? '›' : '‹'}
  </button>
</div>

<style>
.workspace {
  display: flex;
  flex: 1;
  overflow: hidden;
  position: relative;
}

.main-area {
  flex: 1;
  display: flex;
  overflow: hidden;
  background: var(--panel-bg);
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: var(--text-muted);
  padding: 40px;
  text-align: center;
}

.empty-icon {
  font-size: 48px;
  opacity: 0.2;
}

.empty-state h2 { margin: 0; font-size: 18px; color: var(--text); }
.empty-state p  { margin: 0; font-size: 14px; max-width: 320px; }

.browser-toggle {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  background: var(--sidebar-bg);
  border: 1px solid var(--border);
  border-right: none;
  border-radius: 6px 0 0 6px;
  color: var(--text-muted);
  cursor: pointer;
  padding: 8px 4px;
  font-size: 16px;
  line-height: 1;
  z-index: 10;
  transition: all 0.12s;
}
.browser-toggle:hover { color: var(--text); background: var(--hover-bg); }
</style>
