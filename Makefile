C_LIB       = target/release/librcl.so
LIB_OUT     = lib/librcl.so
C_DIR       = examples-c
RS_DIR      = examples-rs

.PHONY: all run clean

all: $(LIB_OUT)
	$(MAKE) -C $(C_DIR)
	$(MAKE) -C $(RS_DIR)

$(LIB_OUT): src/lib.rs
	cargo build --release
	mkdir -p lib
	cp $(C_LIB) $(LIB_OUT)

run: all
	$(MAKE) -C $(C_DIR) run
	$(MAKE) -C $(RS_DIR) run

clean:
	cargo clean
	$(MAKE) -C $(C_DIR) clean
	$(MAKE) -C $(RS_DIR) clean
	rm -f $(LIB_OUT)
