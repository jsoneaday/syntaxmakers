image_name="postgres:14-alpine"

echo "clear docker images"
container_ids=$(docker ps -q --filter "ancestor=$image_name")
if [ -z "$container_ids" ]; then
  echo "No container id found using image: $image_name"
else
  echo "Stopping and removing containers using image: $image_name"
  docker stop $container_ids
  docker rm $container_ids
  rm -rf devdb
fi

echo "build new images"
docker compose -f docker-compose.dev.yml up -d --build

echo "run sqlx migrations"
sqlx database create --database-url postgres://syntaxmakers:syntaxmakers@localhost:5433/syntaxmakers
sqlx migrate run --database-url postgres://syntaxmakers:syntaxmakers@localhost:5433/syntaxmakers

echo "setup test data"
psql -h localhost -p 5433 -d syntaxmakers -U syntaxmakers -f ./tools/setup-dev-data.sql

echo "start running tests"
cargo test -- --nocapture

echo "start rust server locally (not docker)"
cargo run