CREATE TABLE IF NOT EXISTS todos
(
    id          INTEGER PRIMARY KEY NOT NULL,
    chat_id     INTEGER             NOT NULL,
    task_id     INTEGER             NOT NULL,
    description TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);
