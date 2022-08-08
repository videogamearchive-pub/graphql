CREATE TABLE IF NOT EXISTS games
( id INTEGER NOT NULL
, name VARCHAR NOT NULL
, platform_id INTEGER NOT NULL
, PRIMARY KEY (id)
, FOREIGN KEY(platform_id) REFERENCES platforms(id)
, UNIQUE (name, platform_id)
);
