
ARTIFACT_URL=https://github.com/semsphy/khmercut-rs/releases/download/0.2.0/khmercut-Linux-aarch64-musl.tar.gz
LAYER_DIR=layer
ARTIFACT_FILE=khmercut.tar.gz

all: deploy

# Target to download the artifact
download:
	@echo "Downloading artifact from $(ARTIFACT_URL)..."
	curl -L $(ARTIFACT_URL) -o $(ARTIFACT_FILE)

# Target to extract the artifact
extract: download
	@echo "Extracting artifact..."
	mkdir -p $(LAYER_DIR)
	tar -xzf $(ARTIFACT_FILE) -C $(LAYER_DIR)
	@echo "Artifact downloaded and extracted to $(LAYER_DIR)"



deploy: extract
	@echo "Deploying the service..."
	serverless deploy --verbose
	rm $(ARTIFACT_FILE)

# Target to clean up
clean:
	@echo "Cleaning up..."
	rm -rf $(LAYER_DIR)
	rm -f $(ARTIFACT_FILE)

.PHONY: all download extract deploy clean
