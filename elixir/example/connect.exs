defmodule Connect do

  def main do
    hostname = "[HOSTNAME]"

    {:ok, pid} = MyXQL.start_link(username: "[USERNAME]",
      database: "[DATABASE]",
      hostname: hostname,
      password: "[PASSWORD]",
      ssl: true,
      ssl_opts: [
        verify: :verify_peer,
        cacertfile: CAStore.file_path(),
        server_name_indication: String.to_charlist(hostname),
        customize_hostname_check: [
          match_fun: :public_key.pkix_verify_hostname_match_fun(:https)
        ]
      ]
    )

    {:ok, _res} = MyXQL.query(pid, "SELECT 1")
    IO.puts "Successfully connected to PlanetScale!"
  end
end

Connect.main
