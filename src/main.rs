use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration");
    let conn_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = &format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address)?;
    run(listener, conn_pool)?.await
}
