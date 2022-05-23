package main

import (
	"database/sql"
	"fmt"

	"github.com/go-sql-driver/mysql"
)

// Use a DSN directly. Please note that it's not recommended
// to store the password directly in the code for production
// usage.
func withDSN() {
	db, err := sql.Open("mysql", "[USERNAME]:[PASSWORD]@tcp([HOSTNAME])/[DATABASE]?tls=true")
	if err != nil {
		panic(err)
	}
	err = db.Ping()
	if err != nil {
		panic(err)
	}
}

// Use a mysql.Config object to build the DSN. Please note that
// it's not recommended to store the password directly in the
// code for production usage.
func withConfig() {
	config := mysql.NewConfig()
	config.User = "[USERNAME]"
	config.Passwd = "[PASSWORD]"
	config.Net = "tcp"
	config.Addr = "[HOSTNAME]"
	config.DBName = "[DATABASE]"
	config.TLSConfig = "true"

	db, err := sql.Open("mysql", config.FormatDSN())
	if err != nil {
		panic(err)
	}
	err = db.Ping()
	if err != nil {
		panic(err)
	}
}

func main() {
	withConfig()
	fmt.Println("Successfully connected to PlanetScale with configuration object!")
	withDSN()
	fmt.Println("Successfully connected to PlanetScale with DSN!")
}
