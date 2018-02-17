CREATE TABLE locations(
  internal   SERIAL PRIMARY KEY,
  id         BIGINT,
  place      TEXT,
  country    VARCHAR(50),
  city       VARCHAR(50),
  distance   BIGINT
);
