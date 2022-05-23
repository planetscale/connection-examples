using System;
using System.Data;

using MySql.Data;
using MySql.Data.MySqlClient;

public class Mysql
{
    public static void Main()
    {
        string connStr = "server=[HOSTNAME];user=[USERNAME];database=[DATABASE];port=3306;password=[PASSWORD];SslMode=VerifyFull";
        MySqlConnection conn = new MySqlConnection(connStr);
        try {
            conn.Open();
            conn.Close();
            Console.WriteLine("Successfully connected to PlanetScale!");
        }
        catch (Exception ex) {
            Console.WriteLine(ex.ToString());
        }
    }
}
