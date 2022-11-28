CREATE TABLE ledger_entries (
  id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  account_id  TEXT NOT NULL,
  description TEXT NOT NULL,
  amount      REAL NOT NULL
) STRICT;