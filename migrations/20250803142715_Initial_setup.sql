-- Consolidated migration script for VoxFila
-- Create sessions table with all fields and defaults
CREATE TABLE IF NOT EXISTS sessions (
    session_id SERIAL PRIMARY KEY,
    state VARCHAR(20) NOT NULL DEFAULT 'new' CHECK (
        state IN ('new', 'current', 'paused', 'finished')
    ),
    songs_per_singer INTEGER NOT NULL DEFAULT 1,
    current_queue_entry_id INTEGER NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- Create singers table with all fields and defaults
CREATE TABLE IF NOT EXISTS singers (
    singer_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    notes TEXT DEFAULT NULL
);
-- Create songs table with all fields and defaults
CREATE TABLE IF NOT EXISTS songs (
    song_id SERIAL PRIMARY KEY,
    artist TEXT NOT NULL,
    title TEXT NOT NULL,
    yturl TEXT,
    isingurl TEXT,
    UNIQUE (artist, title)
);
-- Create queue_entries table with all fields and defaults
CREATE TABLE IF NOT EXISTS queue_entries (
    queue_entry_id SERIAL PRIMARY KEY,
    session_id INTEGER NOT NULL,
    singer_id INTEGER NOT NULL,
    song_id INTEGER NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    queue_position INTEGER,
    original_position INTEGER,
    special_notes TEXT,
    moved_at TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    performed_at TIMESTAMP NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(session_id),
    FOREIGN KEY (singer_id) REFERENCES singers(singer_id),
    FOREIGN KEY (song_id) REFERENCES songs(song_id),
    CONSTRAINT chk_queue_status CHECK (
        status IN (
            'pending',
            'current',
            'completed',
            'skipped',
            'priority'
        )
    )
);
-- Add foreign key constraints
ALTER TABLE sessions
ADD CONSTRAINT fk_sessions_current_queue FOREIGN KEY (current_queue_entry_id) REFERENCES queue_entries(queue_entry_id);
-- Create trigger for automatic timestamp updates
CREATE OR REPLACE FUNCTION update_sessions_timestamp() RETURNS TRIGGER AS $$ BEGIN IF ROW(NEW.*) IS DISTINCT
FROM ROW(OLD.*) THEN NEW.updated_at = CURRENT_TIMESTAMP;
END IF;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trigger_session_updated_at BEFORE
UPDATE ON sessions FOR EACH ROW EXECUTE FUNCTION update_sessions_timestamp();
-- Sample data insertion (optional)
INSERT INTO songs (artist, title, yturl)
VALUES (
        'Queen',
        'Bohemian Rhapsody',
        'https://youtube.com/watch?v=fJ9rUzIMcZQ'
    ),
    (
        'Journey',
        'Don''t Stop Believin''',
        'https://youtube.com/watch?v=1k8craCGpgs'
    ),
    (
        'Adele',
        'Rolling in the Deep',
        'https://youtube.com/watch?v=rYEDA3JcQqw'
    ),
    (
        'Ed Sheeran',
        'Perfect',
        'https://youtube.com/watch?v=2Vv-BfVoq4g'
    ),
    (
        'Whitney Houston',
        'I Will Always Love You',
        'https://youtube.com/watch?v=3JWTaaS7LdU'
    ) ON CONFLICT DO NOTHING;
INSERT INTO singers (name)
VALUES ('Matilda Foster'),
    ('Kyle Dyer'),
    ('Megan Preston'),
    ('Ava Lawson'),
    ('Georgia Cameron') ON CONFLICT DO NOTHING;