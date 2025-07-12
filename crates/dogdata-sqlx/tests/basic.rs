use sqlx::PgPool;

#[sqlx::test(fixtures("users"))]
async fn test_query(pool: PgPool) {
    let result = dogdata_sqlx::instrumented_query!("SELECT * FROM user")
        .fetch_all(&pool)
        .await;

    assert!(result.is_ok());
}
