# Connecting to PlanetScale from Django

This example demonstrates how to connect a Django application to a PlanetScale database.

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**Django**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

1. Copy the contents of [`django/my.cnf`](https://github.com/planetscale/connection-examples/blob/main/python/django/my.cnf) into your connection file. This may be a `.env` file.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section..
3. To ensure a secure connection, you **must** fill in the SSL certificate path. This is the value for `ssl_ca`.

This path depends on your system, so you need to paste in the appropriate value.

You can find configuration information for your system in our [Secure connections documentation](/concepts/secure-connections#ca-root-configuration).

4. You can update your [`settings.py` file](https://github.com/planetscale/connection-examples/blob/main/python/django/settings.py) to read from your connection file, as shown in our example.
5. (Optional) If you're using Django's default migrations, you may run into issues while migrating, as PlanetScale doesn't support [foreign key constraints](https://planetscale.com/docs/learn/operating-without-foreign-key-constraints). We've created a [PlanetScale Django database wrapper](https://github.com/planetscale/django_psdb_engine.git) that you can pull into your project to disable foreign key constraints.

For further implementation information, refer to the [custom database wrapper section of our Django quickstart](https://planetscale.com/docs/tutorials/connect-django-app#optional-%E2%80%94-bring-in-planetscale-custom-database-wrapper).

## More resources

**Django resources**
- For more in-depth Django instructions, check out the [Django quickstart](https://planetscale.com/docs/tutorials/connect-django-app) or the [sample Django application](https://github.com/planetscale/django-example).

**Next steps**

- Learn more about the [PlanetScale workflow](https://planetscale.com/docs/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://planetscale.com/docs/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://planetscale.com/docs/concepts/deploy-requests).
