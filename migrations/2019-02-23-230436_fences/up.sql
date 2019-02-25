CREATE EXTENSION postgis;

CREATE TABLE fences
(
  title text PRIMARY KEY,
  geo_level integer NOT NULL,
  geo geography(multipolygon, 4326) NOT NULL
);

ALTER TABLE votes
ADD COLUMN geo geography(point, 4326) NOT NULL,
ADD COLUMN fence_title text NOT NULL REFERENCES fences(title) ON DELETE cascade,
ADD COLUMN date_voted TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
