export type ItemType = 'file' | 'url' | 'text';

export interface PlaceItem {
  id: string;
  type: ItemType;
  label: string;
  value: string;   // file path, URL, or text content
  notes: string;
  created: number; // unix ms
}

export interface Project {
  id: string;
  name: string;
  color: string;   // css color for accent
  items: PlaceItem[];
  created: number;
}

export interface AppData {
  projects: Project[];
  version: number;
}

export interface FileEntry {
  name: string;
  path: string;
  is_dir: boolean;
  size: number;
  modified: string; // ISO 8601
}

export const DEFAULT_COLORS = [
  '#4f8ef7', '#e05c4a', '#50b86c', '#e0a030',
  '#9b59b6', '#1abc9c', '#e67e22', '#e91e63',
];

export function makeId(): string {
  return Math.random().toString(36).slice(2) + Date.now().toString(36);
}
