# README

This README would normally document whatever steps are necessary to get the
application up and running.

Things you may want to cover:

* Ruby version

* System dependencies

* Configuration

* Database creation

* Database initialization

* How to run the test suite

* Services (job queues, cache servers, search engines, etc.)

* Deployment instructions

* ...

## Useful commands
```bash
rails new project_name

rails g scaffold ObjectName field:type field2:type

rails s

rails db:migrate

 rails g scaffold Post title:text body:text user_id:integer subreddit_id:integer

  780  rails g scaffold Post title:text body:text user_id:integer subreddit_id:integer
  781  rails g migration AddSlugToPosts slug:uniq 
  782  rails db:migrate
  783  history

```
## File directory:

- app/views/layout/application.html.erb //where the style cdn was added

## Gems 
- https://rubygems.org/ 
- Gemfile
