# Connecting to PlanetScale from Rust

These examples demonstrate how to connect a Rust application to a PlanetScale database using three different methods:

- [`mysql`](mysql/src/main.rs) &mdash; Connect using the [`mysql` Pure Rust MySQL database driver](https://docs.rs/mysql/latest/mysql/).
- [mysql_async](mysql-async/src/main.rs) &mdash; Connect using the asynchronous [`mysql_async`](https://docs.rs/mysql_async/latest/mysql_async/) MySQL client library for Rust.
- [sqlx](sqlx/src/main.rs) &mdash; Connect using the [`sqlx` pure Rust SQL crate](https://docs.rs/sqlx/latest/sqlx/).

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**Rust**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

To connect, find the method you're connecting with below and follow the instructions in that section.

### Option 1: `mysql`

1. Add the `mysql` dependency to your [`Cargo.toml` file](https://github.com/planetscale/connection-examples/blob/main/rust/mysql/Cargo.toml):
```rust
mysql = "*"
```
2. Copy the contents of [`main.rs`](https://github.com/planetscale/connection-examples/blob/main/rust/mysql/src/main.rs) into your connection file.
3. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section.

### Option 2: `mysql_async`

1. Add the `mysql_async` and `tokio` dependencies to your [`Cargo.toml` file](https://github.com/planetscale/connection-examples/blob/main/rust/mysql-async/Cargo.toml):
```rust
mysql_async = "*"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "time"] }
```
2. Copy the contents of [`mysql-async/src/main.rs`](https://github.com/planetscale/connection-examples/blob/main/rust/mysql-async/src/main.rs) into your connection file.
3. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section.

### Option 3: `sqlx`

1. Add the `sqlx` and `async-std` dependencies to your [`Cargo.toml` file](https://github.com/planetscale/connection-examples/blob/main/rust/sqlx/Cargo.toml):
```rust
mysql_async = "*"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "time"] }
```
1. Copy the contents of [`rust/sqlx/src/main.rs`](https://github.com/planetscale/connection-examples/blob/main/rust/sqlx/src/main.rs) into your connection file.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section.

## More resources

**Next steps**

- Learn more about the [PlanetScale workflow](https://docs.planetscale.com/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://docs.planetscale.com/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://docs.planetscale.com/concepts/deploy-requests).
