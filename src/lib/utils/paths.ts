/** Normalize any path style to the current OS style returned by Rust. */
export function normalizePath(p: string): string {
  // Convert forward-slash unix-style /c/Users/... → C:\Users\...
  if (/^\/[a-zA-Z]\//.test(p)) {
    return p[1].toUpperCase() + ':' + p.slice(2).replace(/\//g, '\\');
  }
  return p;
}

export function basename(p: string): string {
  return p.replace(/[/\\]+$/, '').split(/[/\\]/).pop() ?? p;
}

export function dirname(p: string): string {
  const normalized = p.replace(/[/\\]+$/, '');
  const sep = normalized.includes('\\') ? '\\' : '/';
  const parts = normalized.split(sep);
  parts.pop();
  return parts.join(sep) || sep;
}

export function fileExtension(p: string): string {
  const base = basename(p);
  const dot = base.lastIndexOf('.');
  return dot > 0 ? base.slice(dot + 1).toLowerCase() : '';
}
