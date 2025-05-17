-- 001_init.sql
CREATE TABLE users
(
    id             SERIAL PRIMARY KEY,
    chat_id        BIGINT UNIQUE NOT NULL,
    username       TEXT,
    age            INTEGER,
    weight_kg      DOUBLE PRECISION,
    height_cm      DOUBLE PRECISION,
    gender         TEXT,
    activity_level TEXT,
    goal           TEXT,
    imt            DOUBLE PRECISION,
    created_at     TIMESTAMPTZ   NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ
);
