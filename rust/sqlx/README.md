# Connecting to PlanetScale from Rust with sqlx

These examples demonstrate how  [sqlx](sqlx-example/src/main.rs) &mdash; Connect using the [`sqlx` pure Rust SQL crate](https://docs.rs/sqlx/latest/sqlx/).

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**Rust**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

1. Add the `sqlx` and `async-std` dependencies to your [`Cargo.toml` file](https://github.com/planetscale/examples/blob/main/rust/sqlx-example/Cargo.toml):
```rust
mysql_async = "*"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "time"] }
```
1. Copy the contents of [`rust/sqlx-example/src/main.rs`](https://github.com/planetscale/examples/blob/main/rust/sqlx-example/src/main.rs) into your connection file.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section.

## More resources

**Next steps**

- Learn more about the [PlanetScale workflow](https://docs.planetscale.com/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://docs.planetscale.com/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://docs.planetscale.com/concepts/deploy-requests).