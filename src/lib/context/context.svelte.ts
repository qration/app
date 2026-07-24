import { createContext } from 'svelte';
import { MediaQuery } from 'svelte/reactivity';
import { type Feed, type ArticleLight, commands } from '$lib/util/bindings';
import type { SyncStatus } from '$lib/util/enums';

export class FeedStore {
	feeds: Feed[] = $state([]);
	articles_light: ArticleLight[] = $state([]);
	status: SyncStatus = $state('idle');
	error: string | null = $state(null);

	async load() {
		if (this.status == 'loading') return;
		this.status = 'loading';
		const feedsRes = await commands.fetchFeeds();
		if (feedsRes.status == 'error') {
			this.error = feedsRes.error;
			this.status = 'error';
			return;
		}

		const alRes = await commands.fetchArticlesLight();
		if (alRes.status == 'error') {
			this.error = alRes.error;
			this.status = 'error';
			return;
		}

		this.feeds = feedsRes.data;
		this.articles_light = alRes.data;
		this.status = 'ready';
	}
}

export const [getFeedStore, setFeedStore] = createContext<FeedStore>();
export const [getMQ, setMQ] = createContext<MediaQuery>();
