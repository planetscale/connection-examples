# Connect to PlanetScale from MySQL command line client

This example shows you how to connect to a PlanetScale database using the [MySQL command line client](https://dev.mysql.com/doc/refman/8.0/en/mysql.html).

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**MySQL**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connect to your database

There are two ways you can connect to the database: straight from the terminal or using a configuration file. Both options are covered below.

### Option 1: Values in terminal

1. Run the following in your terminal:

```bash
mysql -u [USERNAME] -h [HOSTNAME] -d [DATABASE] --password=[PASSWORD] --ssl-mode=VERIFY_IDENTITY --ssl-ca=[CERT_PATH]
```

2. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `--ssl-ca`.

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

### Option 2: Configuration file

You can also use a configuration file and then reference that with the client.

1. Add the following to your `.my.cnf` file:

```
[client_planetscale]
user=[USERNAME]
password=[PASSWORD]
host=[HOSTNAME]
ssl_mode=VERIFY_IDENTITY
ssl_ca=/etc/ssl/certs/ca-certificates.crt
```

2. Replace the placeholder values with the values copied in the previous section.
3. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `ssl_ca`.

This path depends on your system, so you need to paste in the appropriate value.

You can find some common examples in **Step 2 of Option 1** above or see our [Secure connections documentation](/concepts/secure-connections#ca-root-configuration) for more configuration information.

4. Use the following command to access the MySQL command line:
```bash
mysql --defaults-group-suffix=_planetscale
```

### MariaDB's `mysql` client

By default, [some Linux distributions](https://mariadb.com/kb/en/distributions-which-include-mariadb/) include the command-line client and libraries from MariaDB, which require a different argument to set the SSL mode. When connecting with `--ssl-verify-server-cert`, the client will use the system certificate roots even if you don't specify a `--ssl-ca` path.

```bash
mysql -u [USERNAME] -h [HOSTNAME] -d [DATABASE] --password=[PASSWORD] --ssl-verify-server-cert
```

## More resources

**Next steps**

- Learn more about the [PlanetScale workflow](https://planetscale.com/docs/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://planetscale.com/docs/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://planetscale.com/docs/concepts/deploy-requests).
