CREATE TABLE choices
(
  id SERIAL PRIMARY KEY,
  question_id integer NOT NULL REFERENCES questions ON DELETE cascade,
  content text,
  type text NOT NULL,
  title text NOT NULL
)
