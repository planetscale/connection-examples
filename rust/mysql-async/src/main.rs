#[tokio::main]
async fn main() -> Result<(), mysql_async::Error> {
    let url = "mysql://[USERNAME]:[PASSWORD]@[HOSTNAME]/[DATABASE]";
    let builder = mysql_async::OptsBuilder::from_opts(mysql_async::Opts::from_url(url).unwrap());
    let pool = mysql_async::Pool::new(builder.ssl_opts(mysql_async::SslOpts::default()));
    let _conn = pool.get_conn().await?;
    println!("Successfully connected to PlanetScale!");
    Ok(())
}
