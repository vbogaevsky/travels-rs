CREATE TABLE visits (
  id         BIGINT PRIMARY KEY,
  location   BIGINT,
  "user"     BIGINT,
  visited_at BIGINT,
  mark       SMALLINT
);
