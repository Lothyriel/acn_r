docker build . -t bot_acn:latest
docker run --env-file ./.env bot_acn