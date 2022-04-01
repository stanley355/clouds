-- Your SQL goes here
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    hosts_id integer NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    free BOOLEAN NOT NULL,
    pricing VARCHAR NOT NULL,
    FOREIGN KEY (hosts_id) REFERENCES hosts (id)
)
