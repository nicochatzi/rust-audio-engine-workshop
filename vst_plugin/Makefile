PROJECT_NAME=AUDIO_PLUGIN_EXAMPLE

#
# Assuming this will run only on MacOS or Windows
#
ifeq ($(shell uname), Darwin)
CMAKE_GENERATOR=Xcode
OPEN_CMD=open
PROJECT_FILE=$(PROJECT_NAME).xcodeproj
else
CMAKE_GENERATOR="Visual Studio 16 2019"
OPEN_CMD=explorer
PROJECT_FILE=$(PROJECT_NAME).sln
endif

#
# Targets
#
all: clean gen build test

clean:
	rm -rf build
.PHONY: clean

gen:
	cmake -S .. -B build -G $(CMAKE_GENERATOR)
.PHONY: gen

build:
	cmake --build build --parallel 8
.PHONY: build

open:
	$(OPEN_CMD) $(PROJECT_FILE)
.PHONY: open

test:
	cd build && ctest
.PHONY: test