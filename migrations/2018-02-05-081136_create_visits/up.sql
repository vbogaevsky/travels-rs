CREATE TABLE visits (
  id         BIGINT PRIMARY KEY,
  location   BIGINT NOT NULL,
  "user"     BIGINT NOT NULL,
  visited_at BIGINT NOT NULL,
  mark       SMALLINT NOT NULL
);
