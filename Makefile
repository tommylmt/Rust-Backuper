include .env.local

DOCKER_COMP = docker compose
RUST = $(DOCKER_COMP) exec app

build:
	@$(DOCKER_COMP) build --pull --build-arg NO_CACHE=0

build-no-cache:
	@$(DOCKER_COMP) build --pull --no-cache

up:
	@$(eval env ?=)
	@$(eval o ?=)
	@$(DOCKER_COMP) --env-file $(if $(env),$(env),'.env.local') up --detach $(o)

start: build up

down:
	@$(DOCKER_COMP) down --remove-orphans

logs:
	@$(DOCKER_COMP) logs --tail=0 --follow

rust:
	@$(RUST) bash
