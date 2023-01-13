# Connecting to PlanetScale from Go

This example demonstrates how to connect a Go application to a PlanetScale database using [`go-sql-driver/mysql`](https://github.com/go-sql-driver/mysql).

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**Go**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

1. Add the `go-sql-driver/mysql` dependency to your [`go.mod` file](https://github.com/planetscale/connection-examples/blob/main/go/go.mod).
2. Copy the contents of [`example.go`](https://github.com/planetscale/connection-examples/blob/main/go/example.go) into your connection file.
3. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section. We encourage you to move these placeholder values into your local environment variables file.
4. (Optional) PlanetScale does not support [foreign key constraints](https://planetscale.com/docs/learn/operating-without-foreign-key-constraints). If you're using GORM, you'll need to disable foreign key constraints in your connection file:

```go
DisableForeignKeyConstraintWhenMigrating: true,
```

You can find more information in the [Foreign key constraints section of the Go quickstart](https://planetscale.com/docs/tutorials/connect-go-app#foreign-key-constraints).

## More resources

**Go resources**
- For more in-depth Go instructions, check out the [Go quickstart](https://planetscale.com/docs/tutorials/connect-go-app) or the [sample Go application](https://github.com/planetscale/golang-example).

**Next steps**

- Learn more about the [PlanetScale workflow](https://planetscale.com/docs/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://planetscale.com/docs/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://planetscale.com/docs/concepts/deploy-requests).
