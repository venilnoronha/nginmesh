MODULES_DIR=$(BUILD_DIR)/modules
LIBS_DIR=$(BUILD_DIR)/libs
BUILDER_IMAGE=tracing_builder:0.1

modules-dirs:
	mkdir -p $(MODULES_DIR)
	mkdir -p $(LIBS_DIR)

container-tracing-modules-builder:
	make BUILDER_IMAGE=$(BUILDER_IMAGE) -C docker-tracing

modules:
	docker run --rm -v $(shell pwd)/build:/build $(BUILDER_IMAGE)

clean-modules:
	rm -rf $(MODULES_DIR)
	rm -rf $(LIBS_DIR) 