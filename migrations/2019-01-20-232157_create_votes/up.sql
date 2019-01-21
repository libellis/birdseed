CREATE TABLE votes
(
  choice_id integer NOT NULL REFERENCES choices ON DELETE cascade,
  username text NOT NULL REFERENCES users ON DELETE cascade,
  PRIMARY KEY (choice_id, username),
  score integer NOT NULL
)
