ARG DATABASE_PATH='./db/sqlite.db'

# Builder
# -------
FROM rust:1.63.0-bullseye AS builder
RUN cargo install sqlx-cli

ARG DATABASE_PATH
ENV DATABASE_URL=sqlite:file:$DATABASE_PATH

COPY . .

RUN cargo sqlx database setup --source db/migrations &&\
    cargo build --release

# Runner
# ------
FROM debian:bullseye-slim AS runner

ARG DATABASE_PATH
ENV DATABASE_URL=sqlite:file:$DATABASE_PATH

COPY --from=builder $DATABASE_PATH $DATABASE_PATH
COPY --from=builder ./target/release/graphql ./graphql

CMD ["./graphql"]
