var mysql = require('mysql2');

var conn = mysql.createConnection({
  host: "[HOSTNAME]",
  user: "[USERNAME]",
  password: "[PASSWORD]",
  database: "[DATABASE]",
  ssl: {
    rejectUnauthorized: true
  }
});

conn.connect(function(err) {
  if (err) throw err;
  console.log("Succesfully connected to PlanetScale!");
  process.exit(0)
});
