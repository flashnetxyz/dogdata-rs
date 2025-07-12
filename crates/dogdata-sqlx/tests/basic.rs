use sqlx::PgPool;

#[sqlx::test(fixtures("users"))]
async fn test_query_transparent_types(pool: PgPool) {
    let result = dogdata_sqlx::instrumented_query!("SELECT username FROM \"user\"")
        .fetch_all(&pool)
        .await;

    assert!(result.is_ok());

    let result = dogdata_sqlx::instrumented_query_scalar!("SELECT username FROM \"user\"")
        .fetch_all(&pool)
        .await;

    assert!(result.is_ok());
}
