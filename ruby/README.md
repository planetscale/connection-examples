# Connecting to PlanetScale from Ruby

These examples demonstrate how to connect a Ruby application to a PlanetScale database using two different methods:

- [Rails](https://github.com/planetscale/connection-examples/tree/main/ruby/rails) &mdash; Connect a Rails application to PlanetScale
- [`mysql2` gem](https://github.com/planetscale/connection-examples/blob/main/ruby/mysql.rb) &mdash; Connect a Ruby application to PlanetScale using the [`mysql2` gem](https://github.com/brianmario/mysql2).

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**General**" or "**Rails**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

To connect, find the method you're connecting with below and follow the instructions in that section.

### Option 1: Rails

1. Copy the contents of [`rails/database.yml`](https://github.com/planetscale/connection-examples/blob/main/ruby/rails/database.yml) into your connection file.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section. We encourage you to move these placeholder values into your `.env` file.
3. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `sslca`.

This path depends on your system, so you need to paste in the appropriate value.

You can find configuration information for your system in our [Secure connections documentation](/concepts/secure-connections#ca-root-configuration).

### Option 2: `mysql2` for Ruby

1. Require `mysql2` in your Gemfile:
```ruby
gem "mysql2"
```
1. Copy the contents of [`mysql.rb`](https://github.com/planetscale/connection-examples/blob/main/ruby/mysql.rb) into your connection file.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section. We encourage you to move these placeholder values into your local environment variables file.
3. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `sslca`.

This path depends on your system, so you need to paste in the appropriate value.

You can find configuration information for your system in our [Secure connections documentation](/concepts/secure-connections#ca-root-configuration).

## More resources

**Rails resources**
- For more in-depth Rails instructions, check out the [Rails quickstart](https://docs.planetscale.com/tutorials/connect-rails-app).
- You can learn how to automatically copy Rails migration data during deployment in our [Automatic Rails migrations documentation](https://docs.planetscale.com/tutorials/automatic-rails-migrations).

**Next steps**

- Learn more about the [PlanetScale workflow](https://docs.planetscale.com/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://docs.planetscale.com/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://docs.planetscale.com/concepts/deploy-requests).