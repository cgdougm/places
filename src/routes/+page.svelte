<script lang="ts">
  import { getProjects, getActiveProject } from '$lib/stores/projects.svelte';
  import ProjectList from '$lib/components/ProjectList.svelte';
  import ProjectPanel from '$lib/components/ProjectPanel.svelte';
  import FileBrowser from '$lib/components/FileBrowser.svelte';
  import PlacesLogo from '$lib/components/PlacesLogo.svelte';

  // ── Pane widths ───────────────────────────────────────────────────────────────
  let sidebarW    = $state(220);
  let browserW    = $state(280);
  let sidebarOpen = $state(true);
  let browserOpen = $state(true);

  const MIN_W = 140;
  const MAX_SIDEBAR = 400;
  const MAX_BROWSER = 520;

  // ── Drag state ────────────────────────────────────────────────────────────────
  type DragSide = 'sidebar' | 'browser' | null;
  let dragging: DragSide = $state(null);
  let dragStartX = 0;
  let dragStartW = 0;

  function startDrag(e: MouseEvent, side: DragSide) {
    dragging = side;
    dragStartX = e.clientX;
    dragStartW = side === 'sidebar' ? sidebarW : browserW;
    e.preventDefault();
  }

  function handleMouseMove(e: MouseEvent) {
    if (!dragging) return;
    const delta = e.clientX - dragStartX;
    if (dragging === 'sidebar') {
      sidebarW = Math.max(MIN_W, Math.min(MAX_SIDEBAR, dragStartW + delta));
    } else {
      // Browser is on the right, so dragging left increases width
      browserW = Math.max(MIN_W, Math.min(MAX_BROWSER, dragStartW - delta));
    }
  }

  function stopDrag() { dragging = null; }
</script>

<svelte:window onmousemove={handleMouseMove} onmouseup={stopDrag} />

<div class="workspace" class:dragging-h={dragging !== null}>

  <!-- Sidebar pane -->
  {#if sidebarOpen}
    <div class="pane sidebar-pane" style="width:{sidebarW}px">
      <ProjectList />
    </div>
  {:else}
    <div class="collapsed-strip left" onclick={() => sidebarOpen = true} title="Show projects">
      <span class="strip-label">Projects</span>
    </div>
  {/if}

  <!-- Sidebar splitter -->
  <div
    class="splitter"
    onmousedown={e => startDrag(e, 'sidebar')}
    title="Drag to resize"
  >
    <div class="splitter-track"></div>
    <button
      class="collapse-btn"
      onclick={() => sidebarOpen = !sidebarOpen}
      title={sidebarOpen ? 'Collapse sidebar' : 'Expand sidebar'}
    >{sidebarOpen ? '◀' : '▶'}</button>
  </div>

  <!-- Main panel -->
  <div class="pane main-pane">
    {#if getActiveProject() !== null}
      <ProjectPanel project={getActiveProject()!} />
    {:else if getProjects().length === 0}
      <div class="empty-state">
        <div class="logo-wrap" style="color:var(--accent)">
          <PlacesLogo size={120} />
        </div>
        <h2>Welcome to Places</h2>
        <p>Create a project in the sidebar to start organising your files, URLs, and notes.</p>
      </div>
    {:else}
      <div class="empty-state">
        <div class="logo-wrap" style="color:var(--accent); opacity:0.3">
          <PlacesLogo size={80} />
        </div>
        <p>Select a project from the sidebar.</p>
      </div>
    {/if}
  </div>

  <!-- Browser splitter -->
  <div
    class="splitter"
    onmousedown={e => startDrag(e, 'browser')}
    title="Drag to resize"
  >
    <div class="splitter-track"></div>
    <button
      class="collapse-btn"
      onclick={() => browserOpen = !browserOpen}
      title={browserOpen ? 'Collapse browser' : 'Expand browser'}
    >{browserOpen ? '▶' : '◀'}</button>
  </div>

  <!-- Browser pane -->
  {#if browserOpen}
    <div class="pane browser-pane" style="width:{browserW}px">
      <FileBrowser project={getActiveProject()} />
    </div>
  {:else}
    <div class="collapsed-strip right" onclick={() => browserOpen = true} title="Show file browser">
      <span class="strip-label">Files</span>
    </div>
  {/if}

</div>

<style>
.workspace {
  display: flex;
  flex: 1;
  overflow: hidden;
}
.workspace.dragging-h { cursor: col-resize; user-select: none; }

/* Panes */
.pane {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  flex-shrink: 0;
}
.sidebar-pane { border-right: none; }
.main-pane    { flex: 1; background: var(--panel-bg); }
.browser-pane { border-left: none; }

/* Splitter */
.splitter {
  width: 8px;
  background: var(--sidebar-bg);
  border-left: 1px solid var(--border);
  border-right: 1px solid var(--border);
  cursor: col-resize;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  flex-shrink: 0;
  position: relative;
  transition: background 0.1s;
}
.splitter:hover { background: var(--hover-bg); }

.splitter-track {
  width: 2px;
  height: 40px;
  background: var(--border);
  border-radius: 2px;
}
.splitter:hover .splitter-track { background: var(--accent); }

.collapse-btn {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 16px;
  height: 28px;
  background: var(--card-bg);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  opacity: 0;
  transition: opacity 0.15s, color 0.1s;
  z-index: 2;
}
.splitter:hover .collapse-btn { opacity: 1; }
.collapse-btn:hover { color: var(--text); background: var(--active-bg); }

/* Collapsed strips */
.collapsed-strip {
  width: 20px;
  background: var(--sidebar-bg);
  border-right: 1px solid var(--border);
  border-left: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  flex-shrink: 0;
  transition: background 0.1s;
}
.collapsed-strip:hover { background: var(--hover-bg); }

.strip-label {
  writing-mode: vertical-rl;
  text-orientation: mixed;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--text-muted);
}

/* Empty / welcome state */
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  color: var(--text-muted);
  padding: 40px;
  text-align: center;
}
.logo-wrap { line-height: 0; }
.empty-state h2 { margin: 0; font-size: 20px; color: var(--text); }
.empty-state p  { margin: 0; font-size: 14px; max-width: 320px; line-height: 1.5; }
</style>
