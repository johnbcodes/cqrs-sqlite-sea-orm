CREATE TABLE checks (
  id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  account_id TEXT NOT NULL,
  "check"    TEXT NOT NULL
) STRICT;
