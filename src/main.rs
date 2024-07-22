use axum::{response::Html, routing::get, Extension};
use sqlx::PgPool;

async fn index() -> Html<String> {
    Html("<h1>Hello world!</h1>".to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();

    let server_port_address = std::env::var("SERVER")?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    color_eyre::install().expect("Error with starting color eyre hook...");

    tracing_subscriber::fmt::init();

    let extension_pool = pool.clone();
    let connection = tokio::net::TcpListener::bind(server_port_address).await?;

    let router = axum::Router::new()
        .route("/", get(index))
        .layer(Extension(extension_pool));

    tracing::info!("Start server...");
    axum::serve(connection, router).await?;

    Ok(())
}
