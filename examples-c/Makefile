CC = gcc
CFLAGS = -I. -Wall
LDFLAGS = -L../lib -lrcl -lm -Wl,-rpath=../lib
TARGET = example
SRC = example.c

all: $(TARGET)

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(TARGET) $(LDFLAGS)

run: $(TARGET)
	LD_LIBRARY_PATH=../lib ./$(TARGET)

clean:
	rm -f $(TARGET)
