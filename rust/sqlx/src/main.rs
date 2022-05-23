#[async_std::main]

async fn main() -> Result<(), sqlx::Error> {
    let url = "mysql://[USERNAME]:[PASSWORD]@[HOSTNAME]/[DATABASE]?ssl_mode=verify_identity";
    let _pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(url).await?;

    println!("Successfully connected to PlanetScale!");
    Ok(())
}
