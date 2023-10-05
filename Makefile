IMAGE_REPOSITORY=docker.io/andreclaudino/qdrant-uploader
PROJECT_VERSION := $$(cat Cargo.toml | grep version | head -n 1 | awk '{print $$3}' | sed -r 's/^"|"$$//g')
IMAGE_NAME=$(IMAGE_REPOSITORY):$(PROJECT_VERSION)
GIT_REFERENCE := $$(git log -1 --pretty=%h)

run-storage:
	mkdir -p $(PWD)/qdrant_storage
	podman run \
		-e QDRANT__SERVICE__GRPC_PORT="6334" \
		-p 6333:6333 \
		-p 6334:6334 \
  		-v $(PWD)/qdrant_storage:/qdrant/storage:z \
			docker.io/qdrant/qdrant


flags/create:
	mkdir -p flags/
	touch flags/create


flags/build: flags/create
	podman build -t $(IMAGE_REPOSITORY):latest -f docker/Dockerfile . \
		--build-arg GIT_REFERENCE=$(GIT_REFERENCE) \
		--build-arg VERSION=$(PROJECT_VERSION)
	touch flags/build


flags/login: flags/create
	podman login docker.io
	touch flags/login


flags/tag: flags/build
	podman tag $(IMAGE_REPOSITORY):latest $(IMAGE_NAME)
	touch flags/tag


flags/push: flags/login flags/tag
	podman push $(IMAGE_REPOSITORY):latest
	podman push $(IMAGE_NAME)


clean:
	rm -rf flags/