CREATE TABLE categories
(
  title text NOT NULL PRIMARY KEY
);

ALTER TABLE surveys
ADD COLUMN category text NOT NULL REFERENCES categories(title) ON DELETE cascade;
