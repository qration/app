import dataJson from '$lib/assets/test-data.json';
import type { Article, Feed } from '$lib/util/interfaces';

class FeedStore {
	data = $state<{ feeds: Feed[]; articles: Article[] }>(dataJson);

	addFeed(f: Feed) {
		this.data.feeds.push(f);
	}

	addArticle(a: Article) {
		this.data.articles.push(a);
	}
}

export const feedStore = new FeedStore();
