package com.planetscale.example;

import java.sql.*;

class Mysql {
    public static void main(String args[]){
        try {
            Class.forName("com.mysql.cj.jdbc.Driver");
            Connection conn = DriverManager.getConnection(
                "jdbc:mysql://[HOSTNAME]/[DATABASE]?sslMode=VERIFY_IDENTITY",
                "[USERNAME]",
                "[PASSWORD]");
            conn.close();
            System.out.println("Successfully connected to PlanetScale!");
        } catch(Exception e) {
            e.printStackTrace();
            System.exit(1);
        }
    }
}
