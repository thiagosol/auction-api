use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn init_db() -> PgPool {
    let db_user = std::env::var("DB_POSTGRES_USER").expect("DB_POSTGRES_USER must be set");
    let db_pass = std::env::var("DB_POSTGRES_PASS").expect("DB_POSTGRES_PASS must be set");
    
    let database_url = format!(
        "postgres://{}:{}@thiagosol.com:5432/auction",
        db_user, db_pass
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados")
}
