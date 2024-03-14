CREATE TABLE users
(
    id       SERIAL    PRIMARY KEY NOT NULL,
    username TEXT      NOT NULL,
    password TEXT      NOT NULL,
    admin    BOOLEAN   NOT NULL    DEFAULT FALSE,
    created  TIMESTAMP NOT NULL    DEFAULT NOW(),
    logged   TIMESTAMP NOT NULL    DEFAULT NOW()
)