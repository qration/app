import { createContext } from 'svelte';
import { MediaQuery } from 'svelte/reactivity';
import type { Feed, Article } from '$lib/util/bindings';
import type { SyncStatus } from '$lib/util/enums';
import { invoke } from '@tauri-apps/api/core';

export class FeedStore {
	feeds: Feed[] = $state([]);
	articles: Article[] = $state([]);
	status: SyncStatus = $state('idle');
	error: string | null = $state(null);

	async load() {
		if (this.status == 'loading') return;
		this.status = 'loading';
		try {
			this.feeds = await invoke('fetch_feeds');
			this.status = 'ready';
		} catch (e) {
			this.error = String(e);
			this.status = 'error';
		}
	}
}

export const [getFeedStore, setFeedStore] = createContext<FeedStore>();
export const [getMQ, setMQ] = createContext<MediaQuery>();
