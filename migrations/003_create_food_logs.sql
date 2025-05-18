-- migrations/20240518_create_food_logs.sql
CREATE TABLE IF NOT EXISTS food_logs (
                                         id SERIAL PRIMARY KEY,
                                         chat_id BIGINT NOT NULL,
                                         food_name TEXT NOT NULL,
                                         calories REAL,
                                         proteins REAL,
                                         fats REAL,
                                         carbs REAL,
                                         created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    );
