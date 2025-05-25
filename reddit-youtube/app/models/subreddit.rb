class Subreddit < ApplicationRecord
  extend FriendlyId
  friendly_id :title, use: :slugged

      has_many :posts
      belongs_to :user

end
