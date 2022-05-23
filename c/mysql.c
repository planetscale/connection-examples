#include <stdio.h>
#include <mysql.h>

int main() {
    MYSQL *conn = mysql_init(NULL);

    int mode = SSL_MODE_VERIFY_IDENTITY;
    const char *ca = "/etc/ssl/certs/ca-certificates.crt";
    mysql_options(conn, MYSQL_OPT_SSL_MODE, &mode);
    mysql_options(conn, MYSQL_OPT_SSL_CA, ca);
    if(mysql_real_connect(conn, "[HOSTNAME]", "[USERNAME]", "[PASSWORD]", "[DATABASE]", 3306, NULL, 0) == NULL) {
        printf("%s\n", mysql_error(conn));
        exit(1);
    }

    mysql_close(conn);
    printf("Successfully connected to PlanetScale!\n");
    return 0;
}