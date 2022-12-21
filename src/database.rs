use std::fmt::Display;

use anyhow::Result;
use chrono::NaiveDate;
use sqlx::sqlite::SqlitePool;

pub struct Task {
    pub date: NaiveDate,
    pub description: String,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.date, self.description)
    }
}

pub async fn add_todo(
    pool: &SqlitePool,
    chat_id: i64,
    date: NaiveDate,
    description: &str,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO todos ( chat_id, date, description )
VALUES ( ?1, ?2, ?3 )
        "#,
        chat_id,
        date,
        description
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_todo(pool: &SqlitePool, chat_id: i64, task_id: u32) -> Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
DELETE FROM todos
WHERE rowid IN (
    SELECT rowid FROM (
        SELECT ROW_NUMBER() OVER (ORDER BY date) row, rowid FROM (
            SELECT rowid, date
            FROM todos
            WHERE chat_id = ?1
        )
    ) WHERE row = ?2
)
        "#,
        chat_id,
        task_id,
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn list_todos(pool: &SqlitePool, chat_id: i64) -> Result<Vec<Task>> {
    Ok(sqlx::query_as!(
        Task,
        r#"
SELECT
    date as "date: _",
    description
FROM
    todos
WHERE
    chat_id = ?1
ORDER BY date
    "#,
        chat_id
    )
    .fetch_all(pool)
    .await?)
}
