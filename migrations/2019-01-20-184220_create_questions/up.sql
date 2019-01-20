CREATE TABLE questions
(
  id SERIAL PRIMARY KEY,
  survey_id integer NOT NULL REFERENCES surveys ON DELETE cascade,
  type text NOT NULL,
  title text NOT NULL
)
