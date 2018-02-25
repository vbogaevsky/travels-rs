CREATE TABLE locations(
  id         BIGINT PRIMARY KEY,
  place      TEXT,
  country    VARCHAR(50),
  city       VARCHAR(50),
  distance   BIGINT
);
