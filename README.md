# Example client implementations to connect to PlanetScale

This repository contains small example applications that connect to
PlanetScale in different languages and frameworks.

These examples ensure a minimal but also secure configuration. Usage of
TLS is highly language and framework specific, but these examples show
the safe and correct way to connect to PlanetScale with TLS certificate
validation enabled to prevent any man-in-the-middle attacks.

## Languages / frameworks

- [MySQL command line client](mysql)
- [C](c)
- [C#](c%23)
- [Elixir](elixir)
- [Go](go)
- [Java](java)
- [Node.js](nodejs)
  - [Prisma](nodejs/prisma)
- [PHP](php)
- [Python](python)
  - [Django](python/django)
- [Ruby](ruby)
  - [Rails](ruby/rails)
- [Rust](rust)
  - [mysql](rust/mysql)
  - [mysql-async](rust/mysql-async)
  - [sqlx](rust/sqlx)

Aside from this repository, we also have more extensive guides in their own
repositories for:

- [Django](https://github.com/planetscale/django-example)
- [Express.js](https://github.com/planetscale/express-example)
- [Laravel](https://github.com/planetscale/laravel-crud-mysql)
- [Symfony](https://github.com/planetscale/symfony-example)
- [PHP](https://github.com/planetscale/php-example)
- [Go](https://github.com/planetscale/go-example)
- [Vercel](https://github.com/planetscale/vercel-integration-example)

To learn how to use PlanetScale, please check out our
[documentation](https://docs.planetscale.com).

## Need help?

If you're having trouble getting things to run or you find yourself mingling
with obscure error messages, [get in touch with our Support team](http://planetscale.com/support) or [contact us on Twitter](https://twitter.com/planetscalehelp)! We're
happy to help!

## Further reading

- [The MySQL 8.0 reference manual](https://dev.mysql.com/doc/refman/8.0/en/)
- [Vitess v13.0 MySQL compatibility](https://vitess.io/docs/13.0/reference/compatibility/mysql-compatibility/)

## License

See [LICENSE.md](LICENSE.md)