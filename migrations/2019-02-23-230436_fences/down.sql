ALTER TABLE votes
DROP COLUMN geo,
DROP COLUMN fence_title,
DROP COLUMN date_voted;

DROP TABLE fences cascade;

DROP EXTENSION postgis;