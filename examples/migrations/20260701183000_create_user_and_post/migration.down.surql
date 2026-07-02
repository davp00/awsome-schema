-- Migration: create_user_and_post_down
REMOVE TABLE likes;
REMOVE TABLE post;
REMOVE TABLE user;
