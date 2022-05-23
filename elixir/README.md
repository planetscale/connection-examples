# Connecting to PlanetScale from Elixir

This example demonstrates how to connect an Elixir application to a PlanetScale database using [Mix](https://elixir-lang.org/getting-started/mix-otp/introduction-to-mix.html).

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**Elixir**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

1. Copy the contents of [`elixir/example/connect.exs`](https://github.com/planetscale/examples/blob/main/elixir/example/connect.exs) into your connection file.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section.

## More resources

**Next steps**

- Learn more about the [PlanetScale workflow](https://docs.planetscale.com/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://docs.planetscale.com/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://docs.planetscale.com/concepts/deploy-requests).