-- файл миграции: 2025-05-18-01_add_subscription_ends_at_to_users.sql

ALTER TABLE users
    ADD COLUMN subscription_ends_at TIMESTAMP WITH TIME ZONE;
