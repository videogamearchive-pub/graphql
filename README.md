# videogamearchivepub

```sh
source ./env

cargo sqlx database drop -y
cargo sqlx database setup --source db/migrations
cargo run

open localhost:8080
```

---

```sh
docker build --tag graphql .
docker run \
  --rm \
  --interactive \
  --tty \
  --env PORT=8081 \
  --publish 8082:8081 \
  graphql

open localhost:8082
```
