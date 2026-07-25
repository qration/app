import type { Feed, ArticleLight, FeedType } from '$lib/util/bindings';

export function getFeedIcon(feedType: FeedType): string {
	if (feedType == 'rss') {
		return 'tabler:rss';
	} else if (feedType == 'youtube') {
		return 'tabler:brand-youtube';
	} else if (feedType == 'atom') {
		return 'tabler:atom-2';
	}
	return 'tabler:question-mark';
}

// this is only a temporary function, i'll move this to the backend when we implement it
export function filterArticles(
	feeds: Feed[],
	articles: ArticleLight[],
	filter: string,
): ArticleLight[] {
	if (filter == 'feed-all') {
		return articles;
	} else if (filter == 'article-unread') {
		return articles.filter((a) => !a.article_read);
	} else if (filter == 'article-today') {
		return articles.filter((a) =>
			a.article_date
				? Date.now() - a.article_date * 1000 <= 24 * 60 * 60 * 1000
				: false,
		);
	} else if (filter == 'feed-favourites') {
		return articles.filter(
			(a) => feeds.find((f) => f.id == a.feed_id)!.favourited == true,
		);
	} else if (filter == 'article-saved') {
		return articles.filter((a) => a.article_saved == true);
	}
	return articles.filter((a) => filter == a.feed_id);
}

export function isValidURL(url: string): boolean {
	try {
		new URL(url);
		return true;
	} catch {
		return false;
	}
}
