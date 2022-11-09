use sqlx::sqlite::SqlitePool;

pub async fn add_todo(pool: &SqlitePool, chat_id: i64, description: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO todos ( chat_id, task_id, description )
VALUES ( ?1, ?2, ?3 )
        "#,
        chat_id,
        1,
        description
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = ?1
        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn remove_todo(pool: &SqlitePool, chat_id: i64, task_id: i64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
DELETE FROM todos
WHERE chat_id = ?1 AND task_id = ?2
        "#,
        chat_id,
        task_id,
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn list_todos(pool: &SqlitePool, chat_id: i64) -> anyhow::Result<String> {
    let recs = sqlx::query!(
        r#"
SELECT task_id, description, done
FROM todos
WHERE chat_id = ?1
ORDER BY task_id
        "#,
        chat_id
    )
    .fetch_all(pool)
    .await?;

    // for rec in recs {
    //     println!("- [{}] {}: {}", if rec.done { "x" } else { " " }, rec.id,
    // &rec.description,); }

    Ok(recs
        .into_iter()
        .map(|rec| format!("{}) {}", rec.task_id, rec.description))
        .collect::<Vec<_>>()
        .join("\n"))
}
