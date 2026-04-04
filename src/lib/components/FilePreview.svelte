<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { fileExtension, basename } from '$lib/utils/paths';
  import { onMount } from 'svelte';
  import { marked } from 'marked';
  import hljs from 'highlight.js';

  // Configure marked to use highlight.js
  marked.setOptions({
    // @ts-ignore — marked v9+ gfm option
    gfm: true,
    breaks: true,
    // @ts-ignore
    highlight(code: string, lang: string) {
      if (lang && hljs.getLanguage(lang)) {
        return hljs.highlight(code, { language: lang }).value;
      }
      return hljs.highlightAuto(code).value;
    },
  });

  let { path }: { path: string } = $props();

  type Mode = 'loading' | 'text' | 'markdown' | 'image' | 'video' | 'unsupported';

  let mode = $state<Mode>('loading');
  let textContent = $state('');
  let htmlContent = $state('');
  let assetUrl = $state('');
  let showRaw = $state(false);
  let error = $state('');
  let lang = $state('');

  const TEXT_EXTS = new Set([
    'txt','md','markdown','rs','ts','tsx','js','jsx','json','toml','yaml','yml',
    'html','htm','css','scss','py','rb','go','java','c','cpp','h','hpp','sh',
    'bash','zsh','fish','lua','sql','xml','csv','ini','cfg','conf','env',
    'svelte','vue','astro','php','kt','swift','r','dart','tf','hcl',
  ]);
  const IMAGE_EXTS = new Set(['png','jpg','jpeg','gif','webp','svg','bmp','ico','avif']);
  const VIDEO_EXTS = new Set(['mp4','webm','ogg','mov']);
  const MD_EXTS = new Set(['md','markdown']);

  const EXT_LANG: Record<string, string> = {
    rs: 'rust', ts: 'typescript', tsx: 'typescript', js: 'javascript', jsx: 'javascript',
    py: 'python', rb: 'ruby', go: 'go', java: 'java', c: 'c', cpp: 'cpp',
    h: 'c', hpp: 'cpp', sh: 'bash', bash: 'bash', zsh: 'bash', lua: 'lua',
    sql: 'sql', html: 'html', htm: 'html', css: 'css', scss: 'scss',
    json: 'json', toml: 'toml', yaml: 'yaml', yml: 'yaml', xml: 'xml',
    svelte: 'html', kt: 'kotlin', swift: 'swift', php: 'php', tf: 'hcl',
  };

  async function load() {
    mode = 'loading';
    error = '';
    const ext = fileExtension(path);

    if (VIDEO_EXTS.has(ext)) {
      assetUrl = convertFileSrc(path);
      mode = 'video';
      return;
    }

    if (IMAGE_EXTS.has(ext)) {
      assetUrl = convertFileSrc(path);
      mode = 'image';
      return;
    }

    if (TEXT_EXTS.has(ext)) {
      try {
        textContent = await invoke<string>('read_text_file', { path });
        lang = EXT_LANG[ext] ?? '';

        if (MD_EXTS.has(ext)) {
          htmlContent = await marked(textContent);
          mode = 'markdown';
        } else {
          mode = 'text';
        }
      } catch (e) {
        error = String(e);
        mode = 'unsupported';
      }
      return;
    }

    mode = 'unsupported';
  }

  // Re-load whenever path changes
  $effect(() => { if (path) load(); });

  // Syntax highlight text content after render
  $effect(() => {
    if (mode === 'text') {
      // highlight.js applied via the hljs class on the code block in template
    }
  });
</script>

