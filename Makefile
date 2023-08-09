docker-build:
	docker build --tag js-engine-from-scratch .

docker-container:
	docker create --tty --interactive \
		--name js-engine-from-scratch \
		--hostname js-engine-from-scratch \
		--volume ${PWD}/:/usr/src/myapp \
		--publish 9228:9228 \
		js-engine-from-scratch

docker-clean:
	docker rm js-engine-from-scratch || echo "no container"
	docker rmi js-engine-from-scratch || echo "no image"