CC = g++
CFLAGS = -lm -Wall -Wextra -Iraylib/src -Iraylib-cpp/include -Iraygui/src -Lraylib/src/ -lraylib
CFLAGS_DEBUG = -DDEBUG=1 -g -O0 -w
CFLAGS_RELEASE = -s -DDEBUG=0 -O3
MINGW_RESOURCE_COMMAND = x86_64-w64-mingw32-windres winresource/resource.rc -o winresource/resource.o
RESOURCE_COMMAND = .\\winresource\\windres.exe .\\winresource\\resource.rc -o .\\winresource\\resource.o
MKDIR_P = mkdir -p "
RM_RF = rm -rf "

ifeq ($(OS), Windows_NT)
	CFLAGS += -lgdi32 -lwinmm -lopengl32
	CFLAGS_RELEASE += --static -mwindows
	MKDIR_P = cmd.exe /c "md
	RM_RF = cmd.exe "/c DEL /f /s /q
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		CFLAGS_RELEASE += -Wl,--strip-debug --static
	endif
	ifeq ($(UNAME_S),Darwin)
		CFLAGS += -framework CoreVideo -framework IOKit -framework Cocoa -framework GLUT -framework OpenGL
	endif
endif


# Targets
BIN = ./bin/KGames
MAIN = ./src/main.cc

# Default target
all: debug

# Build modes
release: CFLAGS += $(CFLAGS_RELEASE)
release: build_resource build

debug: CFLAGS += $(CFLAGS_DEBUG)
debug: build

mingw: CC = $(MINGWCC)
mingw: RESOURCE_COMMAND = $(MINGW_RESOURCE_COMMAND)
mingw: CFLAGS += $(CFLAGS_WIN)
mingw: build

# Compile and link
build:
	-$(MKDIR_P) bin"
	$(CC) $(MAIN) -o $(BIN) $(CFLAGS)

# Run target
run:
	$(BIN)

# Clean target
clean:
	$(RM_RF) bin/MultiSim winresource/resource.o"
	$(RM_RF) bin"

# Usage
.PHONY: all release debug mingw build build_resource run clean