<div class="preview">
  {#if mode === 'loading'}
    <div class="center muted">Loading…</div>

  {:else if mode === 'image'}
    <div class="image-wrap">
      <img src={assetUrl} alt={basename(path)} />
    </div>

  {:else if mode === 'video'}
    <div class="video-wrap">
      <!-- svelte-ignore a11y_media_has_caption -->
      <video src={assetUrl} controls preload="metadata">
        Your browser does not support video playback.
      </video>
    </div>

  {:else if mode === 'markdown'}
    <div class="md-toolbar">
      <button class="tab-btn" class:active={!showRaw} onclick={() => showRaw = false}>Rendered</button>
      <button class="tab-btn" class:active={showRaw} onclick={() => showRaw = true}>Raw</button>
    </div>
    {#if showRaw}
      <pre class="code-block"><code class="language-markdown">{@html hljs.highlight(textContent, { language: 'markdown' }).value}</code></pre>
    {:else}
      <div class="markdown-body">{@html htmlContent}</div>
    {/if}

  {:else if mode === 'text'}
    <pre class="code-block"><code>{#if lang && hljs.getLanguage(lang)}{@html hljs.highlight(textContent, { language: lang }).value}{:else}{@html hljs.highlightAuto(textContent).value}{/if}</code></pre>

  {:else if mode === 'unsupported'}
    <div class="center muted">
      {#if error}
        <span style="color:#e05c4a">{error}</span>
      {:else}
        No preview available for <strong>{basename(path)}</strong>
      {/if}
    </div>
  {/if}
</div>

<style>
.preview {
  flex: 1;
  overflow: auto;
  display: flex;
  flex-direction: column;
  font-size: 12px;
}

.center {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: 16px;
}
.muted { color: var(--text-muted); }

.image-wrap {
  flex: 1;
  overflow: auto;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding: 8px;
}
.image-wrap img {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.video-wrap {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px;
  background: #000;
}
.video-wrap video {
  max-width: 100%;
  max-height: 100%;
}

.md-toolbar {
  display: flex;
  gap: 4px;
  padding: 6px 10px;
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}
.tab-btn {
  padding: 3px 10px;
  font-size: 11px;
  border-radius: 5px;
  border: 1px solid var(--border);
  background: none;
  color: var(--text-muted);
  cursor: pointer;
}
.tab-btn.active {
  background: var(--active-bg);
  color: var(--text);
}

.code-block {
  margin: 0;
  padding: 10px 14px;
  overflow: auto;
  flex: 1;
  font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', monospace;
  font-size: 11.5px;
  line-height: 1.6;
  tab-size: 2;
  white-space: pre;
  background: var(--code-bg);
  color: var(--text);
}
.code-block code { background: none; padding: 0; font-size: inherit; }

/* GitHub-like markdown styles */
.markdown-body {
  padding: 14px 18px;
  overflow-y: auto;
  flex: 1;
  line-height: 1.6;
  color: var(--text);
}
:global(.markdown-body h1) { font-size: 1.5em; border-bottom: 1px solid var(--border); padding-bottom: 6px; }
:global(.markdown-body h2) { font-size: 1.25em; border-bottom: 1px solid var(--border); padding-bottom: 4px; }
:global(.markdown-body h3) { font-size: 1.1em; }
:global(.markdown-body code) {
  font-family: 'Cascadia Code', 'Fira Code', monospace;
  font-size: 0.88em;
  background: var(--code-bg);
  padding: 2px 5px;
  border-radius: 4px;
}
:global(.markdown-body pre) {
  background: var(--code-bg);
  border-radius: 6px;
  padding: 12px 14px;
  overflow-x: auto;
  margin: 10px 0;
}
:global(.markdown-body pre code) { background: none; padding: 0; font-size: 11.5px; }
:global(.markdown-body blockquote) {
  border-left: 3px solid var(--accent);
  margin: 0;
  padding: 4px 14px;
  color: var(--text-secondary);
}
:global(.markdown-body table) { border-collapse: collapse; width: 100%; }
:global(.markdown-body th, .markdown-body td) {
  border: 1px solid var(--border);
  padding: 6px 12px;
  text-align: left;
}
:global(.markdown-body th) { background: var(--sidebar-bg); }
:global(.markdown-body a) { color: var(--accent); }
:global(.markdown-body img) { max-width: 100%; }
:global(.markdown-body ul, .markdown-body ol) { padding-left: 1.5em; }
:global(.markdown-body hr) { border: none; border-top: 1px solid var(--border); }
:global(.markdown-body input[type="checkbox"]) { margin-right: 4px; }
</style>
