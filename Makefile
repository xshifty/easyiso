.PHONY: up
up: .env
	docker compose --progress plain up

.PHONY: updb
updb: .env
	docker compose --progress plain up db

.PHONY: down
down:
	docker compose --progress plain down

.PHONY: build
build: .env
	docker compose --progress plain build --no-cache

.PHONY: bup
bup: .env build
	docker compose --progress plain up

.PHONY: bupdb
bupdb: .env
	docker compose --progress plain up db --build

.PHONY: wipedb
wipedb: down
	rm -rf docker/db
	mkdir docker/db
	chown -R ${USER}:${USER} docker/db

.PHONY: migration
migration:
	docker compose --progress plain run migrations migration run

.PHONY: env
env:
	cat .env.dist > .env

.env: env