CREATE TABLE transcripts (
	id CHAR(21) PRIMARY KEY NOT NULL,
  article_id CHAR(21) NOT NULL REFERENCES articles_light(id) ON DELETE CASCADE,
  video_id TEXT NOT NULL,
	lang TEXT NOT NULL DEFAULT 'en',
  feed_type TEXT NOT NULL CHECK (feed_type IN ('youtube')),
  snippets JSON NOT NULL,

  UNIQUE (video_id, lang)
);

CREATE INDEX idx_transcripts_article_id ON transcripts(article_id);