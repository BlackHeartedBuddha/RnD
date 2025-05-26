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

  790  rails activatge_storage:install
  791  rails active_storage:install
  792  rails db:migrate
  793  git status

 800  rails g model vote user:references subreddit:references
  801  rails db:migrate
  802  rails g migration AddPostIdToVotes post post_id:integer
  803  rails db:migrate
  804  V
  805  rails generate migration AddValueToVotes value:integer
  806  rails db:migrate
  807  rails generate migration RemoveSubredditIdFromVotes subreddit_id:integer
  808  rails db:migrate

  817  rails g model comment content:string post_id:integer user_id:integer
  818  rails db:migrate

  827  rails g scaffold membership user_id:integer subreddit_id:string
  828  rails db:migrate
``` ## File directory:

- app/views/layout/application.html.erb //where the style cdn was added

## Gems 
- https://rubygems.org/ 
- Gemfile
