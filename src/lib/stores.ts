import { writable } from 'svelte/store';
// import { invoke } from '@tauri-apps/api/tauri';

export const count = writable(1);
export const uuids = writable([]);
export const uuid = writable("");

// TODO: store with all the UUIDs of connected Solo 2 devices
