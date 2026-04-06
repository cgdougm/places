/** Read a number from localStorage, returning `fallback` if missing/invalid. */
export function getNum(key: string, fallback: number): number {
  try {
    const v = localStorage.getItem(key);
    if (v === null) return fallback;
    const n = Number(v);
    return isFinite(n) ? n : fallback;
  } catch { return fallback; }
}

/** Read a boolean from localStorage. */
export function getBool(key: string, fallback: boolean): boolean {
  try {
    const v = localStorage.getItem(key);
    if (v === null) return fallback;
    return v === 'true';
  } catch { return fallback; }
}

/** Write any value to localStorage (converts to string). */
export function set(key: string, value: unknown): void {
  try { localStorage.setItem(key, String(value)); } catch {}
}
