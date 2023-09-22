-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DO $$ BEGIN
  IF NOT EXISTS (SELECT FROM pg_type WHERE typname = 'Transaction') THEN
    CREATE TYPE "Transaction" AS ENUM ('EXPENSE', 'INCOME');
  END IF;

  IF NOT EXISTS (SELECT FROM pg_type WHERE typname = 'Role') THEN
    CREATE TYPE "Role" AS ENUM ('CLIENT', 'ADMIN');
  END IF;
END $$;

CREATE TABLE IF NOT EXISTS "users" (
  id           uuid            PRIMARY KEY DEFAULT uuid_generate_v4(),
  firstname    TEXT,
  lastname     TEXT,
  username     TEXT            NOT NULL,
  email        TEXT            NOT NULL,
  role         "Role"          NOT NULL DEFAULT 'CLIENT',
  transactions uuid[]          NOT NULL DEFAULT '{}',
  password     TEXT,
  created_at   TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE,
  updated_at   TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE
);

CREATE TABLE IF NOT EXISTS "transactions" (
  id          uuid            PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id     uuid            NOT NULL,
  account_id  uuid            NOT NULL,
  type        "Transaction"   NOT NULL,
  amount      BIGINT          NOT NULL DEFAULT 0,
  created_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE,
  updated_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE
);

CREATE TABLE IF NOT EXISTS "accounts" (
  id          uuid            PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id     uuid            NOT NULL,
  balance     BIGINT          NOT NULL DEFAULT 0,
  created_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE,
  updated_at  TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_DATE
);

CREATE UNIQUE INDEX "users_id_key" ON "users"("id");
CREATE UNIQUE INDEX "transactions_id_key" ON "transactions"("id");
CREATE UNIQUE INDEX "accounts_id_key" ON "accounts"("id")
