<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open as dialogOpen } from '@tauri-apps/plugin-dialog';
  import type { FileEntry, Project } from '$lib/types';
  import { addItem } from '$lib/stores/projects.svelte';
  import { basename, dirname } from '$lib/utils/paths';
  import {
    getBookmarks, addBookmark, removeBookmark, isBookmarked,
    getJumpPath, clearJump,
  } from '$lib/stores/browser.svelte';

  let { project }: { project: Project | null } = $props();

  // ── Tree state ────────────────────────────────────────────────────────────────
  interface VisibleNode { entry: FileEntry; depth: number; }

  let rootPath = $state('');
  let childCache = $state<Record<string, FileEntry[]>>({});
  let expanded  = $state<Set<string>>(new Set());
  let loading   = $state<Set<string>>(new Set());
  let history   = $state<string[]>([]);
  let histIdx   = $state(-1);

  // Flat list of currently visible tree nodes
  const visibleNodes = $derived.by(() => {
    const result: VisibleNode[] = [];
    function walk(path: string, depth: number) {
      const children = childCache[path] ?? [];
      for (const entry of children) {
        result.push({ entry, depth });
        if (entry.is_dir && expanded.has(entry.path)) {
          walk(entry.path, depth + 1);
        }
      }
    }
    walk(rootPath, 0);
    return result;
  });

  async function loadDir(path: string): Promise<FileEntry[]> {
    if (childCache[path]) return childCache[path];
    loading = new Set([...loading, path]);
    try {
      const entries = await invoke<FileEntry[]>('list_dir', { path });
      childCache = { ...childCache, [path]: entries };
      return entries;
    } finally {
      loading = new Set([...loading].filter(p => p !== path));
    }
  }

  async function setRoot(path: string) {
    if (!path) return;
    await loadDir(path);
    history = [...history.slice(0, histIdx + 1), path];
    histIdx = history.length - 1;
    rootPath = path;
    // Collapse everything under old root
    expanded = new Set();
  }

  async function init() {
    const home = await invoke<string>('get_home_dir');
    setRoot(home);
  }
  $effect(() => { init(); });

  async function toggleExpand(entry: FileEntry) {
    if (!entry.is_dir) return;
    if (expanded.has(entry.path)) {
      // Collapse: remove this and all descendants
      const next = new Set<string>();
      for (const p of expanded) {
        const sep = p.includes('\\') ? '\\' : '/';
        if (p !== entry.path && !p.startsWith(entry.path + sep)) next.add(p);
      }
      expanded = next;
    } else {
      await loadDir(entry.path);
      expanded = new Set([...expanded, entry.path]);
    }
  }

  function goBack()    { if (histIdx > 0) setRoot(history[histIdx - 1]); }
  function goForward() { if (histIdx < history.length - 1) setRoot(history[histIdx + 1]); }
  function goUp()      { if (rootPath) setRoot(dirname(rootPath)); }

  async function pickFolder() {
    const sel = await dialogOpen({ directory: true, multiple: false });
    if (sel) setRoot(typeof sel === 'string' ? sel : sel[0]);
  }

  // ── Jump-to from ItemCard ─────────────────────────────────────────────────────
  $effect(() => {
    const jp = getJumpPath();
    if (!jp) return;
    clearJump();
    const parent = dirname(jp);
    setRoot(parent);
  });

  // ── Bookmarks panel ───────────────────────────────────────────────────────────
  let bookmarksOpen = $state(true);

  function toggleBookmark() {
    if (isBookmarked(rootPath)) removeBookmark(rootPath);
    else addBookmark(rootPath);
  }

  // ── Context menu ──────────────────────────────────────────────────────────────
  let ctxEntry = $state<FileEntry | null>(null);
  let ctxX = $state(0), ctxY = $state(0);
  let browserEl = $state<HTMLElement | null>(null);

  function openCtx(e: MouseEvent, entry: FileEntry) {
    e.preventDefault(); e.stopPropagation();
    ctxEntry = entry;
    // Position relative to the .browser container (which has position:relative),
    // not relative to the individual entry element (what offsetX/Y would give).
    const rect = browserEl?.getBoundingClientRect();
    ctxX = rect ? e.clientX - rect.left : e.offsetX;
    ctxY = rect ? e.clientY - rect.top  : e.offsetY;
  }
  function closeCtx() { ctxEntry = null; }

  async function ctxReveal(entry: FileEntry) {
    await invoke('reveal_path', { path: entry.path }).catch(console.error);
    closeCtx();
  }
  async function ctxOpen(entry: FileEntry) {
    await invoke('open_path', { path: entry.path }).catch(console.error);
    closeCtx();
  }
  async function ctxCopyPath(entry: FileEntry) {
    await navigator.clipboard.writeText(entry.path).catch(console.error);
    closeCtx();
  }
  async function ctxBookmark(entry: FileEntry) {
    if (entry.is_dir) {
      if (isBookmarked(entry.path)) removeBookmark(entry.path);
      else addBookmark(entry.path);
    }
    closeCtx();
  }
  async function ctxLink(entry: FileEntry) {
    if (!project) return;
    await addItem(project.id, { type: 'file', label: basename(entry.path), value: entry.path, notes: '' });
    closeCtx();
  }

  // ── Misc ──────────────────────────────────────────────────────────────────────
  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  const INDENT = 14; // px per depth level
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="browser" bind:this={browserEl} onclick={closeCtx} onkeydown={e => e.key === 'Escape' && closeCtx()}>

  <!-- Toolbar -->
  <div class="toolbar">
    <button class="nav-btn" disabled={histIdx <= 0} onclick={goBack} title="Back">‹</button>
    <button class="nav-btn" disabled={histIdx >= history.length - 1} onclick={goForward} title="Forward">›</button>
    <button class="nav-btn" onclick={goUp} title="Up">↑</button>
    <input
      class="path-input"
      value={rootPath}
      onchange={e => setRoot((e.target as HTMLInputElement).value)}
      onkeydown={e => e.key === 'Enter' && setRoot((e.target as HTMLInputElement).value)}
    />
    <button class="nav-btn" onclick={pickFolder} title="Pick folder">…</button>
    <button
      class="nav-btn bookmark-btn"
      class:bookmarked={isBookmarked(rootPath)}
      onclick={toggleBookmark}
      title={isBookmarked(rootPath) ? 'Remove bookmark' : 'Bookmark this folder'}
    >★</button>
  </div>

  <!-- Bookmarks panel -->
  {#if getBookmarks().length > 0}
    <div class="bookmarks-panel">
      <button class="bookmarks-header" onclick={() => bookmarksOpen = !bookmarksOpen}>
        <span>Bookmarks</span>
        <span class="chevron">{bookmarksOpen ? '▾' : '▸'}</span>
      </button>
      {#if bookmarksOpen}
        <div class="bookmarks-list">
          {#each getBookmarks() as bm (bm)}
            <div class="bookmark-row">
              <button class="bookmark-item" onclick={() => setRoot(bm)} title={bm}>
                ★ {basename(bm) || bm}
              </button>
              <button class="bm-remove" onclick={() => removeBookmark(bm)} title="Remove">✕</button>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}

  <!-- Tree -->
  <div class="entry-list">
    {#if loading.has(rootPath)}
      <div class="loading">Loading…</div>
    {:else if visibleNodes.length === 0}
      <div class="loading">Empty folder</div>
    {:else}
      {#each visibleNodes as node (node.entry.path)}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="entry"
          class:is-dir={node.entry.is_dir}
          style="padding-left:{10 + node.depth * INDENT}px"
          oncontextmenu={e => openCtx(e, node.entry)}
        >
          <!-- Expand toggle -->
          <button
            class="expand-btn"
            class:invisible={!node.entry.is_dir}
            onclick={() => toggleExpand(node.entry)}
            title={node.entry.is_dir ? (expanded.has(node.entry.path) ? 'Collapse' : 'Expand') : ''}
          >
            {#if loading.has(node.entry.path)}
              <span class="spin">⟳</span>
            {:else if expanded.has(node.entry.path)}
              ▾
            {:else}
              ▸
            {/if}
          </button>

          <span class="entry-icon">{node.entry.is_dir ? '📁' : '📄'}</span>

          <!-- Name: click dir to expand, dblclick to set as root -->
          <span
            class="entry-name"
            title={node.entry.path}
            onclick={() => node.entry.is_dir && toggleExpand(node.entry)}
            ondblclick={() => node.entry.is_dir && setRoot(node.entry.path)}
          >{node.entry.name}</span>

          {#if !node.entry.is_dir}
            <span class="entry-size">{formatSize(node.entry.size)}</span>
          {/if}

          {#if project}
            <button class="link-btn" title="Link to {project.name}"
              onclick={e => { e.stopPropagation(); ctxLink(node.entry); }}>
              + link
            </button>
          {/if}
        </div>
      {/each}
    {/if}
  </div>

  <!-- Context menu -->
  {#if ctxEntry}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="ctx-menu" style="left:{ctxX}px; top:{ctxY}px" onclick={e => e.stopPropagation()}>
      <button class="ctx-item" onclick={() => ctxReveal(ctxEntry!)}>📂 Show in Explorer</button>
      <button class="ctx-item" onclick={() => ctxOpen(ctxEntry!)}>▶ Open</button>
      {#if ctxEntry.is_dir}
        <button class="ctx-item" onclick={() => { setRoot(ctxEntry!.path); closeCtx(); }}>
          ⤷ Set as root
        </button>
      {/if}
      <div class="ctx-sep"></div>
      <button class="ctx-item" onclick={() => ctxCopyPath(ctxEntry!)}>⎘ Copy path</button>
      {#if ctxEntry.is_dir}
        <button class="ctx-item" onclick={() => ctxBookmark(ctxEntry!)}>
          {isBookmarked(ctxEntry.path) ? '★ Remove bookmark' : '☆ Bookmark folder'}
        </button>
      {/if}
      {#if project}
        <div class="ctx-sep"></div>
        <button class="ctx-item" onclick={() => ctxLink(ctxEntry!)}>+ Link to {project.name}</button>
      {/if}
    </div>
  {/if}
</div>

<style>
.browser {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--sidebar-bg);
  position: relative;
  overflow: hidden;
}

/* Toolbar */
.toolbar {
  display: flex;
  align-items: center;
  gap: 3px;
  padding: 6px 8px;
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}
.nav-btn {
  background: none; border: 1px solid var(--border); border-radius: 5px;
  color: var(--text-muted); cursor: pointer; padding: 3px 7px; font-size: 13px;
  transition: all 0.1s; line-height: 1; flex-shrink: 0;
}
.nav-btn:hover:not(:disabled) { color: var(--text); background: var(--hover-bg); }
.nav-btn:disabled { opacity: 0.3; cursor: default; }
.bookmark-btn { font-size: 13px; }
.bookmark-btn.bookmarked { color: gold; border-color: gold; }
.path-input {
  flex: 1; font-size: 11px; min-width: 0;
  background: var(--input-bg); border: 1px solid var(--border);
  border-radius: 5px; padding: 4px 7px; color: var(--text);
}
.path-input:focus { border-color: var(--accent); outline: none; }

/* Bookmarks */
.bookmarks-panel {
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}
.bookmarks-header {
  display: flex; justify-content: space-between; align-items: center;
  width: 100%; padding: 5px 10px; background: none; border: none;
  color: var(--text-muted); font-size: 11px; font-weight: 600;
  text-transform: uppercase; letter-spacing: 0.06em; cursor: pointer;
}
.bookmarks-header:hover { color: var(--text); background: var(--hover-bg); }
.chevron { font-size: 10px; }
.bookmarks-list { padding: 3px 0; max-height: 140px; overflow-y: auto; }
.bookmark-row {
  display: flex; align-items: center; gap: 2px;
  padding: 1px 6px 1px 10px;
}
.bookmark-item {
  flex: 1; background: none; border: none; color: var(--text-muted); font-size: 12px;
  cursor: pointer; text-align: left; padding: 4px 6px; border-radius: 4px;
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
  min-width: 0;
}
.bookmark-item:hover { color: var(--accent); background: var(--hover-bg); }
.bm-remove {
  background: none; border: none; color: transparent; font-size: 10px; cursor: pointer;
  padding: 2px 4px; border-radius: 3px; flex-shrink: 0;
}
.bookmark-row:hover .bm-remove { color: var(--text-muted); }
.bm-remove:hover { color: #e05c4a !important; }

/* Tree */
.entry-list {
  flex: 1; overflow-y: auto;
  padding: 4px 0;
}
.loading { color: var(--text-muted); font-size: 12px; padding: 12px; }

.entry {
  display: flex; align-items: center; gap: 4px;
  padding-right: 8px;
  cursor: default;
  transition: background 0.1s; font-size: 12px;
  border-radius: 4px; margin: 1px 4px 1px 0;
  min-height: 22px;
}
.entry:hover { background: var(--hover-bg); }

.expand-btn {
  background: none; border: none; color: var(--text-muted); cursor: pointer;
  padding: 0 2px; font-size: 10px; line-height: 1; width: 16px; flex-shrink: 0;
  display: flex; align-items: center; justify-content: center;
}
.expand-btn:hover { color: var(--text); }
.expand-btn.invisible { visibility: hidden; pointer-events: none; }
.spin { display: inline-block; animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }

.entry-icon { font-size: 12px; flex-shrink: 0; }
.entry-name {
  flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
  color: var(--text); cursor: pointer;
}
.entry.is-dir .entry-name { color: var(--text); }
.entry-size { font-size: 10px; color: var(--text-muted); flex-shrink: 0; }

.link-btn {
  display: none; font-size: 10px; padding: 1px 6px; flex-shrink: 0;
  border-radius: 4px; border: 1px solid var(--border);
  background: var(--card-bg); color: var(--text-muted); cursor: pointer;
}
.entry:hover .link-btn { display: block; }
.link-btn:hover { color: var(--accent); border-color: var(--accent); }

/* Context menu */
.ctx-menu {
  position: absolute; z-index: 50;
  background: var(--panel-bg); border: 1px solid var(--border);
  border-radius: 8px; padding: 4px; min-width: 180px;
  box-shadow: 0 6px 24px rgba(0,0,0,0.35);
  display: flex; flex-direction: column; gap: 1px;
}
.ctx-item {
  display: flex; align-items: center; gap: 8px;
  padding: 7px 10px; border: none; background: none;
  color: var(--text); font-size: 12px; font-family: inherit;
  cursor: pointer; border-radius: 5px; text-align: left; width: 100%;
  transition: background 0.1s;
}
.ctx-item:hover { background: var(--hover-bg); }
.ctx-sep { height: 1px; background: var(--border); margin: 3px 6px; }
</style>
