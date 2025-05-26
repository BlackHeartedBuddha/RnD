defmodule ForumWeb.PageController do
  use ForumWeb, :controller

  def home(conn, _params) do
    # The home page is often custom made,
    # so skip the default app layout.
    render(conn, :home, layout: false)
  end

  def users(conn, _params) do
   # IO.puts("Users function hits")
    users = [
    %{id: 1, name: "alice", email: "alice@email.com"},
    %{id: 2, name: "Bob", email: "bob@email.com"},
    ]
    render(conn, :users, users: users)
  end

  def users_json(conn, _params) do
   # IO.puts("Users function hits")
    users = [
    %{id: 1, name: "alice", email: "alice@email.com"},
    %{id: 2, name: "Bob", email: "bob@email.com"},
    ]
    json(conn,  %{users: users})
  end

end
