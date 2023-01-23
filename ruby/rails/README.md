# Connecting to PlanetScale from Rails

This example demonstrates how to connect a Rails application to a PlanetScale database.

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**Rails**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

1. Copy the contents of [`rails/database.yml`](https://github.com/planetscale/connection-examples/blob/main/ruby/rails/database.yml) into your connection file.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section. We encourage you to move these placeholder values into your `.env` file.
3. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `sslca`.

This path depends on your system, so you need to paste in the appropriate value.

You can find configuration information for your system in our [Secure connections documentation](https://planetscale.com/docs/concepts/secure-connections#ca-root-configuration).

## More resources

**Rails resources**
- For more in-depth Rails instructions, check out the [Rails quickstart](https://planetscale.com/docs/tutorials/connect-rails-app).
- You can learn how to automatically copy Rails migration data during deployment in our [Automatic Rails migrations documentation](https://planetscale.com/docs/tutorials/automatic-rails-migrations).

**Next steps**

- Learn more about the [PlanetScale workflow](https://planetscale.com/docs/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://planetscale.com/docs/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://planetscale.com/docs/concepts/deploy-requests).
