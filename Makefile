RUST_LIB = target/release/librcl.so
LIB_OUT = lib/librcl.so
C_DIR = examples-c

all: $(LIB_OUT)
	$(MAKE) -C $(C_DIR)

$(LIB_OUT): src/lib.rs
	cargo build --release
	mkdir -p lib
	cp $(RUST_LIB) $(LIB_OUT)

run: all
	$(MAKE) -C $(C_DIR) run

clean:
	cargo clean
	$(MAKE) -C $(C_DIR) clean
	rm -f $(LIB_OUT)
