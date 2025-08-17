CREATE EXTENSION pg_trgm;
CREATE INDEX ON songs USING gin ((artist || ' ' || title) gin_trgm_ops);