import { createContext } from 'svelte';
import { MediaQuery } from 'svelte/reactivity';
import { type Feed, type ArticleLight, commands } from '$lib/util/bindings';
import type { SyncStatus } from '$lib/util/enums';
import {
	sendNotification,
	isPermissionGranted,
} from '@choochmeque/tauri-plugin-notifications-api';
import { mode } from 'mode-watcher';

export class FeedStore {
	feeds: Feed[] = $state([]);
	articles_light: ArticleLight[] = $state([]);
	status: SyncStatus = $state('idle');
	error: string | null = $state(null);
	lastLoaded: number | null = $state(null);
	lastRefreshed: number | null = $state(null);
	notificationPermsGranted = $state(false);

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
		this.lastLoaded = Date.now();
	}

	async refreshAll(force = false) {
		this.status = 'loading';
		if (
			!force &&
			this.lastRefreshed &&
			Date.now() - this.lastRefreshed < 5 * 60000
		)
			return;

		const res = await commands.refreshFeeds();
		if (res.status == 'error') {
			this.error = res.error;
			this.status = 'error';
			return;
		}

		this.articles_light = res.data.articles_light.concat(this.articles_light);
		this.status = 'ready';
		this.lastRefreshed = Date.now();
		this.notificationPermsGranted = await isPermissionGranted();

		if (res.data.new_count > 0 && this.notificationPermsGranted) {
			sendNotification({
				id: 1,
				title: `Recieved ${res.data.new_count} new items`,
				body: this.articles_light[0].article_name || '[]',
			});
		}
	}
}

let now = $state(Date.now());

setInterval(() => {
	now = Date.now();
}, 60000);

export function getNow() {
	return now;
}

export function getOsbOptions() {
	return {
		scrollbars: {
			theme: mode.current == 'dark' ? 'os-theme-light' : 'os-theme-dark',
		},
	};
}

export const [getFeedStore, setFeedStore] = createContext<FeedStore>();
export const [getMQ, setMQ] = createContext<MediaQuery>();
