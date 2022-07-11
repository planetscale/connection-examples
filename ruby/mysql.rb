require "mysql2"

client = Mysql2::Client.new(
  host: "[HOSTNAME]",
  username: "[USERNAME]",
  password: "[PASSWORD]",
  database: "[DATABASE]",
  ssl_mode: :verify_identity
)

puts "Successfully connected to PlanetScale!"
