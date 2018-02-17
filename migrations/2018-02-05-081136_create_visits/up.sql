CREATE TABLE visits (
  internal   SERIAL PRIMARY KEY,
  id         BIGINT,
  location   BIGINT,
  "user"     BIGINT,
  visited_at BIGINT,
  mark       SMALLINT
);
