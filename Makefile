.PHONY: up
up:
	docker-compose --env-file .env.dist --progress plain up

.PHONY: down
down:
	docker-compose --env-file .env.dist --progress plain down

.PHONY: build
build:
	docker-compose --env-file .env.dist --progress plain build --no-cache