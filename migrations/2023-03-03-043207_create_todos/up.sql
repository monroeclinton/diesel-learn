CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    finish_timestamp TIMESTAMP(6) WITH TIME ZONE,
    timestamp TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT (now() at time zone 'utc')
)
