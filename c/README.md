# Connecting to PlanetScale from C

This example demonstrates how to connect a C application to a PlanetScale database using the [`libmysqlclient`](https://dev.mysql.com/downloads/c-api/).

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**General**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

1. Copy the contents of [`mysql.c`](https://github.com/planetscale/connection-examples/blob/main/c/mysql.c) into your connection file.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section.
3. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `*ca`.

This path depends on your system, so you need to paste in the appropriate value.

You can find configuration information for your system in our [Secure connections documentation](/concepts/secure-connections#ca-root-configuration).

## More resources

**Next steps**

- Learn more about the [PlanetScale workflow](https://docs.planetscale.com/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://docs.planetscale.com/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://docs.planetscale.com/concepts/deploy-requests).