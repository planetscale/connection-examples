# Connecting to PlanetScale from Node.js

These examples demonstrate how to connect a Node application to a PlanetScale database using two different methods:

- [Node.js](https://github.com/planetscale/connection-examples/blob/main/nodejs/mysql.js) &mdash; [`mysql2` package](https://github.com/sidorares/node-mysql2)
- [Prisma + Node.js](https://github.com/planetscale/connection-examples/tree/main/nodejs/prisma)

Follow the instructions below to find and insert your PlanetScale credentials.

## Getting the credentials

1. In the [PlanetScale dashboard](https://app.planetscale.com), select the database you want to connect to.
2. Click "**Branches**" and select the branch you want to connect to.
3. Click "**Connect**".
4. Select "**Node.js**" or "**Prisma**" from the "**Connect from**" dropdown.
5. If the password is blurred, click "**New password**".
6. Copy the credentials. You won't be able to see the password again.

## Connecting your database

If you're just connecting a Node application, proceed with Option 1. If you're using Prisma, skip to Option 2.

### Option 1: Node.js

1. Copy the connection information from the [`mysql.js` file](https://github.com/planetscale/connection-examples/blob/main/nodejs/mysql.js) into your application.
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values from the previous section. We encourage you to move these placeholder values into your `.env` file.

### Option 2: Prisma

1. Open your `schema.prisma` file and replace `datasource db {}` with the code in our [`schema.prisma` example file](https://github.com/planetscale/connection-examples/blob/main/nodejs/prisma/schema.prisma).
2. Replace the placeholders for `HOSTNAME`, `DATABASE`, `USERNAME`, and `PASSWORD` with the copied values. We encourage you to move these placeholder values into your `.env` file.

## More resources

**Express resources**
- For more in-depth Node.js instructions, check out the [Express quickstart](https://planetscale.com/docs/tutorials/connect-nodejs-app) or the [sample Express application](https://github.com/planetscale/express-example).

**Express blog posts**
- [Create a Harry Potter API with Node.js/Express, MySQL, and PlanetScale](https://planetscale.com/blog/create-a-harry-potter-api-with-node-js-express-mysql-and-planetscale)

**Prisma resources**
- If you're looking for in-depth Prisma instructions, see the [Prisma quickstart](https://planetscale.com/docs/tutorials/prisma-quickstart).
- To use the Prisma Data Platform integration, check out the [PlanetScale Prisma Data Platform documentation](https://planetscale.com/docs/tutorials/prisma-data-platform-integration).
- See the [Automatic Prisma migrations documentation](https://planetscale.com/docs/tutorials/automatic-prisma-migrations) for information about automating Prisma migrations

**Prisma blog posts**

- [How to set up a Next.js application with Prisma and PlanetScale](https://planetscale.com/blog/how-to-setup-next-js-with-prisma-and-planetscale)
- [How to seed a database with Prisma and Next.js](https://planetscale.com/blog/how-to-seed-a-database-with-prisma-and-next-js)
- [Building a Next.js app with Netlify, NextAuth.js, Prisma, and a PlanetScale serverless database](https://planetscale.com/blog/nextjs-netlify-planetscale-starter-app)
- [Introducing Prisma’s Data Platform PlanetScale integration](https://planetscale.com/blog/planetscale-mysql-database-on-prisma-platform)

**Prisma videos**
- [Developer-owned databases: a new frontier? - Liz van Dijk I Prisma Day 2022](https://www.youtube.com/watch?v=HDclqWeYt5M)

**Next steps**

- Learn more about the [PlanetScale workflow](https://planetscale.com/docs/concepts/planetscale-workflow).
- Learn how to [branch your PlanetScale database](https://planetscale.com/docs/concepts/branching) for development.
- Learn how to ship schema changes to production with [deploy requests](https://planetscale.com/docs/concepts/deploy-requests).
