EXAMPLE_NAME = example
SRC = example.rs
LIB_DIR = ../lib
OUT = $(EXAMPLE_NAME)

.PHONY: all run clean

all:
	rustc $(SRC) -o $(OUT) \
		-L $(LIB_DIR) -l rcl \
		-C link-args="-Wl,-rpath,$(LIB_DIR)"

run: all
	./$(OUT)

clean:
	rm -f $(OUT)
