CREATE TABLE users (
  internal   SERIAL PRIMARY KEY,
  id         BIGINT,
  email      VARCHAR(100),
  first_name VARCHAR(50),
  last_name  VARCHAR(50),
  gender     CHAR,
  birth_date BIGINT
);
