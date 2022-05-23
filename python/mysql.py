import MySQLdb

db = MySQLdb.connect(
    host     = "[HOSTNAME]",
    user     = "[USERNAME]",
    passwd   = "[PASSWORD]",
    db       = "[DATABASE]",
    ssl_mode = "VERIFY_IDENTITY",
    ssl      = {
        "ca": "/etc/ssl/certs/ca-certificates.crt"
    })

print("Successfully connected to PlanetScale!")