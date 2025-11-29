CC = gcc
CFLAGS = -Wall -Wextra -g
AR = ar
ARFLAGS = rcs

# Library name
LIB_NAME = libcross.a

# Rust library
RUST_LIB = target/release/librust_cross.a

# Source files
LIB_SOURCES = library_base.c
LIB_OBJECTS = $(LIB_SOURCES:.c=.o)

# Main executable
MAIN = main
MAIN_SOURCES = main.c
MAIN_OBJECTS = $(MAIN_SOURCES:.c=.o)

.PHONY: all clean rust

all: $(LIB_NAME) $(RUST_LIB) $(MAIN)

$(LIB_NAME): $(LIB_OBJECTS)
	$(AR) $(ARFLAGS) $@ $^

$(RUST_LIB): rust

rust:
	cargo build --release

$(MAIN): $(MAIN_OBJECTS) $(LIB_NAME) $(RUST_LIB)
	$(CC) $(CFLAGS) $(MAIN_OBJECTS) -L. -lcross -Ltarget/release -lrust_cross -o $@

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(LIB_OBJECTS) $(MAIN_OBJECTS) $(LIB_NAME) $(MAIN)
	cargo clean
