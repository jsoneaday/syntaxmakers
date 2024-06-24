db_name="syntaxmakers-db"

echo "clear docker images"
container_ids=$(docker ps -q --filter "name=$db_name")
if [ -z "$container_ids" ]; then
  echo "No container id found using name: $db_name"
else
  echo "Stopping and removing containers using: $db_name"
  docker stop $container_ids
  docker rm $container_ids
  rm -rf dbdata
fi

echo "build new images"
docker compose -p syntaxmakers -f docker-compose.dev.yml up -d --build

echo "run sqlx migrations"
sqlx database create --database-url postgres://syntaxmakers:syntaxmakers@localhost:5433/syntaxmakers
sqlx migrate run --database-url postgres://syntaxmakers:syntaxmakers@localhost:5433/syntaxmakers

echo "setup test data"
psql postgresql://syntaxmakers:syntaxmakers@localhost:5433/syntaxmakers -f ./tools/setup-dev-data.sql

echo "start running tests"
cargo test -- --nocapture

# echo "start rust server locally (not docker)"
# cargo run
