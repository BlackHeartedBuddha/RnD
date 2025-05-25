class AddPostIdToVotes < ActiveRecord::Migration[8.0]
  def change
    add_column :votes, :post, :string
    add_column :votes, :post_id, :integer
  end
end
