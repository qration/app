export interface Feed {
	id: string;
	name: string;
	type: string;
	favourited: boolean;
	url: string;
	last_fetched: number;
}

export interface Article {
	id: string;
	name: string;
	feed_id: string;
	url: string;
	saved: boolean;
	read: boolean;
	timestamp: number;
	media_type: string;
	content: string;
	media_url: string | null;
}
