CREATE TABLE users
(
  username text PRIMARY KEY,
  password text NOT NULL,
  email text NOT NULL UNIQUE,
  first_name text NOT NULL,
  last_name text NOT NULL,
  photo_url text
    DEFAULT 'https://moonvillageassociation.org/wp-content/uploads/2018/06/default-profile-picture1.jpg',
  is_admin boolean NOT NULL default false
);
