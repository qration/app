import { createContext } from 'svelte';
import { MediaQuery } from 'svelte/reactivity';
import type { Feed, Article } from '$lib/util/bindings';

export class FeedStore {
	feeds: Feed[] = $state([]);
	articles: Article[] = $state([]);
}

export const [getFeedStore, setFeedStore] = createContext<FeedStore>();
export const [getMQ, setMQ] = createContext<MediaQuery>();
