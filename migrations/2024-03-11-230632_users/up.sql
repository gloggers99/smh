CREATE TABLE users
(
    id        SERIAL    PRIMARY KEY NOT NULL,
    email     TEXT      NOT NULL,
    username  TEXT      NOT NULL,
    password  TEXT      NOT NULL,
    admin     BOOLEAN   NOT NULL    DEFAULT FALSE,
    /*activated BOOLEAN   NOT NULL    DEFAULT FALSE,*/
    created   TIMESTAMP NOT NULL    DEFAULT CURRENT_TIMESTAMP,
    logged    TIMESTAMP NOT NULL    DEFAULT CURRENT_TIMESTAMP
)