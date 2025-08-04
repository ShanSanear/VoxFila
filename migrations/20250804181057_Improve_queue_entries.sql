ALTER TABLE queue_entries
    RENAME COLUMN special_notes TO notes;
ALTER TABLE queue_entries
ADD COLUMN second_singer_id INT REFERENCES singers(singer_id);