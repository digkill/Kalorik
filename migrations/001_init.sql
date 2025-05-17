-- 001_init.sql
CREATE TABLE IF NOT EXISTS users (
                                     id SERIAL PRIMARY KEY,
                                     chat_id BIGINT UNIQUE NOT NULL,
                                     username TEXT,
                                     age INT,
                                     weight_kg FLOAT,
                                     height_cm FLOAT,
                                     gender TEXT,
                                     activity_level TEXT,
                                     goal TEXT,
                                     imt FLOAT,
                                     created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                                     updated_at TIMESTAMPTZ
);
