-- Your SQL goes here
ALTER TABLE hosts
  ALTER COLUMN description DROP NOT NULL,
  ALTER COLUMN url DROP NOT NULL