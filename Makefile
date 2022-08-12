TOOLCHAIN=nightly
CLIENT=ace_game_client
DEMO=ace_demo
SERVER=ace_game_server
BACKEND=ace_backend

# FIXME
wasm-it:
	cargo build --release --target wasm32-unknown-unknown --no-default-features -p $(CLIENT)
	wasm-bindgen --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/app.wasm

run-demo: build-ts
	cargo run --release -p $(DEMO)

run-client: build-ts
	cargo run --release -p $(CLIENT)

run-server: build-ts
	cargo run --release -p $(SERVER)

run-backend: build-ts
	cargo run --release -p $(BACKEND)

test: build-ts build-fixtures
	cargo $@

test-release: build-fixtures
	cargo test --release

build: build-ts
	cargo build

release: build-ts
	cargo build --release

run-dyn: build-ts
	cargo run --features bevy/dynamic

nix-run:
	nix-shell shell.nix --run "make run"

nix-run-dyn:
	nix-shell shell.nix --run "make run-dyn"


nix-release:
	nix-shell shell.nix --run "make release"

bevy-deps:
	sudo apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev clang-9

rust-setup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	rustup default $(TOOLCHAIN)
	cargo install bindgen

wasm-setup:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-bindgen-cli

dev-setup:
	cargo install cargo-watch

wasm/target:
	mkdir -p wasm/target

build-fixtures:
	cd ace_runtime/tests/fixtures && $(MAKE) build

DOCKER_BACKEND_TAG = acelabs/backend
DOCKER_SERVER_TAG = acelabs/backend
DOCKER_CLIENT_TAG = acelabs/backend
DOCKER_PORT = 1337

docker-build:
	docker build . --target backend     -t $(DOCKER_BACKEND_TAG)
	docker build . --target game_server -t $(DOCKER_SERVER_TAG)
	docker build . --target game_client -t $(DOCKER_CLIENT_TAG)

docker-run:
	docker run -ti -p $(DOCKER_PORT):$(DOCKER_PORT) $(DOCKER_TAG)

build-ts:
	cd ace_runtime/src/ts && $(MAKE) build


watch-test:
	cargo watch -s 'make test' \
		-i "**/*.wasm" \
		-i "ace_runtime/tests/fixtures/*/pkg" \
		-i "ace_runtime/tests/fixtures/*/build" \
		-i "**/*.log" \
		--why
