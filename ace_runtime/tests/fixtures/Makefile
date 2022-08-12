FIXTURES =
TARGET = wasm32-unknown-unknown

build: $(FIXTURES)

.PHONY: $(FIXTURES)

$(FIXTURES):
	cd $@ && cargo build --release --target $(TARGET)
	cp -f $@/target/$(TARGET)/release/$@.wasm $@/$@.wasm
