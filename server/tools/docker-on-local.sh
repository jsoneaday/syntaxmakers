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
