CREATE TABLE feeds (
  id CHAR(21) PRIMARY KEY NOT NULL,
  feed_name TEXT NOT NULL,
  feed_type TEXT NOT NULL CHECK (feed_type IN ('rss', 'atom', 'youtube')),
  favourited BOOLEAN NOT NULL,
  feed_url TEXT NOT NULL UNIQUE,
	site_url TEXT,
  last_fetched INTEGER,

	UNIQUE (id)
);

CREATE TABLE articles_light (
  id CHAR(21) PRIMARY KEY NOT NULL,
  feed_id CHAR(21) NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
	article_guid TEXT NOT NULL,
	article_name TEXT NOT NULL,
	article_description TEXT,
	article_url TEXT,
	article_date BIGINT,
	media_type TEXT NOT NULL CHECK (media_type IN ('text', 'video')),
	article_read BOOLEAN NOT NULL,
	article_saved BOOLEAN NOT NULL,

	UNIQUE (feed_id, article_guid)
);

CREATE TABLE articles_content (
	id CHAR(21) PRIMARY KEY NOT NULL REFERENCES articles_light(id) ON DELETE CASCADE,
	content TEXT,
	enclosure_url TEXT,
	enclosure_mime_type TEXT,
	enclosure_length BIGINT
);