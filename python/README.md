# Connecting to PlanetScale from Python

These examples demonstrate how to connect a Python application to a PlanetScale database using three different methods:

- [Django](https://github.com/planetscale/connection-examples/blob/main/python/django) &mdash; Connect a Django application to PlanetScale
- [`mysql-connector-python` driver](https://github.com/planetscale/connection-examples/blob/main/python/mysql-connector-python.py) &mdash; Connect a Python application to PlanetScale using [`mysql-connector-python`](https://dev.mysql.com/doc/connector-python/en).
- [MySQLdb interface](https://github.com/planetscale/connection-examples/blob/main/python/mysql.py) &mdash; Connect a Python application to PlanetScale using the [MySQLdb interface](https://mysqlclient.readthedocs.io/user_guide.htmln).

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**Python**" or "**Django**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

To connect, find the method you're connecting with below and follow the instructions in that section.

### Option 1: `mysql-connector-python`

1. Install `mysql-connector-python`:
```bash
pip install mysql-connector-python
```
2. Copy the contents of [`mysql-connector-python.py`](https://github.com/planetscale/connection-examples/blob/main/python/mysql-connector-python.py) into your connection file.
3. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section. We encourage you to move these placeholder values into your `.env` file.
4. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `ssl_ca` in the `config` object.

This path depends on your system, so you need to paste in the appropriate value.

You can find configuration information for your system in our [Secure connections documentation](/concepts/secure-connections#ca-root-configuration).

### Option 2: MySQLdb

1. Install `mysqlclient`:
```bash
pip install mysqlclient
```
2. Copy the contents of [`mysql.py`](https://github.com/planetscale/connection-examples/blob/main/python/mysql.py) into your connection file.
3. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section. We encourage you to move these placeholder values into your `.env` file.
4. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `ca` in the `ssl` object.

This path depends on your system, so you need to paste in the appropriate value.

You can find configuration information for your system in our [Secure connections documentation](/concepts/secure-connections#ca-root-configuration).

### Option 3: Django

1. Copy the contents of [`django/my.cnf`](https://github.com/planetscale/connection-examples/blob/main/python/django/my.cnf) into your connection file. This may be a `.env` file.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section..
3. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `ssl_ca`.

This path depends on your system, so you need to paste in the appropriate value.

You can find configuration information for your system in our [Secure connections documentation](/concepts/secure-connections#ca-root-configuration).

4. You can update your [`settings.py` file](https://github.com/planetscale/connection-examples/blob/main/python/django/settings.py) to read from your connection file, as shown in our example.
5. (Optional) If you're using Django's default migrations, you may run into issues while migrating, as PlanetScale doesn't support [foreign key constraints](https://docs.planetscale.com/learn/operating-without-foreign-key-constraints). We've created a [PlanetScale Django database wrapper](https://github.com/planetscale/django_psdb_engine.git) that you can pull into your project to disable foreign key constraints. 

For further implementation information, refer to the [custom database wrapper section of our Django quickstart](https://docs.planetscale.com/tutorials/connect-django-app#optional-%E2%80%94-bring-in-planetscale-custom-database-wrapper). 

## More resources

**Django resources**
- For more in-depth Django instructions, check out the [Django quickstart](https://docs.planetscale.com/tutorials/connect-django-app) or the [sample Django application](https://github.com/planetscale/django-example).

**Next steps**

- Learn more about the [PlanetScale workflow](https://docs.planetscale.com/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://docs.planetscale.com/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://docs.planetscale.com/concepts/deploy-requests).