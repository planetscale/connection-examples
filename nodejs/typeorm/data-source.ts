import { DataSource } from "typeorm"

export const AppDataSource = new DataSource({
    type: "mysql",
    host: "[HOSTNAME]",
    port: 3306,
    username: "[USERNAME]",
    password: "[PASSWORD]",
    database: "[DATABASE]",
    ssl: {
        rejectUnauthorized: true
    }
})
