CC = gcc
CFLAGS = -Wall -fPIC
LDFLAGS = -shared

# Name of the output shared library
OUTPUT = libellie_native_bridge.so

# List of source files
SOURCES = lib.c

# Compile the shared library
$(OUTPUT): $(SOURCES)
	$(CC) $(CFLAGS) $(SOURCES) $(LDFLAGS) -o $(OUTPUT)

# Clean the generated files
clean:
	rm -f $(OUTPUT)
