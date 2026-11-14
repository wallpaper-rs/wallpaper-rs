CREATE TABLE mail_queue (
  id INTEGER PRIMARY KEY,
  topic TEXT NOT NULL CHECK (topic IN ('case', 'health', 'test')),
  case_id INTEGER REFERENCES cases(id) ON DELETE SET NULL,
  subject TEXT NOT NULL,
  body TEXT NOT NULL,
  state TEXT NOT NULL DEFAULT 'queued' CHECK (state IN ('queued', 'sent', 'failed')),
  tries INTEGER NOT NULL DEFAULT 0,
  next_at INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  sent_at INTEGER
) STRICT;
CREATE INDEX mail_queue_due ON mail_queue (state, next_at);
CREATE INDEX mail_queue_case ON mail_queue (case_id);
CREATE TABLE health_seen (row TEXT PRIMARY KEY, tone TEXT NOT NULL) STRICT, WITHOUT ROWID;
ALTER TABLE repo_state ADD COLUMN failed_at INTEGER;
ALTER TABLE repo_state ADD COLUMN failed_raw TEXT;
