CREATE TABLE surveys
(
  id SERIAL PRIMARY KEY,
  author text NOT NULL REFERENCES users ON DELETE cascade,
  title text NOT NULL UNIQUE,
  description text,
  anonymous boolean NOT NULL default true,
  published boolean NOT NULL default false,
  date_posted TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);
