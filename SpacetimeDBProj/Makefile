DOCKER_CMD := docker-compose -f docker-compose.yml
CONTAINER_CMD := docker exec -it spacetimedb

build:
	$(DOCKER_CMD) build

up:
	$(DOCKER_CMD) up -d

down:
	$(DOCKER_CMD) down

app:
	$(CONTAINER_CMD) bash
