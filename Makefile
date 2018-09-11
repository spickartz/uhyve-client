ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif

client-test: client-test.o target/debug/liblibuhyve.a
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/liblibuhyve.a: src/libuhyve.rs Cargo.toml
	cargo build

client-test.o: client-test.c
	$(CC) -o $@ -c $<

clean:
	rm client-test client-test.o

