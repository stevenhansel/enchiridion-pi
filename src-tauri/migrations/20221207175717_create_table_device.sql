-- Add migration script here
CREATE TABLE IF NOT EXISTS device (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  location TEXT NOT NULL,
  floor_id INTEGER NOT NULL,
  floor_name TEXT NOT NULL,
  building_id INTEGER NOT NULL,
  building_name TEXT NOT NULL,
  building_color TEXT NOT NULL,
  access_key_id TEXT NOT NULL,
  secret_access_key TEXT NOT NULL,
  camera_enabled INTEGER NOT NULL,
  created_at DATETIME NOT NULL,
  updated_at DATETIME
);
