# Forum

To start your Phoenix server:

  * Run `mix setup` to install and setup dependencies
  * Start Phoenix endpoint with `mix phx.server` or inside IEx with `iex -S mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

Ready to run in production? Please [check our deployment guides](https://hexdocs.pm/phoenix/deployment.html).

## Learn more

  * Official website: https://www.phoenixframework.org/
  * Guides: https://hexdocs.pm/phoenix/overview.html
  * Docs: https://hexdocs.pm/phoenix
  * Forum: https://elixirforum.com/c/phoenix-forum
  * Source: https://github.com/phoenixframework/phoenix

## Useful commands
```bash
mix phx.new forum --database sqlite3 
# ORM of phoenix
mix ecto.create

mix phx.server

mix format

mix deps.get

mix phx.gen.json Posts Post posts body:string title:string

mix ecto.migrate

  854  curl -X POST http://localhost:4000/api/posts   -H "Content-Type: application/json"   -d '{"post": {"title": "Second Title", "body": "This is the body of the post."}}'
  855  history

  862  mix phx.gen.json Accounts User users name:string email:string:unique
  863  mix ecto.migrate

  871  mix ecto.gen.migration add_user_id_to_posts
  872  mix ecto.migrate
  873* curl -X POST http://localhost:4000/api/posts   -H "Content-Type: application/json"   -d '{"user": {"name": "Username", "email": "test@mail.com"}}'
  874  curl -X POST http://localhost:4000/api/posts   -H "Content-Type: application/json"   -d '{"post": {"title": "Third Title", "body": "This is the body of the post.", "user_id" : 1}}'
  875  curl -X POST http://localhost:4000/api/posts   -H "Content-Type: application/json"   -d '{"post": {"title": "Fourth Post", "body": "This is the body of the post.", "user_id" : 1}}'
  876  git status

```
