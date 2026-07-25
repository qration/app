PRAGMA foreign_keys=OFF;

CREATE TABLE feeds_new (
  id CHAR(21) PRIMARY KEY NOT NULL,
  feed_name TEXT NOT NULL,
  feed_type TEXT NOT NULL CHECK (feed_type IN ('rss', 'atom', 'youtube')),
  favourited BOOLEAN NOT NULL,
  feed_url TEXT NOT NULL,
  last_fetched INTEGER
);

INSERT INTO feeds_new SELECT * FROM feeds;

DROP TABLE feeds;

ALTER TABLE feeds_new RENAME TO feeds;

PRAGMA foreign_keys=ON;
