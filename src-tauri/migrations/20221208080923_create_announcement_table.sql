-- Add migration script here
CREATE TABLE IF NOT EXISTS "announcement" (
  id INTEGER PRIMARY KEY,
  announcement_id INTEGER NOT NULL,
  local_path TEXT NOT NULL
);
