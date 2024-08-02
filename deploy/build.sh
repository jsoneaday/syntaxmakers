docker rmi $(docker images -q) -f
docker build -f ./Dockerfile -t syntaxmakers:1.0 ..
docker tag syntaxmakers:1.0 201558611471.dkr.ecr.us-east-2.amazonaws.com/syntaxmakers1
aws ecr get-login-password --region us-east-2 --profile cli | docker login --username AWS --password-stdin 201558611471.dkr.ecr.us-east-2.amazonaws.com
docker push 201558611471.dkr.ecr.us-east-2.amazonaws.com/syntaxmakers1