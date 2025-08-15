-- returns the next tail position for a given session
CREATE OR REPLACE FUNCTION queue_next_position(p_session_id int) RETURNS bigint LANGUAGE sql STABLE AS $$
SELECT COALESCE(MAX(queue_position), 0) + 100
FROM queue_entries
WHERE session_id = p_session_id;
$$;
-- rebalance: compact and re-space positions for a session
CREATE OR REPLACE FUNCTION queue_rebalance(p_session_id int) RETURNS void LANGUAGE plpgsql AS $$
DECLARE r RECORD;
rn int := 0;
step constant int := 100;
BEGIN FOR r IN
SELECT queue_entry_id
FROM queue_entries
WHERE session_id = p_session_id
ORDER BY queue_position LOOP rn := rn + 1;
UPDATE queue_entries
SET queue_position = rn * step
WHERE queue_entry_id = r.queue_entry_id;
END LOOP;
END;
$$;
-- move entry p_queue_entry_id so it will be located directly AFTER p_after_entry_id
-- both ids must be in the same session; function will rebalance if there's no gap
CREATE OR REPLACE FUNCTION queue_move_after(p_queue_entry_id int, p_after_entry_id int) RETURNS void LANGUAGE plpgsql AS $$
DECLARE v_session_id int;
pos_after int;
pos_next int;
pos_new numeric;
-- temporary numeric to compute midpoint safely
step constant int := 100;
BEGIN IF p_queue_entry_id = p_after_entry_id THEN RAISE EXCEPTION 'Cannot move an entry after itself';
END IF;
-- lock target row and after-row to avoid races and to learn session_id
PERFORM 1
FROM queue_entries
WHERE queue_entry_id = p_queue_entry_id FOR
UPDATE;
SELECT session_id,
    queue_position INTO v_session_id,
    pos_after
FROM queue_entries
WHERE queue_entry_id = p_after_entry_id FOR
UPDATE;
IF NOT FOUND THEN RAISE EXCEPTION 'After-entry id % not found',
p_after_entry_id;
END IF;
-- find next position (first with greater position in same session)
SELECT queue_position INTO pos_next
FROM queue_entries
WHERE session_id = v_session_id
    AND queue_position > pos_after
ORDER BY queue_position
LIMIT 1 FOR
UPDATE;
IF NOT FOUND THEN -- placing at tail
pos_new := pos_after + step;
ELSE -- midpoint (as numeric to preserve fractional if needed)
pos_new := (pos_after::numeric + pos_next::numeric) / 2;
-- if midpoint equals pos_after (no room), rebalance and recompute
IF pos_new = pos_after::numeric THEN PERFORM queue_rebalance(v_session_id);
-- recalc positions under lock
SELECT queue_position INTO pos_after
FROM queue_entries
WHERE queue_entry_id = p_after_entry_id FOR
UPDATE;
SELECT queue_position INTO pos_next
FROM queue_entries
WHERE session_id = v_session_id
    AND queue_position > pos_after
ORDER BY queue_position
LIMIT 1 FOR
UPDATE;
IF NOT FOUND THEN pos_new := pos_after + step;
ELSE pos_new := (pos_after::numeric + pos_next::numeric) / 2;
END IF;
END IF;
END IF;
-- finally update the moved entry to the computed position
UPDATE queue_entries
SET queue_position = floor(pos_new)::bigint
WHERE queue_entry_id = p_queue_entry_id;
END;
$$;
-- trigger: set default queue_position on INSERT when not provided
CREATE OR REPLACE FUNCTION queue_set_default_position_trigger() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.queue_position IS NULL
    OR NEW.queue_position = 0 THEN NEW.queue_position := queue_next_position(NEW.session_id);
END IF;
RETURN NEW;
END;
$$;
-- attach trigger
DROP TRIGGER IF EXISTS trg_queue_set_default_position ON queue_entries;
CREATE TRIGGER trg_queue_set_default_position BEFORE
INSERT ON queue_entries FOR EACH ROW EXECUTE FUNCTION queue_set_default_position_trigger();