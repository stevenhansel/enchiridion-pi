-- Add migration script here
ALTER TABLE announcement 
ADD COLUMN media_type TEXT;

ALTER TABLE announcement
ADD COLUMN media_duration FLOAT;