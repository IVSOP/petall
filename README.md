# PETALL

## Run Postgres

```
cd backend
docker compose up -d
```

## Run Migrations
```
cd backend
sqlx migrate run
cargo run -- seed
```

## Run Backend Server

Copy `.env.example` to `.env` and setup variables.

```
cd backend
cargo run -- run
```

## Run Frontend Server

```
cd frontend
bun run dev
```
