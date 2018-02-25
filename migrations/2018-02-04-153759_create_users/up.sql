CREATE TABLE users (
  id         BIGINT PRIMARY KEY,
  email      VARCHAR(100) NOT NULL,
  first_name VARCHAR(50) NOT NULL,
  last_name  VARCHAR(50) NOT NULL,
  gender     VARCHAR(1) NOT NULL,
  birth_date BIGINT NOT NULL
);
