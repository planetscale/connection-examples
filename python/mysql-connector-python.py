# pip install mysql-connector-python
import mysql.connector

config = {
    'user': '[USERNAME]',
    'password': '[PASSWORD]',
    'host': '[HOSTNAME]',
    'database': '[DATABASE]',
    'ssl_verify_identity': True,
    'ssl_ca': '/etc/ssl/cert.pem',
}

cnx = mysql.connector.connect(**config)
cnx.close()

print("Successfully connected to PlanetScale!")
