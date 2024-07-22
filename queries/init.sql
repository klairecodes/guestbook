CREATE TABLE IF NOT EXISTS guestbook_app (
    id              UUID PRIMARY KEY, -- app instance id
    creation_time   TIMESTAMP WITH TIME ZONE, -- time app was created
    guestbooks REFERENCES guestbook
);

CREATE TABLE IF NOT EXISTS users (
    id              UUID PRIMARY KEY,
    creation_time   TIMESTAMP WITH TIME ZONE, -- time entry was created
    csh_id          VARCHAR(256), -- 256 is unix username limit
    first_name      VARCHAR(64), -- real first name
    last_name       VARCHAR(64), -- real last name
);

CREATE TABLE IF NOT EXISTS guestbook (
    id              UUID PRIMARY KEY,
    start_date      DATE, -- the date of the first entry in the guestbook
    end_date        DATE, -- the date of the last entry in the guestbook
    host            VARCHAR (128), -- who is the host of this book
    assets          TEXT[], -- array of s3 bucket urls for images
    entries         REFERENCES entry -- all the entries in the guestbook
);

CREATE TABLE IF NOT EXISTS entry (
    id              UUID PRIMARY KEY,
    last_edited     TIMESTAMP WITH TIME ZONE, -- time entry was last modified
    written         DATE, -- date the entry was written in the book
    content         TEXT -- transcribed content of the entry

);