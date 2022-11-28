-- a single table is used for all events in the cqrs system
CREATE TABLE events
(
  aggregate_type text    NOT NULL,
  aggregate_id   text    NOT NULL,
  sequence       integer NOT NULL CHECK (sequence >= 0),
  event_type     text    NOT NULL,
  event_version  text    NOT NULL,
  payload        text    NOT NULL,
  metadata       text    NOT NULL,
  PRIMARY KEY (aggregate_type, aggregate_id, sequence)
) STRICT;

-- this table is only needed if snapshotting is employed
CREATE TABLE snapshots
(
  aggregate_type   text    NOT NULL,
  aggregate_id     text    NOT NULL,
  last_sequence    integer NOT NULL CHECK (last_sequence >= 0),
  current_snapshot integer NOT NULL CHECK (current_snapshot >= 0),
  payload          text    NOT NULL,
  PRIMARY KEY (aggregate_type, aggregate_id, last_sequence)
) STRICT;
