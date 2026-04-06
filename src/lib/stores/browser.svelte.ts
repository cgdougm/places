// Shared state between ItemCard (jump requests) and FileBrowser (navigation + bookmarks).

// ── Jump path ─────────────────────────────────────────────────────────────────
let jumpPath = $state<string | null>(null);

export function getJumpPath() { return jumpPath; }
export function requestJump(path: string) { jumpPath = path; }
export function clearJump() { jumpPath = null; }

// ── Browser bookmarks ─────────────────────────────────────────────────────────
const STORAGE_KEY = 'places_browser_bookmarks';

function load(): string[] {
  try { return JSON.parse(localStorage.getItem(STORAGE_KEY) ?? '[]'); }
  catch { return []; }
}

let bookmarks = $state<string[]>(typeof localStorage !== 'undefined' ? load() : []);

function save() {
  try { localStorage.setItem(STORAGE_KEY, JSON.stringify(bookmarks)); } catch {}
}

export function getBookmarks() { return bookmarks; }

export function addBookmark(path: string) {
  if (!bookmarks.includes(path)) {
    bookmarks = [...bookmarks, path];
    save();
  }
}

export function removeBookmark(path: string) {
  bookmarks = bookmarks.filter(b => b !== path);
  save();
}

export function isBookmarked(path: string) {
  return bookmarks.includes(path);
}
