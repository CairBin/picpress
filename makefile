ifeq ($(OS),Windows_NT)
	TARGET_OS := windows
	APPDATA ?= $(USERPROFILE)/AppData/Local
	PREFIX ?= $(APPDATA)/picpress
	BIN_DIR := $(subst /,\,$(PREFIX)/bin)
	EXECUTABLE := picpress.exe
	RM := del /Q /F
	MKDIR := mkdir
	RMDIR := rmdir /S /Q
	SPACE := $(empty) $(empty)
	BIN_DIR_SAFE := $(subst $(SPACE),\ ,$(BIN_DIR))
	INSTALL_DIRS := $(BIN_DIR)
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		TARGET_OS := linux
		PREFIX ?= /usr/local
	else ifeq ($(UNAME_S),Darwin)
		TARGET_OS := macos
		PREFIX ?= /usr/local
	else
	$(error "Unsupported OS: $(UNAME_S)")
	endif
	BIN_DIR := $(PREFIX)/bin
	EXECUTABLE := picpress
	RM := rm -f
	MKDIR := mkdir -p
	RMDIR := rm -rf
	INSTALL_DIRS := $(BIN_DIR)
endif


CARGO := cargo
BUILD_MODE ?= release
BUILD_FLAGS := $(if $(filter $(BUILD_MODE),release),--release,)
BUILD_DIR := target/$(BUILD_MODE)


.PHONY: all build install uninstall clean

all: build

build:
	$(CARGO) build $(BUILD_FLAGS)

install:
ifeq ($(TARGET_OS),windows)
	powershell -Command "New-Item -ItemType Directory -Force -Path '$(BIN_DIR)'"
	powershell -Command "Copy-Item '$(BUILD_DIR)/$(EXECUTABLE)' '$(BIN_DIR)/$(EXECUTABLE)' -Force"
else
	install $(BUILD_DIR)/$(EXECUTABLE) $(BIN_DIR)/$(EXECUTABLE)
endif

$(INSTALL_DIRS):
ifeq ($(TARGET_OS),windows)
	powershell -Command "New-Item -ItemType Directory -Force -Path '$(BIN_DIR)'"
else
	$(MKDIR) -p $@
endif

uninstall:
ifeq ($(TARGET_OS),windows)
	powershell -Command "Remove-Item '$(BIN_DIR)/$(EXECUTABLE)' -Force -ErrorAction SilentlyContinue"
else
	$(RM) $(BIN_DIR)/$(EXECUTABLE)
endif

clean:
	$(CARGO) clean