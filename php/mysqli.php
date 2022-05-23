<?php
error_reporting(E_STRICT);

$mysqli = mysqli_init();
$mysqli->ssl_set(NULL, NULL, "/etc/ssl/certs/ca-certificates.crt", NULL, NULL);
$mysqli->real_connect('[HOSTNAME]', '[USERNAME]', '[PASSWORD]', '[DATABASE]');
$mysqli->close();
?>
Successfully connected to PlanetScale!
