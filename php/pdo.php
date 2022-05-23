<?php
error_reporting(E_STRICT);

$dsn = "mysql:host=[HOSTNAME];dbname=[DATABASE]";
$options = array(
  PDO::MYSQL_ATTR_SSL_CA => "/etc/ssl/certs/ca-certificates.crt",
);

$pdo = new PDO($dsn, "[USERNAME]", "[PASSWORD]", $options);
?>
Successfully connected to PlanetScale!
