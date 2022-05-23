
fn main() {
    let url = "mysql://[USERNAME]:[PASSWORD]@[HOSTNAME]/[DATABASE]";
    let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(url).unwrap());
    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
    let _conn = pool.get_conn().unwrap();
    println!("Successfully connected to PlanetScale!");
}
