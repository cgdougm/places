import { invoke } from '@tauri-apps/api/core';
import type { AppData, Project, PlaceItem } from '$lib/types';
import { makeId, DEFAULT_COLORS } from '$lib/types';

// ── state ─────────────────────────────────────────────────────────────────────
let projects = $state<Project[]>([]);
let activeProjectId = $state<string | null>(null);

// ── derived ───────────────────────────────────────────────────────────────────
export const activeProject = $derived(
  projects.find(p => p.id === activeProjectId) ?? null
);

// ── persistence ───────────────────────────────────────────────────────────────
async function persist() {
  const data: AppData = { projects, version: 1 };
  await invoke<void>('save_app_data', { json: JSON.stringify(data) });
}

export async function loadData() {
  try {
    const raw = await invoke<string>('load_app_data');
    const data: AppData = JSON.parse(raw);
    projects = data.projects ?? [];
    activeProjectId = projects[0]?.id ?? null;
  } catch {
    projects = [];
    activeProjectId = null;
  }
}

// ── project actions ───────────────────────────────────────────────────────────
export function getProjects() { return projects; }
export function getActiveId() { return activeProjectId; }

export function selectProject(id: string) {
  activeProjectId = id;
}

export async function addProject(name: string) {
  const project: Project = {
    id: makeId(),
    name,
    color: DEFAULT_COLORS[projects.length % DEFAULT_COLORS.length],
    items: [],
    created: Date.now(),
  };
  projects = [...projects, project];
  activeProjectId = project.id;
  await persist();
  return project;
}

export async function renameProject(id: string, name: string) {
  projects = projects.map(p => p.id === id ? { ...p, name } : p);
  await persist();
}

export async function deleteProject(id: string) {
  projects = projects.filter(p => p.id !== id);
  if (activeProjectId === id) {
    activeProjectId = projects[0]?.id ?? null;
  }
  await persist();
}

export async function setProjectColor(id: string, color: string) {
  projects = projects.map(p => p.id === id ? { ...p, color } : p);
  await persist();
}

// ── item actions ──────────────────────────────────────────────────────────────
export async function addItem(projectId: string, item: Omit<PlaceItem, 'id' | 'created'>) {
  const newItem: PlaceItem = { ...item, id: makeId(), created: Date.now() };
  projects = projects.map(p =>
    p.id === projectId ? { ...p, items: [...p.items, newItem] } : p
  );
  await persist();
  return newItem;
}

export async function updateItem(projectId: string, itemId: string, patch: Partial<PlaceItem>) {
  projects = projects.map(p =>
    p.id !== projectId ? p : {
      ...p,
      items: p.items.map(i => i.id === itemId ? { ...i, ...patch } : i),
    }
  );
  await persist();
}

export async function deleteItem(projectId: string, itemId: string) {
  projects = projects.map(p =>
    p.id !== projectId ? p : { ...p, items: p.items.filter(i => i.id !== itemId) }
  );
  await persist();
}
