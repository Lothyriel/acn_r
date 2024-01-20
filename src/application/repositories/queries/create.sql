CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS user_activities (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  guild_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
  date TIMESTAMPTZ NOT NULL,
  activity_type TEXT NOT NULL
);
