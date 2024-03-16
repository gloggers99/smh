CREATE TABLE posts
(
    id          SERIAL    PRIMARY KEY NOT NULL,
    created     TIMESTAMP NOT NULL    DEFAULT  CURRENT_TIMESTAMP,
    title       TEXT      NOT NULL,
    author      TEXT      NOT NULL,
    description TEXT      NOT NULL,
    content     TEXT      NOT NULL
)