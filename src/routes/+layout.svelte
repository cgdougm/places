<script lang="ts">
  import { loadData } from '$lib/stores/projects.svelte';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';

  let { children } = $props();

  let theme = $state<'dark' | 'light'>('dark');

  onMount(async () => {
    const saved = localStorage.getItem('places_theme') as 'dark' | 'light' | null;
    if (saved) theme = saved;
    await loadData();
  });

  function toggleTheme() {
    theme = theme === 'dark' ? 'light' : 'dark';
    localStorage.setItem('places_theme', theme);
  }

  const navItems = [
    { href: '/',         label: 'Projects', icon: '◈' },
    { href: '/settings', label: 'Settings', icon: '⚙' },
    { href: '/migrate',  label: 'Migrate',  icon: '⇄' },
  ];
</script>

<div class="app" data-theme={theme}>
  <header class="app-header">
    <div class="logo-area">
      <span class="logo">places</span>
    </div>

    <nav class="main-nav">
      {#each navItems as item}
        <a
          href={item.href}
          class="nav-link"
          class:active={$page.url.pathname === item.href}
        >
          <span class="nav-icon">{item.icon}</span>
          {item.label}
        </a>
      {/each}
    </nav>

    <div class="header-right">
      <button class="theme-toggle" onclick={toggleTheme} title="Toggle theme">
        {theme === 'dark' ? '☀' : '☾'}
      </button>
    </div>
  </header>

  <main class="app-body">
    {@render children()}
  </main>
</div>

<style>
  /* ── CSS custom properties (theme) ─────────────────────────────────────────── */
  :global([data-theme="dark"]) {
    --bg:           #1a1a1e;
    --panel-bg:     #1f1f23;
    --sidebar-bg:   #17171b;
    --card-bg:      #252529;
    --code-bg:      #141418;
    --hover-bg:     rgba(255,255,255,0.06);
    --active-bg:    rgba(255,255,255,0.1);
    --badge-bg:     rgba(255,255,255,0.08);
    --border:       rgba(255,255,255,0.1);
    --text:         #f2f2f7;
    --text-muted:   #a0a0b8;
    --text-secondary: #c4c4d8;
    --input-bg:     #2a2a2f;
    --accent:       #4f8ef7;
    --header-bg:    #111114;
    --header-border: rgba(255,255,255,0.08);
  }

  :global([data-theme="light"]) {
    --bg:           #f4f4f7;
    --panel-bg:     #ffffff;
    --sidebar-bg:   #ededf2;
    --card-bg:      #ffffff;
    --code-bg:      #f0f0f5;
    --hover-bg:     rgba(0,0,0,0.04);
    --active-bg:    rgba(0,0,0,0.07);
    --badge-bg:     rgba(0,0,0,0.06);
    --border:       rgba(0,0,0,0.1);
    --text:         #1a1a2e;
    --text-muted:   #777788;
    --text-secondary: #55556a;
    --input-bg:     #ffffff;
    --accent:       #3a7de0;
    --header-bg:    #e8e8ee;
    --header-border: rgba(0,0,0,0.1);
  }

  /* ── Global resets ─────────────────────────────────────────────────────────── */
  :global(*, *::before, *::after) { box-sizing: border-box; }
  :global(body) {
    margin: 0; padding: 0;
    font-family: 'Google Sans Flex', 'Google Sans', Inter, system-ui, sans-serif;
    overflow: hidden;
    height: 100vh;
    /* Do NOT set color/background here — CSS vars are defined on .app, not body. */
  }
  :global(a) { color: inherit; text-decoration: none; }
  :global(button) { font-family: inherit; }
  :global(input), :global(textarea) { font-family: inherit; }
  :global(::placeholder) { color: var(--text-muted); opacity: 1; }
  :global(option) { background: var(--input-bg, #2a2a2f); color: var(--text, #f2f2f7); }

  /* Highlight.js dark theme override */
  :global(.hljs) { background: transparent; color: var(--text); }

  /* ── App shell ──────────────────────────────────────────────────────────────── */
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    /* CSS vars are defined on .app, not on body, so set color here
       so all descendants inherit the correct themed value. */
    color: var(--text);
    background: var(--bg);
  }

  .app-header {
    display: flex;
    align-items: center;
    padding: 0 16px;
    height: 44px;
    background: var(--header-bg);
    border-bottom: 1px solid var(--header-border);
    gap: 20px;
    flex-shrink: 0;
  }

  .logo-area {
    flex-shrink: 0;
  }

  .logo {
    font-family: 'Atma', serif;
    font-weight: 500;
    font-size: 22px;
    letter-spacing: -0.04em;
    color: var(--accent);
    line-height: 1;
  }

  .main-nav {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .nav-link {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 12px;
    border-radius: 7px;
    font-size: 13px;
    color: var(--text-muted);
    transition: all 0.12s;
  }
  .nav-link:hover { color: var(--text); background: var(--hover-bg); }
  .nav-link.active { color: var(--text); background: var(--active-bg); }

  .nav-icon { font-size: 14px; }

  .header-right {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .theme-toggle {
    background: none;
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px 8px;
    font-size: 14px;
    transition: all 0.12s;
  }
  .theme-toggle:hover { color: var(--text); background: var(--hover-bg); }

  .app-body {
    flex: 1;
    overflow: hidden;
    display: flex;
  }
</style>
