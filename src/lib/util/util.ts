import type { Feed, Article } from '$lib/util/bindings';

export function getFeedIcon(type: string): string {
	if (type == 'rss') {
		return 'tabler:rss';
	} else if (type == 'youtube') {
		return 'tabler:brand-youtube';
	}
	return '';
}

// this is only a temporary function, i'll move this to the backend when we implement it
export function filterArticles(
	feeds: Feed[],
	articles: Article[],
	filter: string,
): Article[] {
	if (filter == 'feed-all') {
		return articles;
	} else if (filter == 'article-unread') {
		return articles.filter((a) => !a.read);
	} else if (filter == 'article-today') {
		return articles.filter((a) =>
			a.date
				? Date.now() - dateStrParse(a.date) * 1000 <= 24 * 60 * 60 * 1000
				: false,
		);
	} else if (filter == 'feed-favourites') {
		return articles.filter(
			(a) => feeds.find((f) => f.id == a.feed_id)!.favourited == true,
		);
	} else if (filter == 'article-saved') {
		return articles.filter((a) => a.saved == true);
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

export function dateStrParse(dateStr: string | null): number {
	return dateStr ? Math.floor(Date.parse(dateStr) / 1000) : 0;
}
