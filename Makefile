install:
	@echo "Installing kit..."
	@cargo build --release
	@cargo install --path .