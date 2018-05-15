NGINX_VER = 1.13.7
TAG=dev
RUST_COMPILER_TAG = 1.26.0
DOCKER_REPO=nginmesh
export HOST_PROJ_DIR=$(shell PWD)
UNAME_S := $(shell uname -s)
GIT_COMMIT=$(shell git rev-parse --short HEAD)
NGX_DEBUG="--with-debug"
export MODULE_DIR=${PWD}
NGX_MODULES = --with-compat  --with-threads --with-http_addition_module \
     --with-http_auth_request_module   --with-http_gunzip_module --with-http_gzip_static_module  \
     --with-http_random_index_module --with-http_realip_module --with-http_secure_link_module \
     --with-http_slice_module  --with-http_stub_status_module --with-http_sub_module \
     --with-stream --with-stream_realip_module --with-stream_ssl_preread_module
ifeq ($(UNAME_S),Linux)
    NGINX_SRC += nginx-linux
    NGX_OPT= $(NGX_MODULES) \
       --with-file-aio
       --with-cc-opt='-g -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC' \
       --with-ld-opt='-Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie'
endif
ifeq ($(UNAME_S),Darwin)
    NGINX_SRC += nginx-darwin
    NGX_OPT= $(NGX_MODULES)
endif
DOCKER_BUILD=./docker
DOCKER_MODULE_IMAGE = $(DOCKER_REPO)/${MODULE_NAME}
DOCKER_MODULE_BASE_IMAGE = $(DOCKER_REPO)/${MODULE_NAME}-base
DOCKER_MODULE_NGINX_BUILD_IMAGE = $(DOCKER_REPO)/${MODULE_NAME}-ngx-build
DOCKER_MODULE_NGINX_BASE_IMAGE= $(DOCKER_REPO)/${MODULE_NAME}-ngx-base
RUST_TOOL_IMAGE = $(DOCKER_REPO)/ngx-rust-tool:${RUST_COMPILER_TAG}
DOCKER_NGIX_IMAGE = $(DOCKER_REPO)/nginx-dev:${NGINX_VER}
DOCKER_MIXER_IMAGE = $(DOCKER_REPO)/ngix-mixer:1.0
MODULE_SO_DIR=nginx/nginx-linux/objs
MODULE_SO_BIN=${MODULE_SO_DIR}/${MODULE_NAME}.so
NGINX_BIN=${MODULE_SO_DIR}/nginx
MODULE_SO_HOST=module/release/${MODULE_NAME}.so
NGINX_SO_HOST=config


DOCKER_BUILD_TOOL=docker run -it -e CARGO_HOME=/src/.linux-cargo --rm -v ${HOST_PROJ_DIR}:/src -w /src ${RUST_TOOL_IMAGE}
DOCKER_NGINX_NAME=nginx-test
DOCKER_NGINX_EXEC=docker exec -it ${DOCKER_NGINX_NAME}
DOCKER_NGINX_EXECD=docker exec -d ${DOCKER_NGINX_NAME}
DOCKER_NGINX_DAEMON=docker run -d -p 8000:8000  --privileged --name  ${DOCKER_NGINX_NAME} \
    --sysctl net.ipv4.ip_nonlocal_bind=1 \
    --sysctl net.ipv4.ip_forward=1 \
	-v ${MODULE_DIR}/module/release:/etc/nginx/modules \
	-v ${MODULE_DIR}:/src  -w /src   ${DOCKER_NGIX_IMAGE}


setup-nginx:
	mkdir -p nginx


nginx-source:	setup-nginx
	rm -rf nginx/${NGINX_SRC}
	wget http://nginx.org/download/nginx-${NGINX_VER}.tar.gz
	tar zxf nginx-${NGINX_VER}.tar.gz
	mv nginx-${NGINX_VER} ${NGINX_SRC}
	mv ${NGINX_SRC} nginx
	rm nginx-${NGINX_VER}.tar.gz*

nginx-configure:
	cd nginx/${NGINX_SRC}; \
    ./configure --add-dynamic-module=../../module $(NGX_OPT)


nginx-setup:	nginx-source nginx-configure


nginx-module:
	cd nginx/${NGINX_SRC}; \
	make modules; 

nginx-module-release:	nginx-module
	cd nginx/${NGINX_SRC}; \
	strip objs/*.so






copy-module:
	docker rm -v ngx-copy || true
	docker create --name ngx-copy ${DOCKER_MODULE_IMAGE}:${TAG}
	docker cp ngx-copy:/etc/nginx/modules/${MODULE_NAME}.so ${MODULE_SO_HOST}
	docker rm -v ngx-copy


# open bash tool on docker build
docker-build-bash:
	echo "path ${HOST_PROJ_DIR}"
	${DOCKER_BUILD_TOOL} /bin/bash

docker-build-nginx-setup:
	${DOCKER_BUILD_TOOL} make nginx-setup


docker-build-module:
	${DOCKER_BUILD_TOOL} make nginx-module


watch-mixer:
	 kubectl logs -f $(kubectl get pod -l istio=mixer -n istio-system -o jsonpath='{.items[0].metadata.name}')  -n istio-system -c mixer	


# setup nginx container for testing
# copies the configuration and modules
# start test services
test-nginx-setup:
	test/deploy.sh


# run integrated test
test-intg:
	cargo +stable test --color=always intg -- --nocapture


test-unit:
	cargo test --lib


# remove nginx container
test-nginx-clean:
	docker rm -f  ${DOCKER_NGINX_NAME} || true


test-nginx-only: test-nginx-clean
	$(DOCKER_NGINX_DAEMON)
	$(DOCKER_NGINX_EXECD) make test-nginx-setup > make.out
	sleep 1

test-docker-only:
	docker rm -f nginx || true
	docker run --name nginx -d ${DOCKER_MODULE_IMAGE}:${TAG} 

# build image for testing
test-build-image:	docker-build-module
	docker build -f $(DOCKER_BUILD)/Dockerfile.module --build-arg VERSION=${NGINX_VER} -t ${DOCKER_MODULE_IMAGE}:${GIT_COMMIT} .
	docker tag ${DOCKER_MODULE_IMAGE}:${GIT_COMMIT} ${DOCKER_MODULE_IMAGE}:${TAG}
		
# remove test deployment image
test-k8-nginx-clean:
	kubectl delete deployment nginx-test || true

# deploy test image in k8
test-k8-deploy:	test-build-image
	./scripts/k8-test.sh nginx-test ${DOCKER_NGINX_NAME} ${DOCKER_MODULE_IMAGE} ${TAG}

# show test logs
test-nginx-log:
	docker logs -f nginx-test


test-show-k8-logs:
	kubectl logs $(kubectl get pod -l app=nginmesh -o jsonpath='{.items[0].metadata.name}')


kafka-add-test-topic:
	kubectl -n kafka exec testclient -- /usr/bin/kafka-topics --zookeeper my-kafka-zookeeper:2181 --topic test --create --partitions 1 --replication-factor 1	

# display message from beggining on test channel
test-kafkalist-message:
	kubectl run temp-kafka --image solsson/kafka --rm -ti --command -- bash \
	bin/kafka-console-consumer.sh --bootstrap-server broker.kafka:9092 --topic nginx --from-beginning	

test-nginx-full:	build-module test-nginx-only

# test report deployed locally
test-http-report:
	curl localhost:8000/report

# invoke wrecker
test-wreck-medium:
	 wrk -c10 -d1s -t5 http://localhost:8000/report	

