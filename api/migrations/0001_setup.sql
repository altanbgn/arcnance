CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DO $$ BEGIN
  IF NOT EXISTS (SELECT FROM pg_type WHERE typname = 'Transaction') THEN
    CREATE TYPE "Transaction" AS ENUM ('EXPENSE', 'INCOME');
  END IF;
END $$;

CREATE TABLE IF NOT EXISTS "transactions" (
  id          uuid            UNIQUE PRIMARY KEY DEFAULT uuid_generate_v4(),
  type        "Transaction"   NOT NULL,
  amount      BIGINT          NOT NULL DEFAULT 0,
  created_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE,
  updated_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE
);

CREATE TABLE IF NOT EXISTS "account" (
  id          uuid            UNIQUE PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id     uuid            NOT NULL,
  balance     BIGINT          NOT NULL DEFAULT 0,
  created_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE,
  updated_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE
);

CREATE TABLE IF NOT EXISTS "users" (
  id          uuid            UNIQUE PRIMARY KEY DEFAULT uuid_generate_v4(),
  first_name  TEXT,
  last_name   TEXT,
  user_name   TEXT            NOT NULL UNIQUE,
  email       TEXT            NOT NULL,
  password    TEXT,
  created_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE,
  updated_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE
);
