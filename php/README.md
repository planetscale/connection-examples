# Connecting to PlanetScale from PHP

These examples demonstrate how to connect a PHP application to a PlanetScale database using two different methods:

- [PDO](https://github.com/planetscale/connection-examples/blob/main/php/pdo.php) &mdash; Connect using the [PHP PDO class](https://www.php.net/manual/en/class.pdo.php)
- [MySQLi](https://github.com/planetscale/connection-examples/blob/main/php/mysqli.php) &mdash; Connect using the [PHP `mysqli` extension](https://www.php.net/manual/en/book.mysqli.php)

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**PHP**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

1. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values. We encourage you to move these placeholder values into your `.env` file.
2. To ensure a secure connection, you **must** fill in the SSL certificate path.
   - [**PDO option**](https://github.com/planetscale/connection-examples/blob/main/php/pdo.php): This is the `PDO::MYSQL_ATTR_SSL_CA` value.
   - [**MySQLi option**](https://github.com/planetscale/connection-examples/blob/main/php/mysqli.php): This is the third parameter in the `ssl_set()` method.

This path depends on your system, so you need to paste in the appropriate value.

Here are some of the paths for commonly used systems:

**MacOS / FreeBSD / OpenBSD**

```
/etc/ssl/cert.pem
```

**RedHat / Fedora / CentOS / Mageia / Vercel / Netlify**

```
/etc/pki/tls/certs/ca-bundle.crt
```

**Debian / Ubuntu / Gentoo / Arch / Slackware**

```
/etc/ssl/certs/ca-certificates.crt
```

**Alpine**

```
/etc/ssl/cert.pem
```

**OpenSUSE**

```
/etc/ssl/ca-bundle.pem
```

You can find more configuration information in our [Secure connections documentation](/concepts/secure-connections#ca-root-configuration). If you're on Windows, refer to the "Windows" section of that doc.

## More resources

**PlanetScale with PHP**

- For more in depth instructions, check out the [PHP quickstart](https://docs.planetscale.com/tutorials/connect-php-app) or the [sample PHP application](https://github.com/planetscale/php-example).
- If you're looking for Laravel instructions, see the [Laravel quickstart](https://docs.planetscale.com/tutorials/connect-laravel-app) or the [sample Laravel application](https://github.com/planetscale/laravel-example).
- If you're looking for Symfony instructions, see the [Symfony quickstart](https://docs.planetscale.com/tutorials/connect-symfony-app) or the [sample Symfony application](https://github.com/planetscale/symfony-example).

**PHP blog posts**
- [Build a Laravel application with a MySQL (PlanetScale) database](https://planetscale.com/blog/build-a-laravel-application-with-a-mysql-database)
- [Introduction to Laravel caching (w/ PlanetScale demo)](https://planetscale.com/blog/introduction-to-laravel-caching)

**Next steps**

- Learn more about the [PlanetScale workflow](https://docs.planetscale.com/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://docs.planetscale.com/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://docs.planetscale.com/concepts/deploy-requests).