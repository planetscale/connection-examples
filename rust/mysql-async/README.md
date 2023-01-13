# Connecting to PlanetScale from Rust with mysql-async

This example demonstrates how to connect a Rust application to a PlanetScale database using [mysql_async](src/main.rs).

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**Rust**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

1. Add the `mysql_async` and `tokio` dependencies to your [`Cargo.toml` file](https://github.com/planetscale/connection-examples/blob/main/rust/mysql-async/Cargo.toml):
```rust
mysql_async = "*"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "time"] }
```
2. Copy the contents of [`mysql-async/src/main.rs`](https://github.com/planetscale/connection-examples/blob/main/rust/mysql-async/src/main.rs) into your connection file.
3. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section.

## More resources

**Next steps**

- Learn more about the [PlanetScale workflow](https://planetscale.com/docs/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://planetscale.com/docs/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://planetscale.com/docs/concepts/deploy-requests).
