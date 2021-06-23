ifeq ($(OS),Windows_NT)
	CC=gcc
	RM=del /q
	LIB_NAME=zt_desktop_tray.lib
	TRAY_CFLAGS := -O -DTRAY_WINAPI=1 -std=c99
	TRAY_LDFLAGS :=
else ifeq ($(shell uname -s),Linux)
	RM=rm -f
	LIB_NAME=libzt_desktop_tray.a
	TRAY_CFLAGS := -O -DTRAY_APPINDICATOR=1 $(shell pkg-config --cflags appindicator3-0.1) -std=c99
	TRAY_LDFLAGS := $(shell pkg-config --libs appindicator3-0.1)
else ifeq ($(shell uname -s),Darwin)
	RM=rm -f
	LIB_NAME=libzt_desktop_tray.a
	TRAY_CFLAGS := -Os -arch x86_64 -arch arm64 -DTRAY_APPKIT=1 -std=c99 -mmacosx-version-min=10.13
	TRAY_LDFLAGS := -framework Cocoa
endif

CFLAGS := $(TRAY_CFLAGS)
LDFLAGS := $(TRAY_LDFLAGS)

all: zt_lib

zt_lib: zt_desktop_tray.o
	ar rcs ${LIB_NAME} zt_desktop_tray.o

example: example.o
	$(CC) $^ $(LDFLAGS) -o $@

example.o: example.c tray.h

clean:
	-$(RM) *.o *.a example *.exe *.lib
