## create your docker image

docker build -f ./Dockerfile -t syntaxmakers:1.0 ..

## confirm if needed

docker images --filter reference=syntaxmakers

## setup registry (can be done with console as well)

aws ecr create-repository --repository-name <name> --region us-east-2

## tag your image with repositoryUri

docker tag <imagename:tag> 201558611471.dkr.ecr.us-east-2.amazonaws.com/syntaxmakers1

## aws configure (you will be asked to login to sso start url, https://d-9a6779dcda.awsapps.com/start)

aws configure sso

## login to cli sso session

aws ecr get-login-password --region us-east-2 --profile cli | docker login --username AWS --password-stdin 201558611471.dkr.ecr.us-east-2.amazonaws.com

## after building your package push it to repo

docker push 201558611471.dkr.ecr.us-east-2.amazonaws.com/syntaxmakers1
