CREATE TABLE feeds (
  id CHAR(21) PRIMARY KEY NOT NULL,
  feed_name TEXT NOT NULL,
  feed_type TEXT NOT NULL CHECK (feed_type IN ('rss', 'atom')),
  favourited BOOLEAN NOT NULL,
  feed_url TEXT NOT NULL,
  last_fetched INTEGER
);