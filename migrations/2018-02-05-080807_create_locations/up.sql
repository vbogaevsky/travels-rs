CREATE TABLE locations(
  id         BIGINT PRIMARY KEY,
  place      TEXT NOT NULL,
  country    VARCHAR(50) NOT NULL,
  city       VARCHAR(50) NOT NULL,
  distance   BIGINT NOT NULL
);
